use relm::Widget;
use gtk::{prelude::*};
use relm_derive::{Msg, widget};

#[derive(Msg)]
pub enum Msg {
    Pressed
}

pub struct Model {
    text: String
}

#[widget]
impl Widget for Wdg {
    fn model() -> Model {
        Model { text: "".into()}
    }

    fn update(&mut self, event: Msg) {
        match event {
            Pressed => {}
        }
    }

    view! {
        #[name="button"]
        gtk::Button {
            label: "Info"
        }
    }
}
