// ai lab - GUI for annotating, training, and evaluating AI models, simplifying workflows
// Copyright (C) 2024 - Felix Drees - GNU General Public License v3.0

use gtk::prelude::*;

use gtk::{ApplicationWindow, /* Box as GtkBox, CellRendererText, */ MessageDialog, MessageType,};
// use gtk::{Button, ColorButton, Dialog, DropDown, Entry, Label, Orientation, ResponseType};

/* use gtk::glib::IsA; */
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DotFileConfig {
    pub(crate) projects: Vec<String>,
}

pub(crate) fn update_dotfile(
    new_project_path: &str,
    dotfile_path: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let users_home_dir = format!(
        "{}/.config/ai-lab.toml",
        home_dir().unwrap().to_str().unwrap()
    );
    let some_file_path = dotfile_path.unwrap_or(&users_home_dir);

    let current_config = fs::read_to_string(some_file_path)
        .ok()
        .map(|content| {
            let current_dotfile_config: DotFileConfig = toml::from_str(&content).ok().unwrap();
            let mut dotfile_project_list = current_dotfile_config.projects;
            dotfile_project_list.push(new_project_path.parse().unwrap());
            DotFileConfig {
                projects: dotfile_project_list,
            }
        })
        .unwrap_or(DotFileConfig {
            projects: vec![new_project_path.parse().unwrap()],
        });

    fs::write(
        some_file_path,
        format!(
            "# This is a generated configuration file from AI Lab\n\
             #  See: https://github.com/felixbd/ai-lab/ \n\
             #\n\
             # ai lab - GUI for annotating, training, and evaluating AI models, simplifying workflows\n\
             # Copyright (C) 2024 - Felix Drees - GNU General Public License v3.0\n\
             #\n\n\
             {}",
            toml::to_string(&current_config).unwrap()
        ),
    )?;
    Ok(())
}

/// some example struct for the config
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Config {
    title: String,
    pub(crate) owner: Owner,
}

/// even more example structs for the config
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Owner {
    name: String,
    dob: String,
}

pub(crate) fn load_config(file_path: &str) -> Result<Config, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

pub(crate) fn _modify_config(config: &mut Config) {
    config.title = "New Title".to_string();
    config.owner.name = "New Owner".to_string();
}

/// # save workspace configs to .toml file
///
/// where:
/// - `file_path` is the name of the .toml file to which the config will be written
/// - `config` is the given config as a Config struct
///
/// returns:
///     Result
pub(crate) fn save_config(file_path: &str, config: &Config) -> Result<(), Box<dyn Error>> {
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
pub(crate) fn generate_config(
    name: Option<&str>,
    dob: Option<&str>,
    title: Option<&str>,
) -> Config {
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
/// and cannot be down-casted.
pub(crate) fn get_toplevel_window(
    _widget: Option<&impl IsA<gtk::Widget>>,
) -> Option<ApplicationWindow> {
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
pub(crate) fn show_error_message(
    parent: Option<&impl IsA<gtk::Widget>>,
    title: Option<&str>,
    message: Option<&str>,
) {
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
