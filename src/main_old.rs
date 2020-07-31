mod universe;

extern crate crossbeam_channel;
extern crate glib;
extern crate gtk;
extern crate gio;
extern crate gdk;

// To import all needed traits.
use gtk::prelude::*;
use gio::prelude::*;

use std::{env};
use crate::universe::{GtkUniverseView, UniverseModel, UniverseView, UniverseController, UniverseSignal, GtkUniverse};
use crate::scenes::{SceneController, GtkScene};
use std::collections::HashMap;

mod nav;
mod scenes;
mod player;


// fn _build_ui(app: &gtk::Application) {
//     let (s1, r1) = unbounded();
//     let (s2, r2) = (s1.clone(), r1.clone());
//
//     window.connect_button_press_event(move |_, e| -> Inhibit {
//         match e.get_event_type() {
//             EventType::ButtonPress => {
//                 s2.send(player::ControllerAction::PlayPause);
//             },
//             EventType::DoubleButtonPress => {
//                 s2.send(player::ControllerAction::SetUri(String::from("https://upload.wikimedia.org/wikipedia/commons/c/c8/Example.ogg")));
//             },
//             _ => {}
//         }
//         println!("button pressed! {:?}", e);
//         Inhibit(false)
//     });
//
//     gbmut.set_view(&MusicView::new().get_widget());
//
//     thread::spawn(move || {
//         player::spawn(r2);
//     });
// }

fn build_universe(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);


    let (mut universe, tx) = {
        // GtkViewModel needs

        let model: UniverseModel<scenes::GtkSceneView> = UniverseModel {
            current_scene: String::from("music"),
            scenes: {
                let mut map: HashMap<String, GtkScene> = Default::default();
                map.insert(String::from("music"), scenes::default_scene());
                map
            }
        };

        GtkUniverse::gtk_from(model)
    };

    window.set_title("briteplayer");
    window.set_default_size(1280, 720); // width 1165 for golden ratio? 🤔
    window.add(&universe.view().get_ui());
    window.show_all();

    // TODO: investigate if pattern or antipattern
    window.connect_destroy(move |_| {
        tx.send(UniverseSignal::Destroy).unwrap();
    });

    glib::idle_add_local(move || {
        universe.update();
        Continue(universe.idle_looping)
    });
}

fn main() {
    let application = gtk::Application::new(
        Some("org.dwbrite.briteplayer"),
        gio::ApplicationFlags::FLAGS_NONE
    ).expect("Application::new failed");

    application.connect_activate(move |app| {
        build_universe(&app);
    });
    application.run(&env::args().collect::<Vec<_>>());
}

pub(crate) trait View {
    type UI;
    fn get_ui(&self) -> Self::UI;
}

pub(crate) trait Controller {
    type Model;
    type View;

    fn model(&self) -> &Self::Model;
    fn view(&self) -> &Self::View;

    fn update(&mut self);
}
