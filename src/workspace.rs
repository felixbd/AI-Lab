// ai lab - GUI for annotating, training, and evaluating AI models, simplifying workflows
// Copyright (C) 2023  Felix Drees - GNU General Public License v3.0

use gtk::prelude::*;
use gtk::{Box, Button, ColorButton, Dialog, DropDown, Entry, Label, Orientation, ResponseType};

use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs;

// --- helper funcs and structs for workspace ui --------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    title: String,
    owner: Owner,
}

#[derive(Serialize, Deserialize, Debug)]
struct Owner {
    name: String,
    dob: String, // Use appropriate type based on your TOML structure
}

fn load_config(file_path: &str) -> Result<Config, Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

fn modify_config(config: &mut Config) {
    config.title = "New Title".to_string();
    config.owner.name = "New Owner".to_string();
}

fn save_config(file_path: &str, config: &Config) -> Result<(), Box<dyn Error>> {
    let toml_string = toml::to_string(config)?;
    fs::write(file_path, toml_string)?;
    Ok(())
}

fn generate_config(name: Option<&str>, dob: Option<&str>, title: Option<&str>) -> Config {
    let owner = Owner {
        name: name.unwrap_or("Default Name").to_string(),
        dob: dob.unwrap_or("2000-01-01").to_string(), // Use a default date
    };

    Config {
        title: title.unwrap_or("Default Title").to_string(),
        owner: owner,
    }
}

// --- end helper ---------------------------------------------------------------------------------

///
/// Workspace UI
///
/// TODO(felix): add documentation
///
pub fn workspace_ui() -> gtk::Box {
    // container for ui elements
    // -------------------------
    let workspace_main_container = Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .margin_top(4)
        .margin_bottom(24)
        .margin_start(4)
        .margin_end(24)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .spacing(24)
        .build();

    let container_left = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(24)
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

    workspace_main_container.append(&gtk::Label::builder().label("or").build());

    let container_right = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(24)
        .build();

    let title2 = Label::builder()
        .label("Create new workspace")
        .halign(gtk::Align::Start)
        .build();

    title2.add_css_class("title-3");
    container_right.append(&title2);

    // Drop Down for selecting the problem type
    // -----------------------------------------
    let problem_types = vec!["Classification (Predicting Data)", "Clustering (Grouping)"];

    let expression = gtk::PropertyExpression::new(
        gtk::StringObject::static_type(),
        None::<gtk::Expression>,
        "string",
    );

    let problem_kind_dd = DropDown::new(
        Some(&gtk::StringList::new(problem_types.as_slice())),
        Some(expression),
    );
    container_right.append(&problem_kind_dd);

    let data_types = vec!["sequential", "images"];
    let expression2 = gtk::PropertyExpression::new(
        gtk::StringObject::static_type(),
        None::<gtk::Expression>,
        "string",
    );

    let data_kind_dd = DropDown::new(
        Some(&gtk::StringList::new(data_types.as_slice())),
        Some(expression2),
    );
    container_right.append(&data_kind_dd);

    let add_class_btn = Button::with_label("Add class to predict");
    // add_class_btn.set_visible(false); // Initially hidden
    container_right.append(&add_class_btn);

    add_class_btn.connect_clicked(move |_| {
        let dialog = Dialog::new();
        dialog.set_title(Option::from("Enter Label Class Name and Select Color"));
        dialog.set_default_size(400, 200);

        let content_area = dialog.content_area();
        let vbox = Box::new(Orientation::Vertical, 15);

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

        dialog.connect_response(move |dialog, response| {
            if response == ResponseType::Ok {
                let name = name_entry.text();
                let color = color_button.rgba();
                println!("Label Class Name: {}", name);
                println!("Selected Color: {:?}", color);
            }
            dialog.close();
        });

        dialog.show();
    });

    problem_kind_dd.connect_notify_local(Some("selected-item"), move |drop_down, _| {
        if let Some(selected_item) = drop_down.selected_item() {
            add_class_btn.set_visible(
                "Classification (Predicting Data)"
                    == selected_item
                        .downcast_ref::<gtk::StringObject>()
                        .unwrap()
                        .string(),
            );
        }
    });

    // TODO(felix): add a selection for data directory selection
    //  maybe also add a directory for the labels

    container_right.append(&Label::new(Some("Save config to .toml file:")));
    let config_filename_entry = Entry::new();
    config_filename_entry.set_placeholder_text(Some("example_workspace.toml"));
    container_right.append(&config_filename_entry);

    let save_btn = Button::with_label("save config");
    container_right.append(&save_btn);

    save_btn.connect_clicked(move |_| {
        println!("[WARNING] saving config not jet implemented");
    });

    workspace_main_container.append(&container_right);

    workspace_main_container
}
