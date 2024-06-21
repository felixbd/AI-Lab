// ai lab - GUI for annotating, training, and evaluating AI models, simplifying workflows
// Copyright (C) 2024 - Felix Drees - GNU General Public License v3.0

use gtk::prelude::*;
use gtk::{Button, Dialog, Entry, Label, Orientation, ResponseType};

use crate::helper::{
    generate_config, /* load_config, modify_config, */
    save_config, /* get_toplevel_window, */ show_error_message,
    /* Owner, Config */
};

// Gtk RecentChooserDialog

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

        // Connect response event to handle user's selection
        dialog.connect_response(|dialog, response| {
            if response == gtk::ResponseType::Accept {
                if let Some(folder) = dialog.file() {
                    println!("Selected directory: {}", folder.path().unwrap().display());
                }
            }
            dialog.close();
        });

        // Show the dialog
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

    let view = gtk::TreeView::with_model(&model);

    let read1 = gtk::CellRendererText::new();
    let col1 = gtk::TreeViewColumn::new();

    col1.set_title("recent projects:");
    col1.pack_start(&read1, true);
    col1.add_attribute(&read1, "text", 0);
    view.append_column(&col1);

    let scrolled_window = gtk::ScrolledWindow::builder().height_request(150).build();

    scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scrolled_window.set_child(Some(&view));

    // Create a CSS provider and load the CSS
    let provider = gtk::CssProvider::new();
    provider.load_from_data(
        "
        treeview {
            border: 1px solid gray;
        }
        ",
    );

    // Add the CSS provider to the default screen
    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let button = Button::with_label("open selected project");
    // let model_clone = model.clone();

    button.connect_clicked(move |_| {
        println!("hmmm ok ... {:?}", view.selection());
        /*let selection = view.selection();
        if let Some((model, iter)) = selection.selected() {
            // model.remove(&iter);
            // model.connect_row_deleted(iter);
            print!("trying to delete a row");
        }*/
    });

    // add elements to vbox
    // ---------------------------------------------------------------------------------------------
    vbox.append(&title);
    vbox.append(&select_workspace_btn);
    vbox.append(&scrolled_window);
    vbox.append(&button);
    // vbox.set_vexpand(false);
    // vbox.set_hexpand(false);

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
        // .hexpand(true)
        .build();

    main_vbox.append(&selection_box);

    // --- showing a list of all selected classes --------------------------------------------------
    let model = gtk::ListStore::new(&[String::static_type()]);

    model.insert_with_values(None, &[(0, &"default / background".to_value())]);
    /*model.insert_with_values(None, &[(0, &"dog".to_value())]);
    model.insert_with_values(None, &[(0, &"cat".to_value())]);
    model.insert_with_values(None, &[(0, &"tree".to_value())]);
    model.insert_with_values(None, &[(0, &"water".to_value())]);
    model.insert_with_values(None, &[(0, &"road".to_value())]);
    model.insert_with_values(None, &[(0, &"etc.".to_value())]);*/

    let view = gtk::TreeView::with_model(&model);

    let read1 = gtk::CellRendererText::new();
    let col1 = gtk::TreeViewColumn::new();

    col1.set_title("Labels / Classes");
    col1.pack_start(&read1, true);
    col1.add_attribute(&read1, "text", 0);
    view.append_column(&col1);

    let scrolled_window = gtk::ScrolledWindow::builder().height_request(150).build();

    scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scrolled_window.set_child(Some(&view));

    // Create a CSS provider and load the CSS
    let provider = gtk::CssProvider::new();
    provider.load_from_data(
        "
        treeview {
            border: 1px solid gray;
        }
        ",
    );

    // Add the CSS provider to the default screen
    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let add_class_btn = Button::with_label("Add class to predict");
    let button = Button::with_label("Delete Selected Row");
    // let model_clone = model.clone();

    button.connect_clicked(move |_| {
        println!("hmmm ok ... {:?}", view.selection());
        /*let selection = view.selection();
        if let Some((model, iter)) = selection.selected() {
            // model.remove(&iter);
            // model.connect_row_deleted(iter);
            print!("trying to delete a row");
        }*/
    });

    main_vbox.append(&scrolled_window);

    let hbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(30)
        .build();

    hbox.append(&add_class_btn);
    hbox.append(&button);

    main_vbox.append(&hbox);

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
            let vbox = gtk::Box::new(Orientation::Vertical, 15);

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
            dialog.connect_response(move |dialog, response| {
                if response == ResponseType::Ok {
                    let name = name_entry.text().to_string();
                    let color = color_button.rgba();

                    /*if !name.is_empty() {
                        x.add_attribute(&gtk::CellRendererText::new(), "fuu", 0);
                    }*/
                    println!("Label Class Name: {}", name);
                    println!("Selected Color: {:?}", color);
                }
                dialog.close();
            });

            dialog.show();
        }
    });

    /*classification_tgl.connect_clicked(move |classification_tgl| {
        add_class_btn.set_visible(classification_tgl.is_active())
    });*/

    main_vbox.append(&Label::new(Some("Save config to .toml file:")));

    let save_config_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(5)
        .build();

    let config_filename_entry = Entry::builder().placeholder_text("example.toml").build();

    let save_btn = Button::with_label("save");

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
            println!(
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
            save_config(&config_file_name, &workspace_configs).unwrap();
            println!("[INFO] saved config to file: {}", config_file_name);
        }
    });
    // main_vbox.set_hexpand(true);
    // main_vbox.set_vexpand(true);

    main_vbox
}
