use relm::Widget;
use gtk::{prelude::*};
use relm_derive::{Msg, widget};

use Msg::*;

// TODO: Make info buttons work

#[derive(Msg)]
pub enum Msg {
    Pressed,
    UpdateText(Option<String>)
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
            Pressed => {},
            UpdateText(text) => {
                if let Some(text) = text {
                    self.model.text = text;
                    self.widgets.button.set_visible(true);
                }
            
            }
        }
    }

    view! {
        #[name="button"]
        gtk::Button {
            label: "Info"
        }
    }
}
