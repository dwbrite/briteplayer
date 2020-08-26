use crate::{GuiController, LeftNav};
use crossbeam_channel::{Receiver, Sender};
use gtk::{BoxExt, ContainerExt, GtkWindowExt, WidgetExt};

pub enum UniverseMsg {
    Quit,
    SetMedia(String),
    SetScene(String),
}

pub struct Universe {
    tx: Sender<UniverseMsg>,
    rx: Receiver<UniverseMsg>,
    nav: LeftNav,

    view: <Self as GuiController>::View,
    model: (),
}

impl Universe {
    fn __build_window(app: &gtk::Application) -> gtk::ApplicationWindow {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title("briteplayer");
        window.set_default_size(1280, 720); // width 1165 for golden ratio? 🤔

        let vbox = Self::__build_internals();
        window.add(&vbox);

        window.show_all();
        window.connect_destroy(move |_| {});

        return window;
    }

    fn __build_internals() -> gtk::Box {
        let vbox = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Vertical)
            .spacing(0)
            .name("vbox")
            .build();

        vbox.pack_start(
            &{
                let paned = gtk::PanedBuilder::new()
                    .orientation(gtk::Orientation::Horizontal)
                    .position(320)
                    .name("scene_panes")
                    .build();
                paned
            },
            true,
            true,
            0,
        );

        vbox.pack_end(&gtk::Entry::new(), false, true, 0);
        return vbox;
    }

    pub fn create(app: &gtk::Application) -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();
        let window = Self::__build_window(&app);

        Self {
            tx,
            rx,
            nav: LeftNav {},
            view: window,
            model: (),
        }
    }
}

impl GuiController for Universe {
    type Model = ();
    type View = gtk::ApplicationWindow;

    fn view(&self) -> Self::View {
        return self.view.clone();
    }

    fn model(&self) -> () {
        return self.model;
    }
}
