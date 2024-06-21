// ai lab - GUI for annotating, training, and evaluating AI models, simplifying workflows
// Copyright (C) 2024 - Felix Drees - GNU General Public License v3.0

use gtk::prelude::*;

use gtk::{ApplicationWindow, Box as GtkBox, CellRendererText, MessageDialog, MessageType};
use gtk::{Button, ColorButton, Dialog, DropDown, Entry, Label, Orientation, ResponseType};

use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs;

// --- helper funcs and structs for workspace ui --------------------------------------------------

/// some example struct for the config
#[derive(Serialize, Deserialize, Debug)]
struct Config {
    title: String,
    owner: Owner,
}

/// even more example structs for the config
#[derive(Serialize, Deserialize, Debug)]
struct Owner {
    name: String,
    dob: String,
}

/*
fn load_config(file_path: &str) -> Result<Config, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

fn modify_config(config: &mut Config) {
    config.title = "New Title".to_string();
    config.owner.name = "New Owner".to_string();
}
*/

/// # save workspace configs to .toml file
///
/// where:
/// - `file_path` is the name of the .toml file to which the config will be written
/// - `config` is the given config as a Config struct
///
/// returns:
///     Result
fn save_config(file_path: &str, config: &Config) -> Result<(), Box<dyn Error>> {
    let toml_string = toml::to_string(config)?;
    fs::write(file_path, toml_string)?;
    Ok(())
}

/// # generate config
///
/// generates config with default values, where all params are optional
///
/// params:
///     - `name` is the name of the config
///     - `dob` is the date of birth
///     - `title` is the title of the config
///
/// returns:
///     Config struct
fn generate_config(name: Option<&str>, dob: Option<&str>, title: Option<&str>) -> Config {
    let owner = Owner {
        name: name.unwrap_or("Default Name").to_string(),
        dob: dob.unwrap_or("2000-01-01").to_string(),
    };

    Config {
        title: title.unwrap_or("Default Title").to_string(),
        owner, // owner: owner,
    }
}

/// Retrieves the top-level `ApplicationWindow` for a given widget.
///
/// This function traverses the widget hierarchy to find the top-level
/// `ApplicationWindow` associated with the provided widget. This is useful
/// for setting the transient parent of dialogs, ensuring they are properly
/// associated with their parent window.
///
/// # Arguments
///
/// * `widget` - A reference to any widget implementing the `IsA<Widget>` trait.
///
/// # Returns
///
/// An `Option<ApplicationWindow>` which is `Some` if the top-level window
/// is found, or `None` if no `ApplicationWindow` is found in the hierarchy.
///
/// # Example
///
/// ```
/// let toplevel_window = get_toplevel_window(&my_widget);
/// if let Some(window) = toplevel_window {
///     // Use the window as the transient parent for a dialog
/// } else {
///     println!("Top-level window not found.");
/// }
/// ```
///
/// # Panics
///
/// This function will panic if the top-level widget is not of type `ApplicationWindow`
/// and cannot be downcasted.
fn get_toplevel_window(_widget: &impl IsA<gtk::Widget>) -> Option<ApplicationWindow> {
    // FIXME(all): .get_toplevel dose not exist?!?
    /*
    let toplevel = widget.get_toplevel();
    if toplevel.is::<ApplicationWindow>() {
        Some(toplevel.downcast::<ApplicationWindow>().unwrap())
    } else {
        None
    }
    */
    None
}

