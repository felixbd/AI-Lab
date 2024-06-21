// ai lab - GUI for annotating, training, and evaluating AI models, simplifying workflows
// Copyright (C) 2024 - Felix Drees - GNU General Public License v3.0

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Label, Notebook};

mod workspace;
use workspace::projects_ui;

mod annotation;
mod helper;

use annotation::annotation_ui;

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
    let app_name = "AI Lab"; // option_env!("CARGO_PKG_NAME").unwrap_or("AI Lab");
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

    notebook.append_page(&projects_ui(), Some(&Label::new(Some("Projects"))));
    notebook.append_page(&annotation_ui(), Some(&Label::new(Some("Annotation"))));

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

    notebook.append_page(&license_ui(), Some(&Label::new(Some("LICENCE"))));

    window.show(); // window.present();
}

fn license_ui() -> GtkBox {
    let container = GtkBox::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .spacing(24)
        .build();

    let title = Label::builder()
        .label(
            "AI Lab - GUI for annotating, training, and evaluating AI models\n\
               Copyright (C) 2024 - Felix Drees - GNU General Public License v3.0",
        )
        .halign(gtk::Align::Start)
        .build();

    title.add_css_class("title-2");
    container.append(&title);

    let content = Label::builder()
        .label(
            "This program is free software: you can redistribute it and/or modify\n \
            it under the terms of the GNU General Public License as published by\n \
            the Free Software Foundation, either version 3 of the License, or\n \
            (at your option) any later version.\n \
            \n \
            This program is distributed in the hope that it will be useful,\n \
            but WITHOUT ANY WARRANTY; without even the implied warranty of\n \
            MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\n \
            GNU General Public License for more details.\n \
            \n \
            You should have received a copy of the GNU General Public License\n \
            along with this program.  If not, see <https://www.gnu.org/licenses/>.",
        )
        .build();

    container.append(&content);

    container
}
