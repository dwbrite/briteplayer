#[macro_use]
extern crate crossbeam_channel;
extern crate glib;
extern crate gtk;
extern crate gio;
extern crate gdk;

// To import all needed traits.
use gtk::prelude::*;
use gio::prelude::*;

use std::rc::Rc;
use std::sync::Mutex;
use std::{env, thread};
use crate::views::vmusic::MusicView;
use crate::views::ContentView;
use crossbeam_channel::unbounded;

mod nav;
mod views;

mod library_item {
    use gtk::{LabelExt, WidgetExt, ContainerExt, Inhibit};
    use gtk::prelude::Cast;

    pub fn from_markup(title: &str) -> gtk::Widget {
        let label = gtk::Label::new(None);
        label.set_markup(&title);
        label.set_halign(gtk::Align::Start);

        let box_ = gtk::ListBoxRow::new();
        box_.add(&label);
        box_.show_all();

        let l_rc = label.clone();
        box_.connect_event(move |row, e| {
            // TODO: change from "focus" to "select"
            if row.clone().has_focus() && e.get_event_type() == gdk::EventType::FocusChange {
                let label = l_rc.clone();
                label.set_markup("<i>hmmm</i>");
            }
            Inhibit(false)
        });

        box_.upcast::<gtk::Widget>()
    }
}


fn build_library_list() -> gtk::ScrolledWindow {
    let list = gtk::ListBox::new();

    //list.add(&box_.upcast::<gtk::Widget>());
    list.add(&library_item::from_markup("<b>Library</b>"));
    list.add(&library_item::from_markup("Music"));
    list.add(&library_item::from_markup("Podcasts"));
    list.add(&library_item::from_markup("Lorem Ipsum"));
    list.add(&library_item::from_markup("Dolor"));


    let scrollbox = gtk::ScrolledWindow::new::<gtk::Adjustment, gtk::Adjustment>(None, None);
    scrollbox.add(&list);
    scrollbox
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
    use gst::prelude::*;
    use std::io;
    use std::io::Write;
    use self::gst::MessageView;
    use crate::player;

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

pub struct GreatBambino {
    panes: gtk::Paned,
    vbox: gtk::Box,
    window: gtk::ApplicationWindow,
    sender: crossbeam_channel::Sender<player::Action>,
}

impl GreatBambino {
    pub fn set_nav<P: IsA<gtk::Widget>>(&mut self, child: &P) {
        match &self.panes.get_child1() {
            None => {},
            Some(child1) => {
                self.panes.remove(child1);
            },
        }
        self.panes.add1(child);
        self.panes.show_all();
    }

    pub fn set_view<P: IsA<gtk::Widget>>(&mut self, child: &P) {
        match &self.panes.get_child2() {
            None => {},
            Some(child2) => {
                // TODO: set_focus_chain (and for children)
                self.panes.remove(child2);
            },
        }
        self.panes.add2(child);
        self.panes.show_all()
    }
}

fn build_ui(app: &gtk::Application) {
    let (s1, r1) = unbounded();
    let (s2, r2) = (s1.clone(), r1.clone());

    // TODO: vbox(?) as parent, main contents on top, trackbar on bottom
    let panes = gtk::Paned::new(gtk::Orientation::Horizontal);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let window = gtk::ApplicationWindow::new(app);

    window.connect_button_press_event(move |_, e| -> Inhibit {
        s2.send(player::Action::PlayPause);
        println!("button pressed! {:?}", e);
        Inhibit(false)
    });


    let src = Rc::new(nav::NavTree::default_nav_tree());
    let gb = Rc::new(Mutex::new(GreatBambino { panes, vbox, window, sender: s1 }));

    let gbwin = &nav::NavTree::get_nav_window(src, &gb);
    let mut gbmut = gb.lock().unwrap();
    gbmut.set_nav(gbwin);
    gbmut.set_view(&MusicView::new().get_widget());

    gbmut.panes.set_position(320);

    gbmut.vbox.pack_start(&gbmut.panes, true, true, 0);
    gbmut.vbox.pack_end(&gtk::Entry::new(), false, true, 0);
    gbmut.window.set_title("test example");
    gbmut.window.set_default_size(1280, 720); // width 1165 for golden ratio? 🤔
    gbmut.window.add(&gbmut.vbox);
    gbmut.window.show_all();

    thread::spawn(move || {
        gs_test::tut_main(r2);
    });
}

fn main() {
    let application = gtk::Application::new(
        Some("org.dwbrite.briteplayer"),
        gio::ApplicationFlags::FLAGS_NONE
    ).expect("Application::new failed");

    application.connect_activate(|app| {
        build_ui(&app);
    });
    application.run(&env::args().collect::<Vec<_>>());
}