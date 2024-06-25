// ai lab - GUI for annotating, training, and evaluating AI models, simplifying workflows
// Copyright (C) 2024 - Felix Drees - GNU General Public License v3.0

use gtk::prelude::*;
use gtk::Box as GtkBox;

pub fn annotation_ui() -> GtkBox {
    let main_box = gtk::Box::builder()
        .spacing(1)
        .orientation(gtk::Orientation::Vertical)
        .valign(gtk::Align::Center)
        .build();

    main_box.append(&gtk::Label::new(Some("annotator")));

    main_box
}