/// Displays an error message dialog with an optional custom title and message.
///
/// This function creates a `MessageDialog` of type `Error` and displays it.
/// The dialog is set to be transient for the provided parent widget's top-level
/// `ApplicationWindow`, ensuring it is properly associated with the parent
/// and managed accordingly by the window manager. The dialog is closed when
/// the user clicks the "OK" button.
///
/// # Arguments
///
/// * `parent` - A reference to any widget implementing the `IsA<Widget>` trait,
///              used to find the top-level `ApplicationWindow` to set as the parent.
/// * `title` - An optional custom title for the dialog. If `None`, the default title "Error" is used.
/// * `message` - An optional custom message for the dialog. If `None`, the default message "An error has occurred!" is used.
///
/// # Example
///
/// ```
/// use gtk::prelude::*;
/// use gtk::{Application, ApplicationWindow, Box as GtkBox, Button};
///
/// fn main() {
///     let application = Application::new(
///         Some("com.example.gtk-error-popup"),
///         Default::default(),
///     ).expect("Initialization failed...");
///
///     application.connect_activate(|app| {
///         let window = ApplicationWindow::new(app);
///         window.set_title("Error Popup Example");
///         window.set_default_size(300, 100);
///
///         let vbox = GtkBox::new(gtk::Orientation::Vertical, 0);
///
///         let button = Button::with_label("Show Error");
///         button.connect_clicked(clone!(@strong vbox => move |_| {
///             show_error_message(&vbox, Some("Custom Error Title"), Some("A custom error message has occurred!"));
///         }));
///
///         vbox.add(&button);
///         window.add(&vbox);
///         window.show_all();
///     });
///
///     application.run(&[]);
/// }
///
/// fn show_error_message(parent: &impl IsA<Widget>, title: Option<&str>, message: Option<&str>) {
///     let dialog_title = title.unwrap_or("Error");
///     let dialog_message = message.unwrap_or("An error has occurred!");
///
///     if let Some(toplevel) = get_toplevel_window(parent) {
///         let dialog = gtk::MessageDialog::new(
///             Some(&toplevel),
///             gtk::DialogFlags::empty(),
///             gtk::MessageType::Error,
///             gtk::ButtonsType::Ok,
///             dialog_message,
///         );
///
///         dialog.set_title(dialog_title);
///
///         dialog.connect_response(|dialog, _| {
///             dialog.close();
///         });
///
///         dialog.show();
///     } else {
///         println!("Failed to find the top-level window.");
///     }
/// }
/// ```
///
/// # Panics
///
/// This function does not panic. If the top-level `ApplicationWindow` cannot be found,
/// the function will simply print an error message to the console.
fn show_error_message(parent: &impl IsA<gtk::Widget>, title: Option<&str>, message: Option<&str>) {
    let dialog_title = title.unwrap_or("Error");
    let dialog_message = message.unwrap_or("An error has occurred!");

    // note: if the toplevel is not, it's not a critical error but would be nicer if its some
    let toplevel: Option<ApplicationWindow> = get_toplevel_window(parent);

    let dialog = MessageDialog::new(
        toplevel.as_ref(),         // Set the parent window (might be None)
        gtk::DialogFlags::empty(), // No special flags
        MessageType::Error,        // Type of the message
        gtk::ButtonsType::Ok,      // Buttons to display
        dialog_message,            // Message text
    );

    dialog.set_title(Option::from(dialog_title));

    dialog.connect_response(|dialog, _| {
        dialog.close();
    });

    dialog.show();
}

// --- end helper ---------------------------------------------------------------------------------

