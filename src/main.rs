mod core;

use crate::core::scene::music::Music;
use crate::core::scene::spacecasts::Spacecast;
use crate::core::universe;
use crate::core::universe::{Universe, UniverseMsg};
use crate::player::Action;
use crossbeam_channel::unbounded;
use gio::prelude::ApplicationExtManual;
use gio::ApplicationExt;
use glib::signal::Inhibit;
use glib::Continue;
use gtk::prelude::IsA;
use gtk::WidgetExt;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;
use std::{env, thread};

pub trait GuiController {
    type Widget: IsA<gtk::Widget>;
    fn widget(&self) -> Self::Widget;
    fn update_view(&self);
}

fn main() {
    let application = gtk::Application::new(
        Some("org.dwbrite.briteplayer"),
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Application::new failed");

    application.connect_activate(move |app| {
        let mut universe = build_universe(app);
        universe.widget().show_all();

        // stop the universe when the app is destroyed
        let tx = universe.tx.clone();
        universe.widget().connect_destroy(move |_| {
            tx.send(UniverseMsg::Stop);
        });

        let (t2, r2) = unbounded();

        // spawn thread for playback
        thread::spawn(move || {
            gs_test::tut_main(r2);
        });

        universe.window.connect_key_press_event(move |_, _| {
            t2.send(Action::PlayPause);
            Inhibit(false)
        });

        // interpret any messages that have been sent recently
        glib::idle_add_local(move || universe.interpret_messages());
    });

    application.run(&env::args().collect::<Vec<_>>());
}

fn build_universe(app: &gtk::Application) -> universe::Universe {
    let (builder, tx) = universe::UniverseBuilder::new(&app);

    let navbar = core::nav::Nav::new(tx);

    let test_scene = Rc::new(Music::init());

    builder
        .set_navbar(navbar)
        .add_scene(test_scene.clone())
        .add_scene(Rc::new(Spacecast::init()))
        .build()
}

mod player {
    pub(crate) enum Action {
        PlayPause,
        SkipBack,
        SkipFwd,
        Seek(u32),
    }
}

mod gs_test {
    extern crate crossbeam_channel;
    extern crate gstreamer as gst;
    use self::gst::MessageView;
    use crate::player;
    use gst::prelude::*;
    use std::io;
    use std::io::Write;

    struct CustomData {
        playbin: gst::Element,    // Our one and only element
        playing: bool,            // Are we in the PLAYING state?
        terminate: bool,          // Should we terminate execution?
        seek_enabled: bool,       // Is seeking enabled for this media?
        seek_done: bool,          // Have we performed the seek already?
        duration: gst::ClockTime, // How long does this media last, in nanoseconds
    }

    pub(crate) fn tut_main(rx: crossbeam_channel::Receiver<player::Action>) {
        gst::init().expect("could not init gstreamer");

        let uri = "https://upload.wikimedia.org/wikipedia/commons/c/c8/Example.ogg";

        // Creat the playbin element
        let playbin = gst::ElementFactory::make("playbin", Some("playbin"))
            .expect("Failed to create playbin element");

        playbin
            .set_property("uri", &uri)
            .expect("Can't set uri property on playbin");

        // Start playing
        playbin
            .set_state(gst::State::Playing)
            .expect("Unable to set the playbin to the `Playing` state");

        // Listen to the bus
        let bus = playbin.get_bus().unwrap();
        let mut custom_data = CustomData {
            playbin,
            playing: false,
            terminate: false,
            seek_enabled: false,
            seek_done: false,
            duration: gst::CLOCK_TIME_NONE,
        };

        while !custom_data.terminate {
            if let Ok(action) = rx.try_recv() {
                println!("received action PlayPause");
                if custom_data.playing {
                    custom_data.playbin.set_state(gst::State::Paused);
                    custom_data.playing = false;
                } else {
                    custom_data.playbin.set_state(gst::State::Playing);
                    custom_data.playing = true;
                }
            }

            let msg = bus.timed_pop(100 * gst::MSECOND);

            match msg {
                Some(msg) => {
                    handle_message(&mut custom_data, &msg);
                }

                None => {
                    if custom_data.playing {
                        let position = custom_data
                            .playbin
                            .query_position::<gst::ClockTime>()
                            .expect("Could not query current position.");

                        // If we didn't know it yet, query the stream duration
                        if custom_data.duration == gst::CLOCK_TIME_NONE {
                            custom_data.duration = custom_data
                                .playbin
                                .query_duration()
                                .expect("Could not query current duration.")
                        }

                        // Print current position and total duration
                        print!("\rPosition {} / {}", position, custom_data.duration);
                        io::stdout().flush().unwrap();

                        if custom_data.seek_enabled
                            && !custom_data.seek_done
                            && position > 10 * gst::SECOND
                        {
                            println!("\nReached 10s, performing seek...");
                            custom_data
                                .playbin
                                .seek_simple(
                                    gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT,
                                    30 * gst::SECOND,
                                )
                                .expect("Failed to seek.");
                            custom_data.seek_done = true;
                        }
                    }
                }
            }
        }

        custom_data
            .playbin
            .set_state(gst::State::Null)
            .expect("Unable to set the playbin to the `Null` state");
    }

    fn handle_message(custom_data: &mut CustomData, msg: &gst::Message) {
        match msg.view() {
            MessageView::Error(err) => {
                println!(
                    "Error received from element {:?}: {} ({:?})",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );
                custom_data.terminate = true;
            }
            MessageView::Eos(..) => {
                println!("End-Of-Stream reached.");
                custom_data.terminate = true;
            }
            MessageView::DurationChanged(_) => {
                // The duration has changed, mark the current one as invalid
                custom_data.duration = gst::CLOCK_TIME_NONE;
            }
            MessageView::StateChanged(state_changed) => {
                if state_changed
                    .get_src()
                    .map(|s| s == custom_data.playbin)
                    .unwrap_or(false)
                {
                    let new_state = state_changed.get_current();
                    let old_state = state_changed.get_old();

                    println!(
                        "Pipeline state changed from {:?} to {:?}",
                        old_state, new_state
                    );

                    custom_data.playing = new_state == gst::State::Playing;
                    if custom_data.playing {
                        let mut seeking = gst::query::Seeking::new(gst::Format::Time);
                        if custom_data.playbin.query(&mut seeking) {
                            let (seekable, start, end) = seeking.get_result();
                            custom_data.seek_enabled = seekable;
                            if seekable {
                                println!("Seeking is ENABLED from {:?} to {:?}", start, end)
                            } else {
                                println!("Seeking is DISABLED for this stream.")
                            }
                        } else {
                            eprintln!("Seeking query failed.")
                        }
                    }
                }
            }
            _ => (),
        }
    }
}
