pub(crate) mod music;
pub(crate) mod spacecasts;

use gtk::prelude::Cast;
use gtk::{BuildableExt, ContainerExt, PanedExt, Widget};
use std::hash::Hash;

pub trait Scene {
    fn on_select(&self);
    fn widget(&self) -> gtk::Widget;
    fn title(&self) -> String;
    fn section(&self) -> String;
}
