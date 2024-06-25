// ai lab - GUI for annotating, training, and evaluating AI models, simplifying workflows
// Copyright (C) 2024 - Felix Drees - GNU General Public License v3.0

use gtk::prelude::*;
use gtk::{Button, Dialog, Entry, Label, ResponseType};

/* #[macro_use]
mod helper; */
use crate::debug_println;

use crate::helper::{
    generate_config, load_config, save_config, show_error_message, update_dotfile,
};

use std::cell::RefCell;
use std::rc::Rc;

///
/// Workspace UI
///
/// TODO(felix): add documentation
///
pub fn projects_ui() -> gtk::Box {
    let workspace_main_container = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .margin_top(15)
        .margin_bottom(24)
        .margin_start(50)
        .margin_end(50)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .spacing(35)
        .build();

    let separating_or_label = gtk::Label::new(Some("or"));
    separating_or_label.add_css_class("title-3");

    workspace_main_container.append(&select_project_ui());
    workspace_main_container.append(&separating_or_label);
    workspace_main_container.append(&create_new_project_ui());

    workspace_main_container
}

fn select_project_ui() -> gtk::Box {
    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(24)
        // .valign(gtk::Align::Center)
        .build();

    // title
    // ---------------------------------------------------------------------------------------------
    let title = Label::builder()
        .label("Load existing project")
        .halign(gtk::Align::Start)
        .build();

    title.add_css_class("title-3");

    // select project button
    // ---------------------------------------------------------------------------------------------
    let select_workspace_btn = Button::builder()
        .label("open project via file explorer")
        .build();

    select_workspace_btn.connect_clicked(move |_| {
        // Create a new file chooser dialog
        let dialog = gtk::FileChooserDialog::builder()
            .title("Select a workspace .toml file")
            .action(gtk::FileChooserAction::Open)
            .build();

        dialog.add_buttons(&[
            ("Cancel", gtk::ResponseType::Cancel),
            ("Select", gtk::ResponseType::Accept),
        ]);

        dialog.connect_response(|dialog, response| {
            if response == gtk::ResponseType::Accept {
                if let Some(folder) = dialog.file() {
                    debug_println!("Selected directory: {}", folder.path().unwrap().display());
                    let config = load_config(&folder.path().unwrap().display().to_string());

                    if let Ok(x) = config {
                        debug_println!("owner of config: {:?}", x.owner);
                    } else {
                        // TODO gtk dialog popup error / info box
                        debug_println!("WTF, give me a correct .toml file!!! pls")
                    }
                }
            }
            dialog.close();
        });

        dialog.show();
    });

    // add tree view for recent projects
    // ---------------------------------------------------------------------------------------------
    let model = gtk::ListStore::new(&[String::static_type()]);

    model.insert_with_values(None, &[(0, &"example.toml".to_value())]);
    model.insert_with_values(None, &[(0, &"test.toml".to_value())]);
    model.insert_with_values(None, &[(0, &"test2.toml".to_value())]);
    model.insert_with_values(None, &[(0, &"fuu.toml".to_value())]);
    model.insert_with_values(None, &[(0, &"bar.toml".to_value())]);
    model.insert_with_values(None, &[(0, &"test5.toml".to_value())]);
    model.insert_with_values(None, &[(0, &"fuu7.toml".to_value())]);
    model.insert_with_values(None, &[(0, &"bar8.toml".to_value())]);

    let view = gtk::TreeView::with_model(&model.clone());

    let read1 = gtk::CellRendererText::new();
    let col1 = gtk::TreeViewColumn::new();

    col1.set_title("recent projects:");
    col1.pack_start(&read1, true);
    col1.add_attribute(&read1, "text", 0);
    view.append_column(&col1);

    let scrolled_window = gtk::ScrolledWindow::builder().height_request(150).build();

    scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scrolled_window.set_child(Some(&view));

    let provider = gtk::CssProvider::new();
    provider.load_from_data(
        "
        treeview {
            border: 1px solid gray;
        }
        ",
    );

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let open_recent_project = Button::with_label("open selected project");

    let view_clone = view.clone();
    open_recent_project.connect_clicked(move |_| {
        let selection = view_clone.selection();
        if let Some((model, iter)) = selection.selected() {
            if let Ok(value) = model.get_value(&iter, 0).get::<String>() {
                debug_println!("[OPEN RECENT PROJECTS] Open selected project: {}", value);
            } else {
                panic!("[ERROR: OPEN RECENT PROJECTS] Failed to get the string value.");
            }
        } else {
            show_error_message(
                None::<&gtk::Window>,
                Some("WARNING"),
                Some("No project selected.\nPlease select one of the above project form the list."),
            );
            debug_println!("[OPEN RECENT PROJECTS] No row selected.");
        }
    });

    // add elements to vbox
    // ---------------------------------------------------------------------------------------------
    vbox.append(&title);
    vbox.append(&select_workspace_btn);
    vbox.append(&scrolled_window);
    vbox.append(&open_recent_project);

    vbox
}

