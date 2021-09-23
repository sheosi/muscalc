
use crate::calcs::{Activity, AthleteKind, Goal, GoalIntensity, MuscleTrainingKind, Sex, Weight};
use crate::consts::gui::*;

use relm::Widget;
use gtk::{Orientation::Vertical, prelude::*};
use relm::{Sender, Relm};
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
                    value_changed(scale) => Msg::ValChanged(scale.value() as u8),
                    hexpand: true
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
    can_calc: bool,
    fat_has_content: bool,
    height_has_content: bool,
    sender: Option<Sender<CalcData>>,
    list_act_kind: gtk::ListStore,
    
    activity: Activity,
    goal_intensity: GoalIntensity,
}

pub struct CalcData {
    pub weight: Weight,
    pub age: u8,
    pub height: Option<f32>,
    pub activity: Activity,
    pub sex: Sex,
    pub goal: Goal,
    pub training_kind: Option<MuscleTrainingKind>,
    pub is_bodybuilder: bool,
    pub athlete_kind: Option<AthleteKind>
}

#[derive(Msg)]
pub enum Msg {
    ShowResults,
    WeightChanged,
    FatChanged,
    AgeChanged,
    HeightChanged,

    ActivityChanged(u8),
    GoalIntensityChanged(u8),

    ReceiveSender(Sender<CalcData>)
}

use myscale::Wdg as MyScale;
use myscale::Msg::ValChanged as MyScaleValChanged;

fn only_digits() {
    // TODO!!
}

#[widget]    
impl Widget for Wdg {
    fn model(_: &Relm<Self>, _: ()) -> Model {
        let list_act_kind = gtk::ListStore::new(&[glib::Type::STRING]);

        const ACT_KINDS : [&str; 3] = ["Resistance/Cardio", "Strength", "Bodybuilding"];

        for act_kind in ACT_KINDS {
            list_act_kind.set(&list_act_kind.append(), &[(0,&act_kind.to_string())]);
        }

        
        
        Model {
            can_calc: false,
            fat_has_content: false,
            height_has_content: false,
            sender: None,
            list_act_kind,

            activity: Activity::Light,
            goal_intensity: GoalIntensity::Light,
        }
    }

    fn update(&mut self, event: Msg) {
        fn required(entry: &gtk::Entry) {
            if entry.text() == "" {
                entry.style_context().add_class("error");
            }
            else {
                entry.style_context().remove_class("error");
            }
        }

        fn required_spin(entry: &gtk::SpinButton) {
            if entry.text() == "" {
                entry.style_context().add_class("error");
            }
            else {
                entry.style_context().remove_class("error");
            }
        }

        fn can_calc(wdg: &Wdg) -> bool {
            let has_weight = wdg.widgets.e_weight.text() != "";
            let has_age = wdg.widgets.e_age.text() != "";

            has_weight && has_age && (wdg.model.fat_has_content || wdg.model.height_has_content)
        }
        match event {
            ShowResults=> {
                let weight = Weight::new(
                    self.widgets.e_weight.text().parse::<f32>().unwrap(),
                    self.widgets.e_fat.text().parse::<f32>().ok()
                );

                let intensity = self.model.goal_intensity.clone();
                let goal = 
                    if self.widgets.r_goal_none.is_active(){
                        Goal::None
                    }
                    else if self.widgets.r_goal_down.is_active() {
                        Goal::WeightLoss(intensity)
                    }
                    else {
                        Goal::WeightGain(intensity)
                    };


                let sex = 
                    if self.widgets.r_man.is_active() {Sex::Male}
                    else {Sex::Female};

                const RESISTENCE_COMBO_ID: u32= 0;
                const STRENGTH_COMBO_ID: u32 = 1;
                const BODYBUILDR_COMBO_ID:u32 = 2;
                
                let training_num = self.widgets.c_activity_kind.active();
                let is_bodybuilder =
                    training_num.map(|id|id==BODYBUILDR_COMBO_ID).unwrap_or(false);
                
                let training_kind = training_num.map(|n|match n{
                    RESISTENCE_COMBO_ID => MuscleTrainingKind::Resistance,
                    STRENGTH_COMBO_ID|BODYBUILDR_COMBO_ID => MuscleTrainingKind::Strength,
                    _ => panic!("Unknown training kind")
                });

                let activity = self.model.activity.clone();

                let athlete_kind = if self.widgets.c_athlete.is_active(){
                    match activity {
                        Activity::Moderate => Some(AthleteKind::KindaActive),
                        Activity::Vigorous => Some(AthleteKind::VeryActive),
                        Activity::Extreme => Some(AthleteKind::IntenseActivity),
                        _ => None
                    }
                }else {None};

                let data = CalcData {
                    weight,
                    age: self.widgets.e_age.text().parse().unwrap(),
                    height: self.widgets.e_height.text().parse().ok(),
                    activity,
                    sex,
                    goal,
                    is_bodybuilder,
                    training_kind,
                    athlete_kind
                };
                self.model.sender.as_mut().expect("No sender available").send(data).expect("Channel send had an error");
            },

            WeightChanged=> {
                required(&self.widgets.e_weight);
                self.model.can_calc = can_calc(&self);
            },

            FatChanged => {
                required(&self.widgets.e_fat);
                self.model.fat_has_content = self.widgets.e_fat.text() != "";
                self.model.can_calc = can_calc(&self);
            },

            AgeChanged => {
                required_spin(&self.widgets.e_age);
                self.model.can_calc = can_calc(&self);
            },

            HeightChanged => {
                required(&self.widgets.e_height);
                self.model.height_has_content = self.widgets.e_height.text() != "";
                self.model.can_calc = can_calc(&self);
            },

            ActivityChanged(id) => {
                self.model.activity = Activity::from_int(id);
            }

            GoalIntensityChanged(id) => {
                self.model.goal_intensity = GoalIntensity::from_int(id);
            }

            ReceiveSender(s) => {
                self.model.sender = Some(s);
            }
        }
    }

