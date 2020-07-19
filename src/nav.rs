use gtk::prelude::*;
use std::rc::Rc;
use std::sync::Mutex;
use crate::{GreatBambino};

use crate::views::ContentView;

enum NavRowType {
    SectionTitle,
    Item(gtk::Widget)
}

struct NavItem {
    row_type: NavRowType,
    _label: gtk::Label,
    list_row: gtk::ListBoxRow,
}

impl NavItem {
    fn from_view(name: &str, view: &dyn ContentView) -> NavItem {
        let label = gtk::LabelBuilder::new()
            .label(name)
            .halign(gtk::Align::Start)
            .margin(1)
            .margin_bottom(2)
            .margin_start(12).build();

        NavItem::_from_row(NavRowType::Item(view.get_widget()), label)
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
        NavItem::from_view(name, &crate::views::_Blank::new())
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
            _label: label,
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
        list.connect_row_selected(move |_this, row| {
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
        let music_view = crate::views::vmusic::MusicView::new();

        NavTree {
            items: vec![
                NavItem::from_title("<b>Library</b>"),
                NavItem::from_view("Music", &music_view),
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
