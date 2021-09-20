use relm::Widget;
use gtk::prelude::*;
use relm::Relm;
use relm_derive::widget;

pub struct Model {
    calories: (f32, f32),
    proteins: (f32, f32),
    fat: (f32, f32),
    carbs: (f32, f32),
}

#[widget]
impl Widget for Wdg {
    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
            calories: (0.0, 0.0),
            proteins: (0.0, 0.0),
            fat: (0.0, 0.0),
            carbs: (0.0, 0.0),
        }
    }
    fn update(&mut self, _: ()) {}

    view! {
        gtk::Grid {
            gtk::Label {
                text:  "Calories",
                cell: {
                    top_attach: 0,
                    left_attach: 0,
                }
            },

            gtk::Label {
                text: "⭳",
                cell: {
                    top_attach: 0,
                    left_attach: 1,
                }
            },

            gtk::Label {
                text:  &self.model.calories.0.to_string(),
                cell: {
                    top_attach: 0,
                    left_attach: 1,
                }
            },

            gtk::Label {
                text:  "⭱",
                cell: {
                    top_attach: 0,
                    left_attach: 2,
                }
            },

            gtk::Label {
                text: &self.model.calories.1.to_string(),
                cell: {
                    top_attach: 0,
                    left_attach: 3,
                }
            },

            // Proteins
            gtk::Label {
                text:  "Proteins",
                cell: {
                    top_attach: 1,
                    left_attach: 0,
                }
            },

            gtk::Label {
                text: "⭳",
                cell: {
                    top_attach: 1,
                    left_attach: 1,
                }
            },

            gtk::Label {
                text: &self.model.proteins.0.to_string(),
                cell: {
                    top_attach: 1,
                    left_attach: 2,
                }
            },

            gtk::Label {
                text: "⭱",
                cell: {
                    top_attach: 1,
                    left_attach: 3,
                }
            },

            gtk::Label {
                text: &self.model.proteins.1.to_string(),
                cell: {
                    top_attach: 1,
                    left_attach: 4,
                }
            },

            gtk::Label {
                text: "Fat",
                cell: {
                    top_attach: 2,
                    left_attach: 0,
                }
            },

            gtk::Label {
                text: "⭳",
                cell: {
                    top_attach: 2,
                    left_attach: 1,
                }
            },

            gtk::Label {
                text: &self.model.fat.0.to_string(),
                cell: {
                    top_attach: 2,
                    left_attach: 2,
                }
            },

            gtk::Label {
                text: "⭱",
                cell: {
                    top_attach: 2,
                    left_attach: 3,
                }
            },

            gtk::Label {
                text: &self.model.fat.1.to_string(),
                cell: {
                    top_attach: 2,
                    left_attach: 4,
                }
            },

            gtk::Label {
                text: "Carbohidrates",
                cell: {
                    top_attach: 3,
                    left_attach: 0,
                }
            },

            gtk::Label {
                text: "⭳",
                cell: {
                    top_attach: 3,
                    left_attach: 1,
                }
            },

            gtk::Label {
                text: &self.model.calories.0.to_string(),
                cell: {
                    top_attach: 3,
                    left_attach: 2,
                }
            },

            gtk::Label {
                text: "⭱",
                cell: {
                    top_attach: 3,
                    left_attach: 3,
                }
            },

            gtk::Label {
                text: &self.model.calories.1.to_string(),
                cell: {
                    top_attach: 3,
                    left_attach: 4,
                }
            },

        }
    }
}