    view! {
        #[name="grid"]
        gtk::Grid {
            orientation: Vertical,
            column_spacing: 14,
            row_spacing: 19,
            border_width: 15,

            // Weight
            gtk::Label {
                text: "Weight",
                cell: {
                    top_attach: 0,
                    left_attach: 0,
                }
            },

            #[name="e_weight"]
            #[style_class="error"]
            gtk::Entry {
                placeholder_text: Some("Kg"),
                changed => Msg::WeightChanged,
                cell: {
                    top_attach: 0,
                    left_attach: 1,
                    width: 5
                },
                hexpand: true
            },

            // Age
            gtk::Label {
                text: "Age",
                cell: {
                    top_attach: 1,
                    left_attach: 0,
                }
            },

            #[name="e_age"]
            #[style_class="error"]
            gtk::SpinButton {
                placeholder_text: Some("years"),
                changed => Msg::AgeChanged,
                cell: {
                    top_attach: 1,
                    left_attach: 1,
                    width: 2
                }
            },

            // Fat
            gtk::Label {
                text: "Fat %",
                sensitive: !self.model.height_has_content,
                cell: {
                    top_attach: 2,
                    left_attach: 0
                }
            },

            #[name="e_fat"]
            #[style_class="error"]
            gtk::Entry {
                placeholder_text: Some("% (Optional)"),
                sensitive: !self.model.height_has_content,
                changed => Msg::FatChanged,
                cell: {
                    top_attach: 2,
                    left_attach: 1
                }
            },

            // Separator
            gtk::Separator {
                orientation: Vertical,
                cell: {
                    top_attach: 2,
                    left_attach: 2
                }
            },
            // Height

            gtk::Label {
                text: "Height",
                sensitive: !self.model.fat_has_content,
                cell: {
                    top_attach: 2,
                    left_attach: 3,
                }
            },

            #[name="e_height"]
            #[style_class="error"]
            gtk::Entry {
                placeholder_text: Some("meters"),
                sensitive: !self.model.fat_has_content,
                changed => Msg::HeightChanged,
                cell: {
                    top_attach: 2,
                    left_attach: 4,
                    width: 2
                }
            },

            // Sex
            gtk::Label {
                text: "Sex",
                cell: {
                    top_attach: 3,
                    left_attach: 0
                }
            },
            gtk::Box {
                border_width: RADIO_GROUPS_BORDER,
                spacing: RADIO_GROUPS_SPACING,

                #[name="r_man"]
                gtk::RadioButton {
                    label: "♂",
                },
                gtk::RadioButton({group: r_man}) {
                    label: "♀",
                },
                cell: {
                    top_attach: 3,
                    left_attach: 1
                }
            },
            

            // Goal
            gtk::Label {
                text: "Goal",
                cell: {
                    top_attach: 4,
                    left_attach: 0
                }
            },

            gtk::Box {
                border_width: RADIO_GROUPS_BORDER,
                spacing: RADIO_GROUPS_SPACING,

                #[name="r_goal_none"]
                gtk::RadioButton {
                    label: "∅",
                    
                },

                gtk::RadioButton({group: r_goal_none}) {
                    label: "↑",
                },
                
                #[name="r_goal_down"]
                gtk::RadioButton({group: r_goal_none}) {
                    label: "↓"
                },

                cell: {
                    top_attach: 4,
                    left_attach: 1
                }
            },

            #[name="s_goal_intensity"]
            MyScale() {
                MyScaleValChanged(n) => Msg::GoalIntensityChanged(n),
                cell: {
                    top_attach: 4,
                    left_attach: 4,
                    width: 2
                }
            },
            

            // Activity
            gtk::Label {
                text: "Activity",
                cell: {
                    top_attach: 5,
                    left_attach: 0
                }
            },

            #[name="s_activity"]
            MyScale()  {
                MyScaleValChanged(n) => Msg::ActivityChanged(n),
                cell: {
                    top_attach: 5,
                    left_attach: 1,
                    width: 3
                }
            },

            gtk::Label {
                text: "Kind",
                cell: {
                    top_attach: 5,
                    left_attach: 4
                }
            },

            #[name="c_activity_kind"]
            gtk::ComboBox {
                model: Some(&self.model.list_act_kind),
                cell: {
                    top_attach: 5,
                    left_attach: 5
                }
            },
            
            #[name="c_athlete"]
            gtk::CheckButton {
                label: "Athlete",
                cell: {
                    top_attach: 5,
                    left_attach: 6
                }
            },

            // Main button
            #[style_class="suggested-action"]
            #[style_class="linked"]
            gtk::Button {
                label: "Calc",
                clicked => {
                    Msg::ShowResults
                },
                sensitive: self.model.can_calc,
                cell: {
                    top_attach: 7,
                    left_attach: 5,
                }
            },
        }
    }

    fn init_view(&mut self) {
    }
}