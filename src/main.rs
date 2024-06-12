// <one line to give the program's name and a brief idea of what it does.>
// Copyright (C) <year>  <name of author>

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn main() {
    let app = Application::builder()
        .application_id("example.ai-lab")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Click Me")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let version = option_env!("CARGO_PKG_VERSION").unwrap_or("0.0.0");
    let title = format!("example title - by: Felix D. - v{}", version);

    let window = ApplicationWindow::builder()
        .title(&title)
        .application(app)
        .child(&button)
        .build();

    window.show();
}
