// ai lab - GUI for annotating, training, and evaluating AI models, simplifying workflows
// Copyright (C) 2023  Felix Drees - GNU General Public License v3.0

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label, Notebook};

mod workspace;
use workspace::workspace_ui;

/// Sets up and runs the main application.
///
/// This function initializes an application with a specific ID,
/// connects its activation signal to `build_ui`, and runs the application loop.
///
/// # Returns
///
/// Returns a `glib::ExitCode` indicating the exit status of the application.
///
fn main() {
    // -> glib::ExitCode {
    let app = Application::builder()
        .application_id("example.ai-lab")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

/// Builds the user interface for the application.
///
/// This function constructs the main application window with a notebook widget
/// containing several pages for different functionalities related to AI Lab tasks.
///
/// # Arguments
///
/// * `app` - The application context to which the UI is bound.
///
fn build_ui(app: &Application) {
    let app_name = option_env!("CARGO_PKG_NAME").unwrap_or("AI Lab");
    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("0.0.0");
    let title = format!("{} - v{}", app_name, version);

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

    window.show(); // window.present();
}
