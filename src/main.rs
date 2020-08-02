mod nav;

use crate::nav::Navigation;
use gtk::prelude::*;
use gtk::Orientation::Vertical;
use relm::{connect, Component, Relm, Update, Widget};
use relm_derive::{widget, Msg};

#[derive(Msg)]
pub enum Msg {
    Quit,
    SelectScene(usize),
}

pub struct UniverseModel {
    nav: Component<Navigation>,
    relm: Relm<Universe>,
}

#[widget]
impl Widget for Universe {
    fn model(relm: &Relm<Self>, _: ()) -> UniverseModel {
        UniverseModel {
            nav: relm::init::<Navigation>(()).unwrap(),
            relm: relm.clone(),
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
            Msg::SelectScene(idx) => {
                println!("nani?! {}", idx);
            }
        }
    }

    // Create the widgets.
    view! {
        #[name="window"]
        gtk::Window {
            title: "briteplayer",
            property_height_request: 720,
            property_width_request: 1280,
            #[name="vbox"]
            gtk::Box {
                orientation: Vertical,
                spacing: 0,
                #[name="scene_panes"]
                gtk::Paned {
                    position: 320,
                    #[name="nav_window"]
                    gtk::ScrolledWindow {

                    },
                    gtk::Entry {},
                },
                #[name="placeholder"]
                gtk::Entry {},
            },
            delete_event(_, _) => (Msg::Quit, Inhibit(false)),

        },
    }

    fn init_view(&mut self) {
        self.vbox
            .set_child_packing(&self.scene_panes, true, true, 0, gtk::PackType::Start);
        self.vbox
            .set_child_packing(&self.placeholder, false, true, 0, gtk::PackType::End);

        // self.scene_panes.add1(self.model.nav.widget());
        self.nav_window.add(self.model.nav.widget());
        let l = self.model.nav.widget();
        let relm = &self.model.relm;

        connect!(relm, l, connect_row_activated(_, b), {
            Msg::SelectScene(b.get_index() as usize)
        });
    }
}

struct SceneList {
    scenes: Vec<String>,
    current_scene: usize,
}

fn main() {
    Universe::run(()).unwrap();
}
