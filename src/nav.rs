use gtk::prelude::*;
use std::rc::Rc;
use std::sync::Mutex;
use crate::{GreatBambino, View};

struct NavItemModel {
    title: String,
    idk: Option<u32>
}

struct NavSectionModel {
    title: String,
    children: Vec<NavItemModel>
}

impl NavSectionModel {
    fn from_vec(title: &str, child_titles: Vec<&str>) -> Self {
        use std::iter;
        let children: Vec<NavItemModel> = child_titles.into_iter().map(|name| -> NavItemModel {
            NavItemModel { title: String::from(name), idk: None }
        }).collect();

        Self {
            title: String::from(title),
            children
        }
    }
}

pub(crate) struct NavModel {
    sections: Vec<NavSectionModel>
}

impl Default for NavModel {
    fn default() -> Self {
        let sections = vec![
            NavSectionModel::from_vec("Library", vec![
                "Music", "Soundcloud", "SomaFM"
            ]),
            NavSectionModel::from_vec("Podcasts", vec![
                "James Earl Jones", "SpaceJam", "How I Met Your Mother"
            ]),
        ];

        Self { sections }
    }
}

pub(crate) struct NavController<T: NavView> {
    model: NavModel,
    pub(crate) view: T,
}

impl<T: NavView> NavController<T> {
    pub(crate) fn from_model(model: NavModel) -> Self {
        let view = T::from_model(&model);
        Self {
            model,
            view,
        }
    }

    fn select_scene(tx: crossbeam_channel::Sender<TMP>, scene: &str) {
        tx.send(TMP::PH);
    }
}

pub(crate) struct GtkNavView {
    root: gtk::ScrolledWindow,
    list: gtk::ListBox,
    list_items: Vec<NavListItem>,
}

struct NavListItem {
    row: gtk::ListBoxRow,
    label: gtk::Label,
}

impl NavView for GtkNavView {
    fn from_model(model: &NavModel) -> Self {

        let mut nav_view = GtkNavView {
            root: gtk::ScrolledWindow::new::<gtk::Adjustment, gtk::Adjustment>(None, None),
            list: gtk::ListBox::new(),
            list_items: vec![],
        };

        for section in &model.sections {
            let section_label = gtk::LabelBuilder::new()
                .label(&format!("<b>{}</b>", &section.title))
                .use_markup(true)
                .halign(gtk::Align::Start)
                .margin(4)
                .margin_bottom(6)
                .margin_start(6)
                .build();

            let section_row = gtk::ListBoxRowBuilder::new()
                .activatable(false).selectable(false).build();
            section_row.add(&section_label);

            nav_view.list.add(&section_row);
            nav_view.list_items.push(NavListItem { row: section_row, label: section_label });


            for child in &section.children {
                let child_label = gtk::LabelBuilder::new()
                    .label(&child.title).halign(gtk::Align::Start)
                    .margin(1).margin_bottom(2).margin_start(12).build();
                let child_row = gtk::ListBoxRow::new();
                child_row.add(&child_label);

                nav_view.list.add(&child_row);
                nav_view.list_items.push(NavListItem { row: child_row, label: child_label });
            }
        }

        nav_view.list.connect_row_activated(|_this, row| {
            // if row.is_none() { return }


        });

        nav_view.root.add(&nav_view.list);
        nav_view
    }
}

impl View for GtkNavView {
    type UI = gtk::ScrolledWindow;

    fn get_ui(&self) -> Self::UI {
        self.root.clone()
    }
}

enum TMP { PH }

pub(crate) trait NavView: View {
    fn from_model(model: &NavModel) -> Self;
    // fn get_sender(&self) -> crossbeam_channel::Sender<TMP>;
}
