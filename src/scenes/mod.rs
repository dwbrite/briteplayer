use glib::{Cast};
use crate::View;

pub mod music;

pub(crate) fn default_scene() -> SceneController<GtkSceneView> {
    SceneController {
        view: GtkSceneView {
            widget: gtk::Entry::new().upcast()
        }
    }
}

pub(crate) struct SceneController<T: SceneView> {
    pub(crate) view: T,
}

pub struct GtkSceneView {
    widget: gtk::Widget,
}

impl SceneView for GtkSceneView {}

impl View for GtkSceneView {
    type UI = gtk::Widget;

    fn get_ui(&self) -> Self::UI {
        self.widget.clone()
    }
}

pub(crate) trait SceneView: View {

}