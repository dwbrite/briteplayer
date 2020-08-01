mod nav;

use crate::nav::Navigation;
use gtk::prelude::*;
use gtk::Orientation::Vertical;
use relm::{Component, Relm, Update, Widget};
use relm_derive::{widget, Msg};

#[derive(Msg)]
pub enum Msg {
    Quit,
}

pub struct UniverseModel {
    nav: Component<Navigation>,
}

#[widget]
impl Widget for Universe {
    fn model(relm: &Relm<Self>, _: ()) -> UniverseModel {
        let relm = relm::init::<Navigation>(());
        UniverseModel { nav: relm.unwrap() }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
            _ => {}
        }
    }

    // Create the widgets.
    view! {
        #[name="window"]
        gtk::Window {
            title: "briteplayer",
            // TODO: default_size: (1280, 720),
            #[name="vbox"]
            gtk::Box {
                orientation: Vertical,
                spacing: 0,
                #[name="scene_panes"]
                gtk::Paned {
                    position: 320,
                },
                #[name="placeholder"]
                gtk::Entry {},


            },
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),

        },
    }

    fn init_view(&mut self) {
        self.window.set_default_size(1280, 720); // TODO: remove
        self.window.resize(1280, 720); // TODO: remove

        self.vbox
            .set_child_packing(&self.scene_panes, true, true, 0, gtk::PackType::Start);
        self.vbox
            .set_child_packing(&self.placeholder, false, true, 0, gtk::PackType::End);

        self.scene_panes.add1(self.model.nav.widget());
    }
}

struct SceneList {
    scenes: Vec<String>,
    current_scene: usize,
}

fn main() {
    Universe::run(()).unwrap();
}
