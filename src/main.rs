// ai lab - GUI for annotating, training, and evaluating AI models, simplifying workflows
// Copyright (C) 2023  Felix Drees

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Notebook, Box, DropDown};

fn main() { // -> glib::ExitCode {
    let app = Application::builder()
        .application_id("example.ai-lab")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let app_name = option_env!("CARGO_PKG_NAME").unwrap_or("AI Lab");
    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("0.0.0");
    let title = format!("{} - by Felix D. - v{}", app_name, version);

    let window = ApplicationWindow::builder()
        .title(&title)
        .application(app)
        .default_width(700)
        .default_height(500)
        .build();

    let notebook = Notebook::new();
    window.set_child(Some(&notebook));

    notebook.append_page(&workspace_ui(), Some(&Label::new(Some("Workspace"))));

    let page2_label = Label::new(Some("Annotation"));
    notebook.append_page(&page2_label, Some(&Label::new(Some("Annotation"))));

    let page3_label = Label::new(Some("Preprocessing"));
    notebook.append_page(&page3_label, Some(&Label::new(Some("Preprocessing"))));

    let page4_label = Label::new(Some("Training"));
    notebook.append_page(&page4_label, Some(&Label::new(Some("Training"))));

    let page5_label = Label::new(Some("Postprocessing"));
    notebook.append_page(&page5_label, Some(&Label::new(Some("Postprocessing"))));

    let page6_label = Label::new(Some("Prediction"));
    notebook.append_page(&page6_label, Some(&Label::new(Some("Prediction"))));

    let page7_label = Label::new(Some("Evaluation"));
    notebook.append_page(&page7_label, Some(&Label::new(Some("Evaluation"))));

    window.show();  // window.present();
}


fn workspace_ui() -> gtk::Box {
    // container for ui elements
    // -------------------------
    let container = Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
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
    let button = Button::builder()
        .label("select workspace")
        .build();


    // Connect button click event to show file chooser dialog
    button.connect_clicked(move |_| {
        // Create a new file chooser dialog
        let dialog = gtk::FileChooserDialog::builder()
            .title("Select a workspace .yaml file")
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
    container_left.append(&button);
    container.append(&container_left);

    container.append(&gtk::Label::builder().label("or").build());

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

    let drop_down = DropDown::new(Some(&gtk::StringList::new(problem_types.as_slice())),
                                  Some(expression));
    // drop_down.set_enable_search(true);
    // drop_down.set_search_match_mode(gtk::StringFilterMatchMode::Substring);

    container_right.append(&drop_down);

    // TODO(felix): fi the user selected classification, add a section where the user
    //  can select what classes to classify

    // TODO(felix): add a text filed for a filename to write the workspace config to

    // TODO(felix): add a selection for data directory selection
    //  maybe also add a directory for the labels

    container.append(&container_right);

    container
}