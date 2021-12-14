
use crate::gui::entry_page::CalcData;

use relm::Widget;
use gtk::prelude::*;
use relm::Relm;
use relm_derive::{Msg, widget};

use crate::calcs::{self, EnumToString};

use super::info_button::{Wdg as InfoButton, Msg::UpdateText as InfoButtonUpdateText};


pub struct Model {
    calories: (f32, f32),
    proteins: (f32, f32),
    fat: (f32, f32),
    carbs: (f32, f32),
}

#[derive(Msg)]
pub enum Msg {
    Update(CalcData)
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
    fn update(&mut self, msg: Msg) {
        fn make_msg<E: EnumToString>(infos: Vec<E>) -> Option<String> {
            if !infos.is_empty() {
                let mut res = String::new();
                let (last, all_msg) = infos.split_last().unwrap();
                for i in all_msg {
                    if infos.len() > 1 {
                        res.push_str("* ");
                    }

                    res.push_str(&i.to_string());
                    res.push_str("\n\n");
                }

                if infos.len() > 1 {
                    res.push_str("* ");
                }
                res.push_str(&last.to_string());

                Some(res)
            } else {
                None
            }
        }
        match msg {
            Msg::Update(data) => {                
                {
                    let (base_cals, problems_cals) = calcs::base_calories(&data.weight, data.height, data.age, &data.sex);
                    let (min_cals, max_cals) = calcs::target_calories(base_cals, &data.activity, &data.goal);
                    self.model.calories = (min_cals.round(), max_cals.round());
                    self.components.i_calories.stream().emit(InfoButtonUpdateText(make_msg(problems_cals)));
                }

                {
                    let (min_fat, max_fat, problems_fat) = calcs::fat(&data.weight, &data.goal);
                    self.model.fat = (min_fat.round(), max_fat.round());
                    self.components.i_fat.stream().emit(InfoButtonUpdateText(make_msg(problems_fat)));
                }

                {
                    let (min_proteins, max_proteins, problems_proteins) = 
                        calcs::proteins(&data.weight, data.age, 
                            &data.training_kind, &data.goal, &data.sex,
                        &data.activity, data.is_bodybuilder);

                    self.model.proteins = (min_proteins.round(), max_proteins.round());
                    self.components.i_proteins.stream().emit(InfoButtonUpdateText(make_msg(problems_proteins)));
                }
                
                {
                    let (min_carbs, max_carbs) = calcs::carbs(
                        self.model.calories,
                        self.model.proteins,
                        self.model.fat, data.weight.total, data.athlete_kind);
                    self.model.carbs = (min_carbs.round(), max_carbs.round());
                }
            }
        }
    }

    view! {
        gtk::Grid {
            column_spacing: 10,
            row_spacing: 10,
            border_width: 10,
            
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
                    left_attach: 2,
                }
            },

            gtk::Label {
                text:  "⭱",
                cell: {
                    top_attach: 0,
                    left_attach: 3,
                }
            },

            gtk::Label {
                text: &self.model.calories.1.to_string(),
                cell: {
                    top_attach: 0,
                    left_attach: 4,
                }
            },

            #[name="i_calories"]
            InfoButton() {
                visible: false,
                cell: {
                    top_attach: 0,
                    left_attach: 5,
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

            #[name="i_proteins"]
            InfoButton() {
                visible: false,
                cell: {
                    top_attach: 1,
                    left_attach: 5,
                }
            },

            // Fat

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

            #[name="i_fat"]
            InfoButton() {
                visible: false,
                cell: {
                    top_attach: 2,
                    left_attach: 5,
                }
            },

            // Carbohidrates

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
                text: &self.model.carbs.0.to_string(),
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
                text: &self.model.carbs.1.to_string(),
                cell: {
                    top_attach: 3,
                    left_attach: 4,
                }
            }
        }
    }
}
