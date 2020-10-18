use crate::core::scene::Scene;
use gtk::prelude::Cast;
use gtk::{ContainerExt, PanedExt, Widget};

struct MusicModel {}

struct MusicView {
    vpane: gtk::Paned,
    hpane: gtk::Paned,
    primary_list: gtk::ListBox,
    secondary_list: gtk::ListBox,
    tertiary_list: gtk::ListBox,
}

// TODO: create UI+/Model from music sources?
pub(crate) struct Music {
    view: MusicView,
    model: MusicModel,
}

impl Music {
    pub(crate) fn init() -> Music {
        let model = MusicModel {};

        let view = {
            let vpane = gtk::Paned::new(gtk::Orientation::Vertical);
            let hpane = gtk::Paned::new(gtk::Orientation::Horizontal);

            let list1 = gtk::ListBox::new();
            let list2 = gtk::ListBox::new();
            let list3 = gtk::ListBox::new();
            for i in 0..5 {
                let w1 = gtk::ListBoxRow::new();
                w1.add(&gtk::LabelBuilder::new().label(&i.to_string()).build());
                let w2 = gtk::ListBoxRow::new();
                w2.add(&gtk::LabelBuilder::new().label(&i.to_string()).build());
                let w3 = gtk::ListBoxRow::new();
                w3.add(&gtk::LabelBuilder::new().label(&i.to_string()).build());
                list1.add(&w1);
                list2.add(&w2);
                list3.add(&w3);
            }

            vpane.pack1(&hpane, true, false);
            vpane.pack2(&list1, true, false);
            hpane.pack1(&list2, true, false);
            hpane.pack2(&list3, true, false);

            MusicView {
                vpane,
                hpane,
                primary_list: list1,
                secondary_list: list2,
                tertiary_list: list3,
            }
        };

        Music { model, view }
    }
}

impl Scene for Music {
    fn on_select(&self) {
        println!("selected");
    }

    fn widget(&self) -> Widget {
        self.view.vpane.clone().upcast()
    }

    fn title(&self) -> String {
        String::from("Music")
    }

    fn section(&self) -> String {
        String::from("Library")
    }
}
