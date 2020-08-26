mod universe;

use crate::universe::Universe;
use crossbeam_channel::*;
use gio::prelude::ApplicationExtManual;
use gio::ApplicationExt;
use gtk::{ContainerExt, GtkWindowExt, WidgetExt};
use std::env;

struct LeftNav {}

pub trait GuiController {
    type Model;
    type View;

    fn view(&self) -> Self::View;
    fn model(&self) -> Self::Model;
}

fn main() {
    let application = gtk::Application::new(
        Some("org.dwbrite.briteplayer"),
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Application::new failed");

    application.connect_activate(move |app| {
        universe::Universe::create(&app);
    });
    application.run(&env::args().collect::<Vec<_>>());
}
