extern crate glib;
extern crate gtk;
extern crate gio;
extern crate gdk;

// To import all needed traits.
use gtk::prelude::*;
use gio::prelude::*;

use std::rc::Rc;
use std::sync::Mutex;
use std::env;
use gtk::Widget;

mod nav;

fn build_artists_albums_songs() -> gtk::Paned {
    let vpane = gtk::Paned::new(gtk::Orientation::Vertical);
    let hpane = gtk::Paned::new(gtk::Orientation::Horizontal);

    let artists = {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        vbox.pack_start(&gtk::Label::new(Some("<b><i>Artists</i></b>")), false, false, 0);
        vbox.pack_end(&build_library_list(), true, true, 0);
        vbox
    };

    let albums = {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        vbox.pack_start(&gtk::Label::new(Some("<b><i>Albums</i></b>")), false, false, 0);
        vbox.pack_end(&build_library_list(), true, true, 0);
        vbox
    };

    let songs = {
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        vbox.pack_start(&gtk::Label::new(Some("<b><i>Songs</i></b>")), false, false, 0);
        vbox.pack_end(&build_library_list(), true, true, 0);
        vbox
    };

    hpane.add1(&artists);
    hpane.add2(&albums);
    vpane.add1(&hpane);
    vpane.add2(&songs);

    vpane.show_all();

    vpane
}

mod LibraryItem {
    use gtk::{LabelExt, WidgetExt, ContainerExt, Inhibit};
    use gtk::prelude::Cast;

    pub fn from_markup(title: &str) -> gtk::Widget {
        let label = gtk::Label::new(None);
        label.set_markup(&title);
        label.set_halign(gtk::Align::Start);

        let box_ = gtk::ListBoxRow::new();
        box_.add(&label);
        box_.show_all();

        let l_rc = label.clone();
        box_.connect_event(move |row, e| {
            // TODO: change from "focus" to "select"
            if row.clone().has_focus() && e.get_event_type() == gdk::EventType::FocusChange {
                let label = l_rc.clone();
                label.set_markup("<i>hmmm</i>");
            }
            Inhibit(false)
        });

        box_.upcast::<gtk::Widget>()
    }
}



fn build_library_list() -> gtk::ScrolledWindow {
    let list = gtk::ListBox::new();

    //list.add(&box_.upcast::<gtk::Widget>());
    list.add(&LibraryItem::from_markup("<b>Library</b>"));
    list.add(&LibraryItem::from_markup("Music"));
    list.add(&LibraryItem::from_markup("Podcasts"));
    list.add(&LibraryItem::from_markup("Lorem Ipsum"));
    list.add(&LibraryItem::from_markup("Dolor"));


    let scrollbox = gtk::ScrolledWindow::new::<gtk::Adjustment, gtk::Adjustment>(None, None);
    scrollbox.add(&list);
    scrollbox
}

pub struct GreatBambino {
    panes: gtk::Paned,
    vbox: gtk::Box,
    window: gtk::ApplicationWindow
}

impl GreatBambino {
    pub fn set_nav<P: IsA<gtk::Widget>>(&mut self, child: &P) {
        match &self.panes.get_child1() {
            None => {},
            Some(child1) => {
                self.panes.remove(child1);
            },
        }
        self.panes.add1(child);
        self.panes.show_all();
    }

    pub fn set_view<P: IsA<gtk::Widget>>(&mut self, child: &P) {
        match &self.panes.get_child2() {
            None => {},
            Some(child2) => {
                // TODO: set_focus_chain (and for children)
                self.panes.remove(child2);
            },
        }
        self.panes.add2(child);
        self.panes.show_all()
    }
}

fn build_ui(app: &gtk::Application) {
    // TODO: vbox(?) as parent, main contents on top, trackbar on bottom
    let panes = gtk::Paned::new(gtk::Orientation::Horizontal);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let window = gtk::ApplicationWindow::new(app);


    let src = Rc::new(nav::NavTree::default_nav_tree());
    let gb = Rc::new(Mutex::new(GreatBambino { panes, vbox, window }));

    let gbwin = &nav::NavTree::get_nav_window(src, &gb);
    let mut gbmut = gb.lock().unwrap();
    gbmut.set_nav(gbwin);
    gbmut.set_view(&build_artists_albums_songs());

    gbmut.panes.set_position(320);

    gbmut.vbox.pack_start(&gbmut.panes, true, true, 0);
    gbmut.vbox.pack_end(&gtk::Entry::new(), false, true, 0);

    gbmut.window.set_title("test example");
    gbmut.window.set_default_size(1280, 720); // width 1165 for golden ratio? 🤔
    gbmut.window.add(&gbmut.vbox);
    gbmut.window.show_all();

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