
use relm::Widget;
use gtk::{Orientation::Vertical, prelude::*};
use relm::{connect, Component, init, Relm};
use relm_derive::{Msg, widget};

use Msg::*;

mod myscale {
    use relm::{Relm, Widget};
    use relm_derive::{Msg, widget};
    use gtk::prelude::*;
    use gtk::Adjustment;

    use Msg::*;

    #[derive(Msg)]
    pub enum Msg {
        ValChanged(u8),
        
    }

    #[widget]
    impl Widget for Wdg {
        fn model(_: &Relm<Self>, _: ()) -> () {}
    
        fn update(&mut self, event: Msg) {
            match event {
                ValChanged(new_val) => self.widgets.label.set_text(&format!("{}", new_val)),
            }
        }

        view! {
            #[name="titlebar"]
            gtk::Grid {
                #[name="scale"]
                gtk::Scale {
                    adjustment: &Adjustment::new(10.0, 0.0, 100.0, 1.0, 10.0, 0.0),
                    value_changed(scale) => Msg::ValChanged(scale.value() as u8)
                },
                
                #[name="label"]
                gtk::Label {
                    text: "0"
                }
            }
        }

        fn init_view(&mut self) {
            //self.widgets.scale.attach(self.model.s_goal_intensity.widget(), 4, 5, 1,1);
        }
    }
}

pub struct Model {
    s_goal_intensity: Component<myscale::Wdg>,
    s_activity: Component<myscale::Wdg>
}

#[widget]    
impl Widget for Wdg {
    fn model(_: &Relm<Self>, _: ()) -> Model {
        let s_goal_intensity = init::<myscale::Wdg>(()).expect("Goal Intensity");
        let s_activity = init::<myscale::Wdg>(()).expect("Activity");

        Model {
            s_goal_intensity,
            s_activity
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            // A call to self.label1.set_text() is automatically inserted by the
            // attribute every time the model.counter attribute is updated.
            /*Msg::Decrement => self.model.counter -= 1,
            Msg::Increment => self.model.counter += 1,*/
            ShowResults=> {
            }
        }
    }

    view! {
        #[name="grid"]
        gtk::Grid {
            orientation: Vertical,

            // Weight
            gtk::Label {
                text: "Weight",
                cell: {
                    top_attach: 0,
                    left_attach: 0,
                }
            },

            gtk::Entry {
                placeholder_text: Some("Kg"),
                cell: {
                    top_attach: 0,
                    left_attach: 1,
                }
            },

            // Fat
            gtk::Label {
                text: "Fat %",
                cell: {
                    top_attach: 1,
                    left_attach: 0,
                }
            },

            gtk::Entry {
                placeholder_text: Some("% (Optional)"),
                cell: {
                    top_attach: 1,
                    left_attach: 1,
                }
            },

            // Height

            gtk::Label {
                text: "Height",
                cell: {
                    top_attach: 2,
                    left_attach: 0,
                }
            },

            gtk::Entry {
                placeholder_text: Some("meters"),
                cell: {
                    top_attach: 2,
                    left_attach: 1,
                }
            },

            // Age
            gtk::Label {
                text: "Age",
                cell: {
                    top_attach: 3,
                    left_attach: 0,
                }
            },

            gtk::Entry {
                placeholder_text: Some("years"),
                cell: {
                    top_attach: 3,
                    left_attach: 1,
                }
            },

            // Sex
            gtk::Label {
                text: "Sex",
                cell: {
                    top_attach: 4,
                    left_attach: 0
                }
            },

            #[name="r_man"]
            gtk::RadioButton {
                label: "Man",
                cell: {
                    top_attach: 4,
                    left_attach: 1
                }
            },
            gtk::RadioButton({group: r_man}) {
                label: "Woman",
                cell: {
                    top_attach: 4,
                    left_attach: 2
                }
            },

            // Goal
            gtk::Label {
                text: "Goal",
                cell: {
                    top_attach: 5,
                    left_attach: 0
                }
            },

            #[name="r_goal_none"]
            gtk::RadioButton {
                label: "None",
                cell: {
                    top_attach: 5,
                    left_attach: 1
                }
            },

            gtk::RadioButton({group: r_goal_none}) {
                label: "↑",
                cell: {
                    top_attach: 5,
                    left_attach: 2
                }
            },
            
            gtk::RadioButton({group: r_goal_none}) {
                label: "↓",
                cell: {
                    top_attach: 5,
                    left_attach: 3
                }
            },

            // Activity
            gtk::Label {
                text: "Activity",
                cell: {
                    top_attach: 6,
                    left_attach: 0
                }
            },

            // Main button
            #[style_class="suggested-action"]
            #[style_class="linked"]
            gtk::Button {
                label: "Calc",
                clicked => Msg::ShowResults,
                cell: {
                    top_attach: 7,
                    left_attach: 0,
                }
            },
        }
    }

    fn init_view(&mut self) {
        self.widgets.grid.attach(self.model.s_goal_intensity.widget(), 4, 5, 1,1);
        self.widgets.grid.attach(self.model.s_activity.widget(), 1,6,1,1);
    }
}

#[derive(Msg)]
pub enum Msg {
    ShowResults
}
