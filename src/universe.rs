use crate::{View, Controller, scenes};
use gtk::{ContainerExt, PanedExt, BoxExt};
use crate::scenes::{SceneView, GtkSceneView, SceneController};
use crate::nav::{GtkNavView, NavController, NavModel};

pub(crate) struct UniverseModel<S: SceneView> {
    pub(crate) current_scene: String,
    pub(crate) scenes: std::collections::HashMap<String, SceneController<S>>,
}

pub(crate) enum UniverseSignal {
    Destroy,
    SetScene(String)
}

pub(crate) type GtkUniverse = UniverseController<GtkUniverseView, scenes::GtkSceneView>;

pub(crate) struct UniverseController<T: UniverseView, S: SceneView> {
    pub(crate) model: UniverseModel<S>,
    pub(crate) view: T,

    pub(crate) sender_replica: crossbeam_channel::Sender<UniverseSignal>,
    pub(crate) signal_receiver: crossbeam_channel::Receiver<UniverseSignal>,

    pub(crate) idle_looping: bool,

    nav_controller: crate::nav::NavController<GtkNavView>,
}

impl GtkUniverse {
    pub(crate) fn gtk_from(model: UniverseModel<GtkSceneView>) -> (GtkUniverse, crossbeam_channel::Sender<UniverseSignal>) {
        let (tx, rx) = crossbeam_channel::unbounded();

        let nav_model = NavModel::default();
        let nav_controller = NavController::<GtkNavView>::from_model(nav_model, tx.clone());

        let view = GtkUniverseView::from_model(&model, &nav_controller);
        // FIXME: this seems really messy...
        // view.panes.add1(&nav_controller.view().get_ui());

        let universe = GtkUniverse {
            model,
            view,
            sender_replica: tx.clone(),
            signal_receiver: rx,
            idle_looping: true,
            nav_controller
        };

        (universe, tx)
    }
}

impl<T: UniverseView, S: SceneView> Controller for UniverseController<T, S> {
    type Model = UniverseModel<S>;
    type View = T;

    fn model(&self) -> &Self::Model { &self.model }
    fn view(&self) -> &Self::View { &self.view }
    fn update(&mut self) {
        self.nav_controller.update();

        while !self.signal_receiver.is_empty() {
            match self.signal_receiver.recv().unwrap() {
                UniverseSignal::Destroy => {
                    self.idle_looping = false;
                    println!("Have a nice day!");
                },
                UniverseSignal::SetScene(s) => {
                    println!("set the scene to {}", s)
                    // TODO: set the scene someehow, I guess
                }
            }
        }
    }
}

pub(crate) struct GtkUniverseView {
    panes: gtk::Paned,
    vbox: gtk::Box,
}

impl UniverseView for GtkUniverseView {
    type Scene = gtk::Widget;
    type Model = UniverseModel<scenes::GtkSceneView>;

    fn from_model(model: &Self::Model, nav_controller: &NavController::<GtkNavView>) -> Self {
        let navbar = nav_controller.view.get_ui();

        let pane_left = navbar;

        let scene = model.scenes.get(&model.current_scene).unwrap();
        let pane_right = scene.view.get_ui();
        let panes = gtk::Paned::new(gtk::Orientation::Horizontal);
        panes.add1(&pane_left);
        panes.add2(&pane_right);

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        panes.set_position(320);

        vbox.pack_start(&panes, true, true, 0);
        vbox.pack_end(&gtk::Entry::new(), false, true, 0);

        GtkUniverseView {
            panes,
            vbox,
        }
    }

    fn set_scene(&self, scene: &Self::Scene) {
        if let Some(child) = &self.panes.get_child2() {
            self.panes.remove(child);
        }
        self.panes.add2(scene);
    }
}

pub(crate) trait UniverseView: View {
    type Scene;
    type Model;
    fn from_model(model: &Self::Model, nav_controller: &NavController<GtkNavView>) -> Self;
    fn set_scene(&self, _: &Self::Scene);
}

impl View for GtkUniverseView {
    type UI = gtk::Box;
    fn get_ui(&self) -> Self::UI { self.vbox.clone() }
}