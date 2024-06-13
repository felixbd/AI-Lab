// ai lab - GUI for annotating, training, and evaluating AI models, simplifying workflows
// Copyright (C) 2023  Felix Drees

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Notebook, Box};

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
    let title = format!("{} - by: Felix D. - v{}", app_name, version);

    let window = ApplicationWindow::builder()
        .title(&title)
        .application(app)
        .build();
        //.default_width(700)
        //.default_height(500)
    window.set_default_size(600, 400);

    let notebook = Notebook::new();
    window.set_child(Some(&notebook));

    // === BEGIN Workspace sub page ================================================================
    // container for ui elements
    // -------------------------
    let container = Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .spacing(24)
        .build();

    // add label to container
    // ----------------------
    let title = Label::builder()
        .label("Example Label")
        .halign(gtk::Align::Start)
        .build();

    title.add_css_class("title-2");
    container.append(&title);

    // add button to container
    // -----------------------
    let button = Button::builder()
        .label("select workspace")
        .build();

    container.append(&button);

    // Connect button click event to show file chooser dialog
    button.connect_clicked(move |_| {
        // Create a new file chooser dialog
        let dialog = gtk::FileChooserDialog::builder()
            .title("Select a Directory")
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

    container.append(&gtk::Label::builder().label("or\ncreate a new workspace (todo)").build());

    notebook.append_page(&container, Some(&Label::new(Some("Workspace"))));
    // === END Workspace sub page ==================================================================

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

    // ---------------------------------------------------------------------------------------------
    
    window.show();
    // window.present();
}