fn create_new_project_ui() -> gtk::Box {
    let main_vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(24)
        .build();

    let title2 = Label::builder()
        .label("Create new projects")
        .halign(gtk::Align::Start)
        .build();

    title2.add_css_class("title-3");
    main_vbox.append(&title2);

    let selection_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(20)
        .build();

    // Drop Down for selecting the problem type
    // -----------------------------------------
    let classification_tgl = gtk::ToggleButton::with_label("Classification (Predicting Data)");
    let clustering_tgl = gtk::ToggleButton::with_label("Clustering (Grouping)");
    classification_tgl.set_group(Some(&clustering_tgl));
    classification_tgl.set_active(true);

    // TODO: write to dotfile ...
    let show_dialog = Rc::new(RefCell::new(true));
    let show_dialog_clone = show_dialog.clone();

    let class_cluster_tgls = gtk::Box::builder()
        .spacing(0)
        .orientation(gtk::Orientation::Horizontal)
        .build();
    class_cluster_tgls.append(&classification_tgl);
    class_cluster_tgls.append(&clustering_tgl);

    main_vbox.append(&class_cluster_tgls);

    let data_types = vec![
        "images",
        /*"DICOM",*/ "sound / speech",
        "sequential sensors", /*, etc. TODO */
    ];

    let expression2 = gtk::PropertyExpression::new(
        gtk::StringObject::static_type(),
        None::<gtk::Expression>,
        "string",
    );

    let data_kind_dd = gtk::DropDown::builder()
        .model(&gtk::StringList::new(data_types.as_slice()))
        .expression(expression2)
        .build();

    let temp = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .build();

    temp.append(&gtk::Label::new(Some("Data type: ")));
    temp.append(&data_kind_dd);
    selection_box.append(&temp);

    let _select_and_add_class_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(40)
        .build();

    main_vbox.append(&selection_box);

    // --- showing a list of all selected classes --------------------------------------------------
    let model = gtk::ListStore::new(&[String::static_type()]);

    model.insert_with_values(None, &[(0, &"default / background".to_value())]);
    model.insert_with_values(None, &[(0, &"dog".to_value())]);
    model.insert_with_values(None, &[(0, &"cat".to_value())]);

    let view = gtk::TreeView::with_model(&model.clone());

    let read1 = gtk::CellRendererText::new();
    let col1 = gtk::TreeViewColumn::new();

    col1.set_title("Labels / Classes");
    col1.pack_start(&read1, true);
    col1.add_attribute(&read1, "text", 0);
    view.append_column(&col1);

    let scrolled_window = gtk::ScrolledWindow::builder().height_request(150).build();

    scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scrolled_window.set_child(Some(&view));

    let provider = gtk::CssProvider::new();
    provider.load_from_data(
        "
        treeview {
            border: 1px solid gray;
        }
        ",
    );

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let add_class_btn = Button::with_label("Add class to predict");
    let del_selected_row = Button::with_label("Delete Selected Row");
    // let model_clone = model.clone();

    let view_clone = view.clone();
    del_selected_row.connect_clicked(
        gtk::glib::clone!(@strong model =>
        move |_| {
        let selection = view_clone.selection();
        if let Some((tree_model, iter)) = selection.selected() {
            if let Ok(value) = tree_model.get_value(&iter, 0).get::<String>() {
                if value == "default / background" {
                    debug_println!("[WARNING: DEL SELECTED CLASS] no you dont!!! why would anyone want to delete the background label?");
                } else {
                    debug_println!(
                        "[DEL SELECTED CLASS] the following label/class will be deleted: {}",
                        value
                    );
                    model.remove(&iter);
                }
            } else {
                panic!("[ERROR: DEL SELECTED CLASS] Failed to get the string value.");
            }
        } else {
            debug_println!("[DEL SELECTED CLASS] failed to del a class, since no class was selected!");
        }
    }));

    let hbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(30)
        .build();

    hbox.append(&add_class_btn);
    hbox.append(&del_selected_row);

    let v_box_labels_with_add_del_btn = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(5)
        .build();

    v_box_labels_with_add_del_btn.append(&scrolled_window);
    v_box_labels_with_add_del_btn.append(&hbox);
    main_vbox.append(&v_box_labels_with_add_del_btn);

    let v_box_labels_with_add_del_btn_rev = Rc::new(RefCell::new(v_box_labels_with_add_del_btn));

    {
        let v_box_clone = Rc::clone(&v_box_labels_with_add_del_btn_rev);

        classification_tgl.connect_toggled(move |_button| {
            // if button.is_active() {
            // v_box_labels_with_add_del_btn.set_visible(true);
            v_box_clone.borrow_mut().set_visible(true);
            //}
        });
    }

    {
        let v_box_clone = Rc::clone(&v_box_labels_with_add_del_btn_rev);

        clustering_tgl.connect_toggled(move |_button: &gtk::ToggleButton| {
            /*if button.is_active()*/
            // {
            // v_box_labels_with_add_del_btn.set_visible(false);
            v_box_clone.borrow_mut().set_visible(false);

            if *show_dialog_clone.borrow()
            /* && false */
            {
                let dialog = gtk::MessageDialog::new(
                    None::<&gtk::Window>,
                    gtk::DialogFlags::MODAL,
                    gtk::MessageType::Info,
                    gtk::ButtonsType::Ok,
                    "For clustering problems, the labels/classes will be ignored.",
                );

                dialog.set_title(Option::from("Note"));

                let check_button = gtk::CheckButton::with_label("Don't show again");
                check_button.show();
                dialog.content_area().append(&check_button);

                let show_dialog_inner_clone: Rc<RefCell<bool>> = show_dialog_clone.clone();

                dialog.connect_response(move |dialog, response| {
                    if response == gtk::ResponseType::Ok && check_button.is_active() {
                        // User doesn't want to see the info again ...
                        *show_dialog_inner_clone.borrow_mut() = false;
                    }
                    dialog.destroy();
                });

                dialog.show();
            } //}
        });
    }

    // ------------------------------------------------------------------------------------------

    add_class_btn.connect_clicked(move |_| {
            // gtk::glib::clone!(@strong workspace_main_container => move |_| {
            if !classification_tgl.is_active() {
                show_error_message(
                    None::<&gtk::Widget>,
                    Option::from("WORKSPACE ERROR"),
                    Option::from(
                        "Unable to add label/class, since classification is NOT selected.\n\
                    The problem has to be classification, otherwise classes/labels will be ignored.",
                    ),
                );
            } else {
                // --- aks the user for a name and color for the new class / label ---
                let dialog = Dialog::new();
                dialog.set_title(Option::from("Enter Label Class Name and Select Color"));
                dialog.set_default_size(400, 200);

                let content_area = dialog.content_area();
                let vbox = gtk::Box::new(gtk::Orientation::Vertical, 15);

                let name_entry = Entry::new();
                name_entry.set_placeholder_text(Some("Enter label class name"));
                let color_button = gtk::ColorButton::new();

                vbox.append(&Label::new(Some("Label Class Name:")));
                vbox.append(&name_entry);
                vbox.append(&Label::new(Some("Select Color:")));
                vbox.append(&color_button);

                content_area.append(&vbox);

                dialog.add_button("Cancel", ResponseType::Cancel);
                dialog.add_button("OK", ResponseType::Ok);

                // --- implementing response for adding class / label ---
                dialog.connect_response(
                    gtk::glib::clone!(@strong model =>
                    move |dialog, response| {
                    if response == ResponseType::Ok {
                        let name = name_entry.text().to_string();
                        let color = color_button.rgba();

                        debug_println!("Label Class Name: {}", name);
                        debug_println!("Selected Color: rgb({},{},{})", color.red(), color.green(), color.blue());

                        // TODO: check if name is already in the list
                        if !name.is_empty() {
                            model.insert_with_values(None, &[(0, &name)]);
                        }
                    }
                    dialog.close();
                }));

                dialog.show();
            }
        });

    /*classification_tgl.connect_clicked(move |classification_tgl| {
        add_class_btn.set_visible(classification_tgl.is_active())
    });*/

    let save_config_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(5)
        .build();

    let config_filename_entry = Entry::builder().placeholder_text("example.toml").build();

    let save_btn = Button::with_label("save");

    save_config_box.append(&Label::new(Some("Save configuration:")));
    save_config_box.append(&config_filename_entry);
    save_config_box.append(&save_btn);
    main_vbox.append(&save_config_box);

    save_btn.connect_clicked(move |_| {
        // gtk::glib::clone!(@strong workspace_main_container => move |_| {
        let conf_name: Option<&str> = Option::from("name");
        let conf_dob: Option<&str> = Option::from("01.01.2024");
        let conf_title: Option<&str> = Option::from("ai lab config title");
        let workspace_configs = generate_config(conf_name, conf_dob, conf_title);
        let config_file_name = config_filename_entry.text().to_string();

        // if the filename is not empty and ends with .toml
        if config_file_name.is_empty() || !config_file_name.ends_with(".toml") {
            debug_println!(
                "[WARNING] configs not saved - no filename given: {}",
                config_file_name
            );
            show_error_message(
                None::<&gtk::Widget>,
                Option::from("WORKSPACE ERROR"),
                Option::from(
                    "\nUnable to save configs.\n\n  \
                        No filename given or\nfilename dose not end with .toml",
                ),
            );
        } else {
            // update dotfile list of all project config files
            update_dotfile(&config_file_name, None).unwrap();

            // save generated config to .toml file
            save_config(&config_file_name, &workspace_configs).unwrap();
            debug_println!("[INFO] saved config to file: {}", config_file_name);
        }
    });
    // main_vbox.set_hexpand(true);
    // main_vbox.set_vexpand(true);

    main_vbox
}
