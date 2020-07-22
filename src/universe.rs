use crate::{View, Controller, scenes};
use gtk::prelude::IsA;
use gtk::{ContainerExt, PanedExt, BoxExt};
use crate::scenes::SceneView;
use crate::nav::{GtkNavView, NavModel, NavController};

pub(crate) struct UniverseModel<T: SceneView> {
    pub(crate) current_scene: String,
    pub(crate) scenes: std::collections::HashMap<String, T>,
}

pub(crate) struct UniverseController<T: UniverseView, S: SceneView> {
    pub(crate) model: UniverseModel<S>,
    pub(crate) view: T,
}

impl<T: UniverseView, S: SceneView> Controller for UniverseController<T, S> {
    type Model = UniverseModel<S>;
    type View = T;

    fn model(&self) -> &Self::Model { &self.model }
    fn view(&self) -> &Self::View { &self.view }
}

pub(crate) struct GtkUniverseView {
    panes: gtk::Paned,
    vbox: gtk::Box,
}

impl UniverseView for GtkUniverseView {
    type Scene = gtk::Widget;
    type Model = UniverseModel<scenes::GtkSceneView>;

    fn from_model(model: &Self::Model) -> Self {
        // let m = model.get_ui();

        let nav_model = NavModel::default();
        let nav_controller = NavController::<GtkNavView>::from_model(nav_model);
        let navbar = nav_controller.view.get_ui();

        let pane_left = navbar;
        let pane_right = scenes::default_scene().view.get_ui();
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
    fn from_model(model: &Self::Model) -> Self;
    fn set_scene(&self, _: &Self::Scene);
}

impl View for GtkUniverseView {
    type UI = gtk::Box;
    fn get_ui(&self) -> Self::UI { self.vbox.clone() }
}

///////////////////////////////////////////////////////////////////////////////