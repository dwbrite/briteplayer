use glib::{Cast};

pub mod vmusic {
    use gtk::{BoxExt, PanedExt, WidgetExt};
    use crate::views::ContentView;
    use crate::build_library_list;
    use glib::{Cast};

    pub(crate) struct MusicView {
        outer_pane: gtk::Paned,
        _inner_pane: gtk::Paned,
        _artists_box: gtk::Box,
        _albums_box: gtk::Box,
        _songs_box: gtk::Box,
    }

    impl MusicView {
        pub(crate) fn new() -> MusicView {
            let vpane = gtk::Paned::new(gtk::Orientation::Vertical);
            let hpane = gtk::Paned::new(gtk::Orientation::Horizontal);

            let artists = {
                let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
                vbox.pack_start(&gtk::Label::new(Some("<b><i>Artists</i></b>")), false, false, 0);
                vbox.pack_end(&build_library_list(), true, true, 0);
                vbox
            };

            let albums = {
                let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
                vbox.pack_start(&gtk::Label::new(Some("<b><i>Albums</i></b>")), false, false, 0);
                vbox.pack_end(&build_library_list(), true, true, 0);
                vbox
            };

            let songs = {
                let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
                vbox.pack_start(&gtk::Label::new(Some("<b><i>Songs</i></b>")), false, false, 0);
                vbox.pack_end(&build_library_list(), true, true, 0);
                vbox
            };

            hpane.add1(&artists);
            hpane.add2(&albums);
            vpane.add1(&hpane);
            vpane.add2(&songs);

            vpane.show_all();

            MusicView {
                outer_pane: vpane,
                _inner_pane: hpane,
                _artists_box: artists,
                _albums_box: albums,
                _songs_box: songs
            }
        }
    }

    impl ContentView for MusicView {
        fn get_widget(&self) -> gtk::Widget {
            self.outer_pane.clone().upcast::<gtk::Widget>()
        }
    }
}

pub trait ContentView {
    fn get_widget(&self) -> gtk::Widget;
}

pub struct _Blank {
    widget: gtk::Entry
}

impl _Blank {
    pub(crate) fn new() -> _Blank {
        _Blank {
            widget: gtk::Entry::new()
        }
    }
}

impl ContentView for _Blank {
    fn get_widget(&self) -> gtk::Widget {
        self.widget.clone().upcast::<gtk::Widget>()
    }
}