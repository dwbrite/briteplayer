use crate::core::scene::Scene;
use crate::core::universe::UniverseMsg;
use crate::GuiController;
use crossbeam_channel::Sender;
use glib::ObjectExt;
use gtk::{BuildableExt, ContainerExt, ListBoxExt};
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

struct ItemStuff {
    row: gtk::ListBoxRow,
    label: gtk::Label,
    scene: Rc<dyn Scene>,
}

pub struct NavModel {
    pub(crate) tree: RefCell<LinkedHashMap<String, Vec<Rc<dyn Scene>>>>,
}

pub struct NavView {
    pub(crate) window: gtk::ScrolledWindow,
    pub(crate) list: gtk::ListBox,
}

pub struct Nav {
    pub(crate) view: Rc<NavView>,
    pub(crate) model: Rc<NavModel>,
    channel: Sender<UniverseMsg>,
    treemapping: HashMap<String, ItemStuff>,
    sections: Vec<String>,
}

impl Nav {
    pub(crate) fn new(tx: Sender<UniverseMsg>) -> Self {
        let model = Rc::new(NavModel {
            tree: RefCell::new(LinkedHashMap::new()),
        });

        let view = Rc::new(NavView {
            window: gtk::ScrolledWindowBuilder::new().build(),
            list: gtk::ListBoxBuilder::new().name("").build(),
        });

        view.window.add(&view.list);

        let channel = tx.clone();
        view.list.connect_row_activated(move |_, row| unsafe {
            let scene_title: &String = row.get_data("scene").unwrap();
            channel.send(UniverseMsg::SetScene(String::from(scene_title)));
        });

        Self {
            view,
            model,
            channel: tx,

            treemapping: Default::default(),
            sections: vec![],
        }
    }

    pub(crate) fn add_scene(&mut self, scene: Rc<dyn Scene>) {
        if self.treemapping.contains_key(&scene.title()) {
            // TODO: error handling
            println!("can't have two with the same name :^)")
        }

        let section = scene.section();

        if !self.model.tree.borrow().contains_key(&section) {
            self.model
                .tree
                .borrow_mut()
                .insert(section, vec![scene.clone()]);
        } else {
            self.model
                .tree
                .borrow_mut()
                .get_mut(&section)
                .unwrap()
                .push(scene.clone());
        }
    }
}

impl GuiController for Nav {
    type Widget = gtk::ScrolledWindow;
    fn widget(&self) -> Self::Widget {
        self.view.window.clone()
    }

    fn update_view(&self) {
        let list = self.view.list.clone();

        list.get_children().iter().for_each(|w| {
            list.remove(w);
        });

        // for each section, create the section
        //   for each title in the section, create and link a label
        for section in self.model.tree.borrow().keys() {
            if !self.sections.contains(section) {
                let section_label = gtk::LabelBuilder::new()
                    .label(&format!("<b>{}</b>", section))
                    .use_markup(true)
                    .halign(gtk::Align::Start)
                    .margin(4)
                    .margin_bottom(6)
                    .margin_start(6)
                    .build();

                let section_row = gtk::ListBoxRowBuilder::new()
                    .activatable(false)
                    .selectable(false)
                    .build();

                section_row.add(&section_label);

                list.add(&section_row);
            }

            for scene in self.model.tree.borrow().get(section).unwrap() {
                if self.treemapping.contains_key(&scene.title()) {
                    continue;
                }

                let scene_label = gtk::LabelBuilder::new()
                    .label(&scene.title())
                    .halign(gtk::Align::Start)
                    .margin(1)
                    .margin_bottom(2)
                    .margin_start(12)
                    .build();
                let scene_row = gtk::ListBoxRow::new();
                scene_row.add(&scene_label);

                unsafe {
                    scene_row.set_data("scene", scene.title());
                }

                list.add(&scene_row);
            }
        }
    }
}
