use relm::{connect, Relm, Update, Widget};
use gtk::prelude::*;
use gtk::{Window, Inhibit, WindowType};
use gtk::Orientation::Vertical;
use relm_derive::{Msg, widget};

#[derive(Msg)]
pub enum Msg {
    SetScene(usize),
    Quit,
}

pub struct UniverseModel {
    scene_list: SceneList,
}

#[widget]
impl Widget for Universe {

    fn model(_: &Relm<Self>, _: ()) -> UniverseModel {
        UniverseModel {
            scene_list: SceneList {
                scenes: vec![String::from("Music"), String::from("Idfk")],
                current_scene: 0
            }
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
            _ => {}
        }
    }

    // Create the widgets.
    view! {
        gtk::Window {
            gtk::Box {
                orientation: Vertical,
            }
        }
    }
}

struct SceneList {
    scenes: Vec<String>,
    current_scene: usize,
}

fn main() {
    Universe::run(()).unwrap();
}