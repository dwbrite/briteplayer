use gtk::ListBoxExt;
use gtk::ListBoxRowExt;
use gtk::{ContainerExt, WidgetExt};
use relm::{connect, Relm, Update, Widget};
use relm_derive::{widget, Msg};

pub struct NavModel {
    scenes: Vec<(String, Vec<String>)>,
    current_scene: usize,
}

#[derive(Msg)]
pub enum NavMsg {
    Select(usize),
}

#[widget]
impl Widget for Navigation {
    fn model(relm: &Relm<Self>, _: ()) -> NavModel {
        NavModel {
            scenes: vec![(
                String::from("Library"),
                vec![String::from("Music"), String::from("Blank")],
            )],
            current_scene: (1),
        }
    }

    fn update(&mut self, event: NavMsg) {
        match event {
            _ => {}
        }
    }

    // Create the widgets.
    view! {
        #[name="nav_list"]
        gtk::ListBox { },
    }

    fn init_view(&mut self) {
        for section in &self.model.scenes {
            let section_label = gtk::LabelBuilder::new()
                .label(&format!("<b>{}</b>", &section.0))
                .use_markup(true)
                .halign(gtk::Align::Start)
                .margin(4)
                .margin_bottom(6)
                .margin_start(6)
                .build();

            let section_row = gtk::ListBoxRowBuilder::new()
                .activatable(false)
                .selectable(false)
                .build();
            section_row.add(&section_label);

            self.nav_list.add(&section_row);

            for child in &section.1 {
                let child_label = gtk::LabelBuilder::new()
                    .label(child)
                    .halign(gtk::Align::Start)
                    .margin(1)
                    .margin_bottom(2)
                    .margin_start(12)
                    .build();
                let child_row = gtk::ListBoxRow::new();
                child_row.add(&child_label);

                self.nav_list.add(&child_row);
            }
        }
        self.nav_list.show_all();
    }
}
