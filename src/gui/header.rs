use relm::Widget;
use relm_derive::{Msg, widget};
use gtk::prelude::*;

use Msg::*;


#[derive(Msg)]
pub enum Msg {
    OnBack,
    EnableBack
}

#[widget]
impl Widget for Wdg {
    fn model() -> () {

    }

    fn update(&mut self, event: Msg) {
        match event {
            OnBack => self.widgets.b_back.set_sensitive(false),
            EnableBack => self.widgets.b_back.set_sensitive(true),
        }
    }

    view! {
        #[name="titlebar"]
        gtk::HeaderBar {
            title: Some("BodyCalc"),
            show_close_button: true,

            #[name="b_back"]
            gtk::Button {
                label: "⟵  Back",
                sensitive: false,
                clicked => OnBack
            },
        }
    }
}
