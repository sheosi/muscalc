use relm::Widget;
use gtk::{prelude::*};
use relm::{Component, init};
use relm_derive::{Msg, widget};

use Msg::*;

#[derive(Msg)]
pub enum Msg {
    Pressed,
    UpdateText(Option<String>)
}

pub struct Model {
    popover: Component<info_popover::Wdg>
}

#[widget]
impl Widget for Wdg {
    fn model() -> Model {
        let popover = init::<info_popover::Wdg>(()).expect("Popover");
        Model {popover}
    }

    fn update(&mut self, event: Msg) {
        match event {
            Pressed => {
                self.model.popover.stream().emit(info_popover::Msg::Show);
            },
            UpdateText(text) => {
                if let Some(text) = text {
                    self.widgets.button.show();
                    self.model.popover.stream().emit(info_popover::Msg::UpdateText(text));
                }
                else {
                    self.widgets.button.hide();
                }
            }
            
            
        }
    }

    view! {
        #[name="button"]
        gtk::Button {
            image: Some(&gtk::Image::from_icon_name(Some("user-info-symbolic"), gtk::IconSize::Button)),
            //label: "Info",
            clicked => Pressed
        }
    }

    fn init_view(&mut self) {
        self.model.popover.stream().emit(info_popover::Msg::SetRelativeTo(self.widgets.button.clone()));
    }
}

mod info_popover {
    use relm::Widget;
    use gtk::{prelude::*};
    use relm_derive::{Msg, widget};

    use Msg::*;

    #[derive(Msg)]
    pub enum Msg {
        UpdateText(String),
        SetRelativeTo(gtk::Button),
        Show
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
                UpdateText(text) => {
                    self.model.text = text;
                }
                SetRelativeTo(relative_to) => {
                    self.widgets.popover.set_relative_to(Some(&relative_to));
                }
                Show => {
                    self.widgets.popover.show();
                }                
            }
        }

        view! {
            #[name="popover"]
            gtk::Popover {
                gtk::Label {
                    text: &self.model.text
                },
                visible: false
            }
        }
    }
}
