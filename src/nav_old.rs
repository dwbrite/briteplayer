use gtk::prelude::*;
use crate::{View, Controller};
use crate::universe::UniverseSignal;
use crossbeam_channel::Sender;

struct NavItemModel {
    title: String,
    scene: String
}

struct NavSectionModel {
    title: String,
    children: Vec<NavItemModel>
}

impl NavSectionModel {
    fn from_vec(title: &str, child_titles: Vec<(&str, &str)>) -> Self {
        let children: Vec<NavItemModel> = child_titles.into_iter().map(|info| -> NavItemModel {
            NavItemModel { title: String::from(info.0), scene: String::from(info.1) }
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
                ("Music", "music"), ("Soundcloud", "soundcloud"), ("SomaFM", "soma")
            ]),
            NavSectionModel::from_vec("Podcasts", vec![
                ("James Earl Jones", ""), ("SpaceJam", ""), ("How I Met Your Mother", "")
            ]),
        ];

        Self { sections }
    }
}

pub(crate) struct NavController<T: NavView> {
    model: NavModel,
    pub(crate) view: T,
    universe_tx: crossbeam_channel::Sender<UniverseSignal>,
}

impl<T: NavView> NavController<T> {
    pub(crate) fn from_model(model: NavModel, universe_tx: crossbeam_channel::Sender<UniverseSignal>) -> Self {
        let view = T::from_model(&model, universe_tx.clone());
        Self {
            model,
            view,
            universe_tx
        }
    }
}

impl<T: NavView> Controller for NavController<T> {
    type Model = NavModel;
    type View = T;

    fn model(&self) -> &Self::Model {
        &self.model
    }

    fn view(&self) -> &Self::View {
        &self.view
    }

    fn update(&mut self) {
        // TODO: read from some receiver and do something, I guess, lol
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
    fn from_model(model: &NavModel, tx: Sender<UniverseSignal>) -> Self {

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

                unsafe { child_row.set_data("scene", child.scene.clone()); }

                nav_view.list.add(&child_row);
                nav_view.list_items.push(NavListItem { row: child_row, label: child_label });
            }
        }

        let txc = tx.clone();
        nav_view.list.connect_row_activated(move |_, row| {
            unsafe {
                let scene: &String = row.get_data("scene").unwrap();
                txc.clone().send(UniverseSignal::SetScene(String::from(scene)));
            }
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

pub(crate) trait NavView: View {
    fn from_model(model: &NavModel, tx: Sender<UniverseSignal>) -> Self;
}
