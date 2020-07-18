use gtk::prelude::*;
use gtk::{Widget};
use std::rc::Rc;
use std::sync::Mutex;
use std::ops::DerefMut;
use crate::{GreatBambino, build_artists_albums_songs};

// struct NavRow {
//     label: gtk::Label,
//     view_widget: Option<gtk::Widget>,
//     is_header: bool,
// }

enum NavRowType {
    SectionTitle,
    Item(gtk::Widget)
}

struct NavItem {
    row_type: NavRowType,
    label: gtk::Label,

    list_row: gtk::ListBoxRow,
}

impl NavItem {
    fn from_widget(name: &str, widget: gtk::Widget) -> NavItem {
        let label = gtk::LabelBuilder::new()
            .label(name)
            .halign(gtk::Align::Start)
            .margin(1)
            .margin_bottom(2)
            .margin_start(12).build();

        NavItem::_from_row(NavRowType::Item(widget), label)
    }

    fn from_title(title: &str) -> NavItem {
        let label = gtk::LabelBuilder::new()
            .label(title)
            .use_markup(true)
            .halign(gtk::Align::Start)
            .margin(4)
            .margin_bottom(6)
            .margin_start(6)
            .build();

        NavItem::_from_row(NavRowType::SectionTitle, label)
    }

    fn _from_placeholder(name: &str) -> NavItem {
        NavItem::from_widget(name, default_music_view())
    }

    fn _from_row(row_type: NavRowType, label: gtk::Label) -> NavItem {
        let row = gtk::ListBoxRowBuilder::new();

        let row = match row_type {
            NavRowType::SectionTitle => {
                row.activatable(false).selectable(false).build()
            },
            NavRowType::Item(_) => {
                row.build()
            },
        };

        row.add(&label);

        NavItem {
            row_type,
            label,
            list_row: row
        }
    }
}

// TODO: make this more tree-like, perhaps
pub struct NavTree {
    items: Vec<NavItem>,
}

impl NavTree {
    pub fn get_nav_window(src: Rc<NavTree>, gb: &Rc<Mutex<GreatBambino>>) -> gtk::ScrolledWindow {
        let window = gtk::ScrolledWindow::new::<gtk::Adjustment, gtk::Adjustment>(None, None);

        let list = gtk::ListBox::new();

        // TODO: hbox w/ icon
        for item in &src.items {
            list.add(&item.list_row);
        }

        let gbrc = gb.clone();
        list.connect_row_selected(move |this, row| {
            if row.is_none() { return }

            match &src.clone().find_item(&row.unwrap()).unwrap().row_type {
                NavRowType::SectionTitle => { },
                NavRowType::Item(w) => {
                    let mut gbmut = gbrc.try_lock().unwrap();
                    gbmut.set_view(&mut w.clone());
                },
            }
        });

        window.add(&list);

        window
    }

    fn find_item(&self, row: &gtk::ListBoxRow) -> Option<&NavItem> {
        for item in &self.items {
            if item.list_row.eq(row) {
                return Some(item)
            }
        }
        return None
    }

    pub fn default_nav_tree() -> NavTree {
        NavTree {
            items: vec![
                NavItem::from_title("<b>Library</b>"),
                NavItem::from_widget("Music", build_artists_albums_songs().upcast::<gtk::Widget>()),
                NavItem::_from_placeholder("Podcasts"),
                NavItem::_from_placeholder("Soundcloud"),
                NavItem::_from_placeholder("SomaFM"),
                NavItem::from_title("<b>Podcasts</b>"),
                NavItem::_from_placeholder("James Earl Jones"),
                NavItem::_from_placeholder("SpaceJam"),
                NavItem::_from_placeholder("How I Met Your Mother"),
            ],
        }
    }
}

fn default_music_view() -> Widget {
    let pane = gtk::Paned::new(gtk::Orientation::Vertical);
    pane.pack1(&gtk::Entry::new(), true, false);
    pane.upcast::<gtk::Widget>()
}