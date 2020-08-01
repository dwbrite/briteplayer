use gtk::ListBoxExt;
use gtk::ListBoxRowExt;
use gtk::{ContainerExt, WidgetExt};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;

pub struct NavModel {
    scenes: Vec<(String, Vec<String>)>,
    current_scene: usize,
}

#[derive(Msg)]
pub enum NavMsg {
    Select(usize),
}

pub struct Navigation {
    model: NavModel,
    widget: gtk::ScrolledWindow,
}

impl Update for Navigation {
    type Model = NavModel;
    type ModelParam = ();
    type Msg = NavMsg;

    fn model(relm: &Relm<Self>, param: Self::ModelParam) -> Self::Model {
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
            NavMsg::Select(scene) => {
                println!("Setting scene to {}", scene);
                self.model.current_scene = scene;
            }
        }
    }
}

impl Widget for Navigation {
    type Root = gtk::ScrolledWindow;

    fn root(&self) -> Self::Root {
        self.widget.clone()
    }

    // Create the widgets.
    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let root = gtk::ScrolledWindow::new::<gtk::Adjustment, gtk::Adjustment>(None, None);
        let list = gtk::ListBox::new();
        // let mut list_items = vec![];

        for section in &model.scenes {
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

            list.add(&section_row);

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

                list.add(&child_row);
                // nav_view.list_items.push(NavListItem {
                //     row: child_row,
                //     label: child_label,
                // });
            }
        }

        connect!(relm, list, connect_row_activated(a, b), {
            // returns the message
            NavMsg::Select(b.get_index() as usize)
        });

        root.add(&list);
        root.show_all();
        Self {
            model,
            widget: root,
        }
    }
}
