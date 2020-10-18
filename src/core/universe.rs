use crate::core::nav::{Nav, NavModel as _};
use crate::core::scene::Scene;
use crate::GuiController;
use crossbeam_channel::{Receiver, Sender};
use gtk::{BoxExt, ContainerExt, GtkWindowExt, PanedExt, WidgetExt};
use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Mutex;

pub enum UniverseMsg {
    Stop,
    SetMedia(String),
    SetScene(String),
}

// TODO: does the universe even need a "model"????

pub struct UniverseBuilder {
    pub window: gtk::ApplicationWindow,

    pub tx: Sender<UniverseMsg>,
    pub rx: Receiver<UniverseMsg>,
    pub view: Rc<UniverseView>,

    pub scenes: Vec<Rc<dyn Scene>>,

    pub navbar: Option<Nav>,
}

impl UniverseBuilder {
    pub fn new(app: &gtk::Application) -> (Self, Sender<UniverseMsg>) {
        let window = gtk::ApplicationWindow::new(app);
        window.set_title("briteplayer");
        window.set_default_size(1280, 720); // width 1165 for golden ratio? 🤔

        let view = Rc::new(UniverseView {
            window: window.clone(),
            root_box: gtk::Box::new(gtk::Orientation::Vertical, 0),
            paned: gtk::Paned::new(gtk::Orientation::Horizontal),
        });

        let (tx, rx) = crossbeam_channel::unbounded();

        (
            UniverseBuilder {
                window,
                tx: tx.clone(),
                rx,
                view,
                scenes: Default::default(),
                navbar: None,
            },
            tx.clone(),
        )
    }

    pub fn set_navbar(mut self, nav: Nav) -> Self {
        self.navbar = Some(nav);
        self
    }

    pub fn add_scene(mut self, scene: Rc<dyn Scene>) -> Self {
        self.scenes.push(scene);
        self
    }

    pub fn build(self) -> Universe {
        let mut nav = self.navbar.unwrap();

        for scene in &self.scenes {
            nav.add_scene(scene.clone());
        }
        nav.update_view();

        self.view.paned.add1(&nav.widget());
        self.view.paned.add2(&self.scenes[0].clone().widget());
        self.view.paned.set_position(320);

        self.view
            .root_box
            .pack_start(&self.view.paned.clone(), true, true, 0);
        self.view
            .root_box
            .pack_end(&gtk::EntryBuilder::new().build(), false, false, 0);

        self.view.window.add(&self.view.root_box);

        return Universe {
            window: self.window,
            tx: self.tx,
            rx: self.rx,
            view: self.view,
            // panics if either are empty
            scene: self.scenes[0].clone(),
            scenes: self.scenes,
            navbar: nav,
        };
    }
}

pub struct Universe {
    pub(crate) window: gtk::ApplicationWindow,

    pub(crate) tx: Sender<UniverseMsg>,
    pub(crate) rx: Receiver<UniverseMsg>,
    pub(crate) view: Rc<UniverseView>,
    pub(crate) scenes: Vec<Rc<dyn Scene>>,
    pub(crate) navbar: Nav,
    pub scene: Rc<dyn Scene>,
}

impl Universe {
    pub fn interpret_messages(&mut self) -> glib::Continue {
        let receiver = self.rx.clone();
        for msg in receiver.try_iter() {
            match msg {
                UniverseMsg::Stop => {
                    println!("quit");
                    return glib::Continue(false);
                }
                UniverseMsg::SetMedia(_) => {
                    println!("set_media");
                }
                UniverseMsg::SetScene(title) => {
                    // TODO: do something to actually switch the scene
                    println!("set_scene");

                    let opt_s = self.scenes.iter().find(move |&s| s.title() == title);
                    self.scene = opt_s.unwrap().clone();
                    self.update_view();
                }
            }
        }
        glib::Continue(true)
    }
}

pub struct UniverseView {
    window: <Universe as GuiController>::Widget,
    root_box: gtk::Box,
    paned: gtk::Paned,
}

impl GuiController for Universe {
    type Widget = gtk::ApplicationWindow;
    fn widget(&self) -> Self::Widget {
        self.view.window.clone()
    }

    fn update_view(&self) {
        let p = self.view.paned.clone();
        p.remove(&p.get_child2().unwrap());
        p.add2(&self.scene.clone().widget());
        self.view.window.show_all();
    }
}
