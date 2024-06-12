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

    let window = ApplicationWindow::builder()
        .title("example title")
        .application(app)
        .child(&button)
        .build();

    window.show();
}
