extern crate crossbeam_channel;
extern crate gstreamer as gst;
use gst::prelude::*;
use std::io;
use std::io::Write;
use self::gst::MessageView;

pub(crate) enum ControllerAction {
    PlayPause,
    SkipBack,
    SkipFwd,
    Seek(u32),
    SetUri(String)
}

struct PlaybackData {
    playbin: gst::Element,
    playing: bool,
    terminate: bool,
    seek_enabled: bool,
    seek_done: bool,
    duration: gst::ClockTime, // media length
    uri: Option<String>
}

// TODO: return a PlaybackState* receiever
pub(crate) fn spawn(rx: crossbeam_channel::Receiver<ControllerAction>) {
    gst::init().expect("could not init gstreamer");

    // Creat the playbin element
    let playbin = gst::ElementFactory::make("playbin", Some("playbin"))
        .expect("Failed to create playbin element");

    // Listen to the bus
    let mut custom_data = PlaybackData {
        playbin,
        playing: false,
        terminate: false,
        seek_enabled: false,
        seek_done: false,
        duration: gst::CLOCK_TIME_NONE,
        uri: None,
    };

    custom_data.listen_forever(rx);
}

impl PlaybackData {
    fn listen_forever(&mut self, rx: crossbeam_channel::Receiver<ControllerAction>) {
        let bus = self.playbin.get_bus().unwrap();
        while !self.terminate {
            // try to receive an action // todo move this to a function?
            if let Ok(action) = rx.try_recv() {
                println!("received action PlayPause");
                match action {
                    ControllerAction::PlayPause => {
                        if self.playing {
                            self.playbin.set_state(gst::State::Paused);
                        } else {
                            self.playbin.seek_simple(gst::SeekFlags::FLUSH | gst::SeekFlags::SEGMENT | gst::SeekFlags::KEY_UNIT, 0 * gst::SECOND);
                            self.playbin.set_state(gst::State::Playing);
                        }
                    },
                    ControllerAction::SkipBack => {},
                    ControllerAction::SkipFwd => {},
                    ControllerAction::Seek(ms) => {
                        self.playbin.seek_simple(gst::SeekFlags::SEGMENT | gst::SeekFlags::KEY_UNIT, 0 * gst::SECOND);
                    },
                    ControllerAction::SetUri(uri) => {
                        self.playbin.set_property("uri", &uri);
                        self.uri = Some(uri);
                        self.playbin.set_state(gst::State::Playing);
                    }
                }
            }

            let pbc = self.playbin.clone();
            self.playbin.connect("about-to-finish", false, move |_| -> Option<gst::glib::Value> {
                // pbc.seek_simple(gst::SeekFlags::SEGMENT | gst::SeekFlags::KEY_UNIT, 0 * gst::SECOND);
                pbc.seek_simple(gst::SeekFlags::FLUSH | gst::SeekFlags::SEGMENT | gst::SeekFlags::KEY_UNIT, 0 * gst::SECOND);
                // println!(";)");
                None
            });


            // TODO: find out if this is sleeping* for 100ms?
            let msg = bus.timed_pop(100 * gst::MSECOND);

            match msg {
                Some(msg) => {
                    self.handle_message(&msg);
                }

                None => {
                    if self.playing {
                        let position = self
                            .playbin
                            .query_position::<gst::ClockTime>()
                            .expect("Could not query current position.");

                        // If we didn't know it yet, query the stream duration
                        if self.duration == gst::CLOCK_TIME_NONE {
                            self.duration = self
                                .playbin
                                .query_duration()
                                .expect("Could not query current duration.")
                        }

                        // Print current position and total duration
                        print!("\rPosition {} / {}", position, self.duration);
                        io::stdout().flush().unwrap();

                        if self.seek_enabled
                            && !self.seek_done
                            && position > 10 * gst::SECOND
                        {
                            println!("\nReached 10s, performing seek...");
                            self
                                .playbin
                                .seek_simple(
                                    gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT,
                                    30 * gst::SECOND,
                                )
                                .expect("Failed to seek.");
                            self.seek_done = true;
                        }
                    }
                }
            }
        }
    }

    fn handle_message(&mut self, msg: &gst::Message) {
        match msg.view() {
            MessageView::Error(err) => {
                println!(
                    "Error received from element {:?}: {} ({:?})",
                    err.get_src().map(|s| s.get_path_string()),
                    err.get_error(),
                    err.get_debug()
                );
                self.terminate = true;
            }
            MessageView::Eos(_) => {
                // self.uri = None;
                self.playbin.seek_simple(gst::SeekFlags::SEGMENT | gst::SeekFlags::KEY_UNIT, 0 * gst::SECOND);
                // self.playbing.set
                // TODO: must go from paused to ready, but is this correct?
                // self.playbin.set_state(gst::State::Playing);
                //self.playbin.set_state(gst::State::Ready);
                // self.playbin.set_state(gst::State::Null);
                // self.playbin.set_property("uri", &String::from(""));
                // self.playing = false;
            }
            MessageView::DurationChanged(_) => {
                // The duration has changed, mark the current one as invalid
                self.duration = gst::CLOCK_TIME_NONE;
            }
            MessageView::StateChanged(state_changed) => {
                if state_changed
                    .get_src()
                    .map(|s| s == self.playbin)
                    .unwrap_or(false)
                {
                    let new_state = state_changed.get_current();

                    self.playing = new_state == gst::State::Playing;
                    if self.playing {
                        let mut seeking = gst::query::Seeking::new(gst::Format::Time);
                        if self.playbin.query(&mut seeking) {
                            let (seekable, start, end) = seeking.get_result();
                            self.seek_enabled = seekable;
                        } else {
                            eprintln!("Seeking query failed.")
                        }
                    }
                }
            },
            _ => (),
        }
    }
}