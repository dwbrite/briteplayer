extern crate gtk;
extern crate gio;

// To import all needed traits.
use gtk::prelude::*;
use gio::prelude::*;

use std::env;

fn build_ui(app: &gtk::Application) {
    let btn = gtk::Button::with_label("o hej");
    let txt = gtk::Label::new(Some(";)"));
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.pack_start(&btn, true, true, 0);
    vbox.pack_start(&btn, true, true, 0);
    vbox.pack_start(&txt, true, true, 0);
    vbox.pack_start(&btn, true, true, 0);

    let window = gtk::ApplicationWindow::new(app);
    window.set_title("test example");
    window.set_default_size(200, 100);
    window.add(&vbox);
    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("org.dwbrite.briteplayer"),
        gio::ApplicationFlags::FLAGS_NONE
    ).expect("Application::new failed");




    application.connect_activate(|app| {
        build_ui(&app);
    });
    application.run(&env::args().collect::<Vec<_>>());
}