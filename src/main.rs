#[macro_use]
extern crate glib;
extern crate gtk;
extern crate gio;
extern crate gdk;

// To import all needed traits.
use gtk::prelude::*;
use gio::prelude::*;


use gtk::ResponseType;
use std::env::args;
use std::env;
use std::convert::TryFrom;
use std::rc::Rc;


fn build_ui(app: &gtk::Application) {

    let list = gtk::ListBox::new();

    for i in 0..25 {
        let txt = gtk::Label::new(Some(&i.to_string()));

        let box_ = gtk::ListBoxRow::new();
        box_.add(&txt.clone());
        box_.show_all();

        let txt_rc = Rc::new(txt);
        box_.connect_event(move |row, e| {
            if row.clone().has_focus() && e.get_event_type() == gdk::EventType::FocusChange {
                txt_rc.set_markup("o <i>hej</i>");
                println!("cool");
            }
            Inhibit(false)
        });

        list.add(&box_.upcast::<gtk::Widget>());
    }

    let scrollbox = gtk::ScrolledWindow::new::<gtk::Adjustment, gtk::Adjustment>(None, None);
    scrollbox.add(&list);


    let window = gtk::ApplicationWindow::new(app);
    window.set_title("test example");
    window.set_default_size(600, 400);
    window.add(&scrollbox);
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