///
/// Workspace UI
///
/// TODO(felix): add documentation
///
pub fn workspace_ui() -> GtkBox {
    // container for ui elements
    // -------------------------
    let workspace_main_container = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .margin_top(15)
        .margin_bottom(24)
        .margin_start(50)
        .margin_end(50)
        // .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .spacing(35)
        .build();

    let container_left = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(24)
        // .valign(gtk::Align::Center)
        .build();

    // add label to container
    // ----------------------
    let title = Label::builder()
        .label("Load existing workspace")
        .halign(gtk::Align::Start)
        .build();

    title.add_css_class("title-3");
    container_left.append(&title);

    // add button to container
    // -----------------------
    let select_workspace_btn = Button::builder().label("select workspace").build();

    // Connect button click event to show file chooser dialog
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
    container_left.append(&select_workspace_btn);

    workspace_main_container.append(&container_left);
    container_left.set_hexpand(false);
    container_left.set_vexpand(false);
    // workspace_main_container.pack_start(&container_left, false);  //, false, 0);

    // ---------------------------------------------------------------------------------------------
    workspace_main_container.append(&gtk::Label::builder().label("or").build());

    // workspace_main_container.pack_start(&gtk::Label::builder().label("or").build(), false);
    // ---------------------------------------------------------------------------------------------

    let container_right = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(24)
        .build();

    let title2 = Label::builder()
        .label("Create new workspace")
        .halign(gtk::Align::Start)
        .build();

    title2.add_css_class("title-3");
    container_right.append(&title2);

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

    container_right.append(&class_cluster_tgls);

    let data_types = vec![
        "images",
        /*"DICOM",*/ "sound / speech", /*, etc. TODO */
    ];
    let expression2 = gtk::PropertyExpression::new(
        gtk::StringObject::static_type(),
        None::<gtk::Expression>,
        "string",
    );

    let data_kind_dd = DropDown::new(
        Some(&gtk::StringList::new(data_types.as_slice())),
        Some(expression2),
    );
    selection_box.append(&data_kind_dd);

    let _select_and_add_class_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(40)
        .hexpand(true)
        .build();

    container_right.append(&selection_box);

    // --- showing a list of all selected classes --------------------------------------------------

    let model = gtk::ListStore::new(&[String::static_type()]);

    model.insert_with_values(None, &[(0, &"default / background".to_value())]);
    model.insert_with_values(None, &[(0, &"dog".to_value())]);
    model.insert_with_values(None, &[(0, &"cat".to_value())]);
    model.insert_with_values(None, &[(0, &"tree".to_value())]);
    model.insert_with_values(None, &[(0, &"water".to_value())]);
    model.insert_with_values(None, &[(0, &"road".to_value())]);
    model.insert_with_values(None, &[(0, &"etc.".to_value())]);

    let view = gtk::TreeView::with_model(&model);
    let read1 = CellRendererText::new();
    let col1 = gtk::TreeViewColumn::new();

    col1.set_title("Labels / Classes");
    col1.pack_start(&read1, true);
    col1.add_attribute(&read1, "text", 0);
    view.append_column(&col1);

    let scrolled_window = gtk::ScrolledWindow::builder().height_request(150).build();

    scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    scrolled_window.set_child(Some(&view));

    let add_class_btn = Button::with_label("Add class to predict");
    let button = Button::with_label("Delete Selected Row");
    // let model_clone = model.clone();

    button.connect_clicked(move |_| {
        println!("hmmm ok ... {}", view.selection());
        /*let selection = view.selection();
        if let Some((model, iter)) = selection.selected() {
            // model.remove(&iter);
            // model.connect_row_deleted(iter);
            print!("trying to delete a row");
        }*/
    });

    container_right.append(&scrolled_window);

    let hbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(30)
        .build();

    hbox.append(&add_class_btn);
    hbox.append(&button);

    container_right.append(&hbox);

    // ------------------------------------------------------------------------------------------

    add_class_btn.connect_clicked(
        gtk::glib::clone!(@strong workspace_main_container => move |_| {
            if !classification_tgl.is_active() {
                show_error_message(
                    &workspace_main_container,
                    Option::from("WORKSPACE ERROR"),
                    Option::from("\nUnable to add label/class,\nsince clustering is selected."),
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
                let color_button = ColorButton::new();

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
        }),
    );

    /*classification_tgl.connect_clicked(move |classification_tgl| {
        add_class_btn.set_visible(classification_tgl.is_active())
    });*/

    container_right.append(&Label::new(Some("Save config to .toml file:")));

    let save_config_box = gtk::Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(5)
        .build();

    let config_filename_entry = Entry::builder().placeholder_text("example.toml").build();

    let save_btn = Button::with_label("save");

    save_config_box.append(&config_filename_entry);
    save_config_box.append(&save_btn);
    container_right.append(&save_config_box);

    save_btn.connect_clicked(
        gtk::glib::clone!(@strong workspace_main_container => move |_| {
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
                    &workspace_main_container,
                    Option::from("WORKSPACE ERROR"),
                    Option::from("\nUnable to save configs.\n\n  \
                        No filename given or\nfilename dose not end with .toml"),
                );
            } else {
                save_config(&config_file_name, &workspace_configs).unwrap();
                println!("[INFO] saved config to file: {}", config_file_name);
            }
        }),
    );

    workspace_main_container.append(&container_right);
    container_right.set_hexpand(true);
    container_right.set_vexpand(true);
    // workspace_main_container.pack_start(&container_right, true, true, 0);

    workspace_main_container
}
