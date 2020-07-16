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

struct LibraryItem {
    //title: Rc<gtk::Label>,
    title: String,

    // TODO: content
    // TODO: gtk "window"
}

impl LibraryItem {
    fn from_markup(title: &str) -> LibraryItem{
        LibraryItem {
            title: title.to_string(),
        }
    }

    fn as_widget(&self) -> gtk::Widget {
        // TODO: find out if txt is instantiated w/ markup
        let txt = gtk::Label::new(None);
        txt.set_markup(&self.title);
        txt.set_halign(gtk::Align::Start);

        let box_ = gtk::ListBoxRow::new();
        box_.add(&txt.clone());
        box_.show_all();

        let txt_rc = Rc::new(txt);
        box_.connect_event(move |row, e| {
            if row.clone().has_focus() && e.get_event_type() == gdk::EventType::FocusChange {
                //txt_rc.set_markup("o <i>hej</i>");
                println!("cool");
            }
            Inhibit(false)
        });

        box_.upcast::<gtk::Widget>()
    }
}



fn build_library_list() -> gtk::ScrolledWindow {
    let list = gtk::ListBox::new();

    //list.add(&box_.upcast::<gtk::Widget>());
    list.add(&LibraryItem::from_markup("<b>Library</b>").as_widget());
    list.add(&LibraryItem::from_markup("Music").as_widget());
    list.add(&LibraryItem::from_markup("Podcasts").as_widget());
    list.add(&LibraryItem::from_markup("Lorem Ipsum").as_widget());
    list.add(&LibraryItem::from_markup("Dolor").as_widget());


    let scrollbox = gtk::ScrolledWindow::new::<gtk::Adjustment, gtk::Adjustment>(None, None);
    scrollbox.add(&list);
    scrollbox
}

fn build_ui(app: &gtk::Application) {
    let scrollbox = build_library_list();

    // TODO: vbox(?) as parent, main contents on top, trackbar on bottom
    let panes = gtk::Paned::new(gtk::Orientation::Horizontal);
    panes.pack1(&build_library_list(), true, false);
    panes.pack2(&build_library_list(), true, false);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.pack_start(&panes, true, true, 0);
    vbox.pack_end(&gtk::Entry::new(), false, true, 0);


    let window = gtk::ApplicationWindow::new(app);
    window.set_title("test example");
    window.set_default_size(1280, 720);
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