use crate::consts::{msgs::*, nums::*};

/***** Pub interface *******************************************************/
pub trait EnumToString {
    fn to_string(&self) -> String;
}

pub trait EnumIter where Self:Sized {
    fn iter_all() -> &'static[Self];
}

pub trait EnumIterString {
    fn vec_string() -> Vec<String>;
}

impl<T: 'static> EnumIterString for T where T:EnumIter + EnumToString {
    fn vec_string() -> Vec<String> {
        let i= Self::iter_all();
        i.iter().map(|e| e.to_string()).collect()
    }
}

pub fn base_calories(weight: &Weight, height: Option<f32>, age: u8, sex: &Sex ) -> (f32, Vec<CaloriesSpecialCases>) {
    if let Some(fat) = weight.fat_percent {
        (katch_mcardle(weight.total, fat), Vec::new())
    }
    else if let Some(height) = height { 
        (mifflin_st_jeor(weight.total, height, age, sex), vec![CaloriesSpecialCases::MifflinFormula])
    }
    else {
        panic!("Either weight.fat_percent or height must be some")
    }
}

pub fn target_calories(base_cals: f32, activity: &Activity, goal: &Goal) -> (f32, f32) {
    let (min, max) = activity.adjust();
    let goal_adj = goal.adjust();
    (min * base_cals * goal_adj, max * base_cals * goal_adj)
}

pub fn carbs(
    total_cals: (f32, f32),
    proteins: (f32, f32),
    fat: (f32, f32),
    weight_total: f32,
    athlete_kind: Option<AthleteKind> ) -> (f32, f32) {
    if let Some(kind) = athlete_kind {
        carbs_athlete(weight_total, kind)
    }
    else {
        carbs_normal(total_cals, proteins, fat)
    }
}

pub fn proteins(
    weight: &Weight,
    age: u8,
    training: &Option<MuscleTrainingKind>,
    goal: &Goal,
    sex: &Sex,
    activity: &Activity,
    is_bodybuilder: bool
    ) -> (f32, f32, Vec<ProteinSpecialCases>) {
    fn is_teen(age: u8) -> bool {
        age <= TEEN_MAX_AGE
    }

    fn is_goal_very_low_carbs(goal: &Goal) -> bool {
        fn is_intensity_moderate(intensity: &GoalIntensity) -> bool {
            matches!(intensity, GoalIntensity::Extreme | GoalIntensity::High)
        }
        
        if let Goal::WeightLoss(intensity) = goal {is_intensity_moderate(intensity)}
        else {false}
    }

    fn is_really_lean(weight: &Weight, sex: &Sex) -> bool {
        let min_fat = match sex {
            Sex::Male => REALLY_LEAN_MALE, 
            Sex::Female => REALLY_LEAN_FEMALE
        };

        weight.is_fat_percent_higher(min_fat)
    }

    fn is_activity_high(activity: &Activity) -> bool {
        matches!(activity, Activity::Extreme | Activity::Vigorous)
    }

    fn is_sedentary(activity: &Activity) -> bool {
        activity == &Activity::Sedentary
    }

    fn calc_for_weight(weight: &Weight, for_total: f32, for_lean: f32) -> f32 {
        if let Some(lean) = weight.lean {
            for_lean * lean
        }
        else {for_total * weight.total}
    }

    if is_bodybuilder {
        let (min, max) = proteins_bodybuilder(weight);
        (min, max, vec![])
    }

    else if is_teen(age) {
        (weight.total * 1.8, weight.total * 2.0, vec![ProteinSpecialCases::Teen])
    }

    else if is_goal_very_low_carbs(goal) || is_activity_high(activity) {
        if is_really_lean(weight, sex) {
            let lean = weight.lean.unwrap();
            (lean * 2.5, lean * 4.0, vec![ProteinSpecialCases::ReallyLean])
        }
        else {
            let special_case: ProteinSpecialCases = {
                if is_goal_very_low_carbs(goal) {ProteinSpecialCases::LowCarbs}
                else {ProteinSpecialCases::HighActivity}
            };
            (weight.total * 2.5, weight.total * 3.0, vec![special_case, ProteinSpecialCases::GeneralCase(proteins_bodybuilder(weight))])
        }
    }

    else if is_sedentary(activity) {
        (
            calc_for_weight(weight,  1.5, 2.0),
            calc_for_weight(weight,  2.0, 2.0),
            vec![ProteinSpecialCases::Sedentary, ProteinSpecialCases::GeneralCase(proteins_bodybuilder(weight))]
        )
    }
    else if weight.is_fat_percent_higher(HIGHER_FAT_THRESHOLD) {
        (
            calc_for_weight(weight, 1.5,  2.0),
            calc_for_weight(weight, 2.0,  2.0),
            vec![ProteinSpecialCases::HighFat, ProteinSpecialCases::GeneralCase(proteins_bodybuilder(weight))]
        )
    }

    else {
        match training {
            Some(MuscleTrainingKind::Strength) =>  (weight.total * 1.4, weight.total * 2.0, vec![]),
            Some(MuscleTrainingKind::Resistance) =>  (weight.total * 1.2, weight.total * 1.8, vec![]),
            None => {
                // For the most general case, the best recommendation is actually
                // having a dose like a bodybuilder, given that proteins are important
                // and help with common things like feeling full or sugar control
                let (min, max) = proteins_bodybuilder(weight);
                (min, max, vec![])
            }
        }
    }
}

/***** Enums ******************************************************************/

pub struct Weight {
    pub total: f32,
    fat_percent: Option<f32>,
    lean: Option<f32>
}
impl Weight {
    pub fn new(total: f32, fat_percent: Option<f32>) -> Self {
        Self {
            total,
            fat_percent,
            lean: fat_percent.map(|f_percent|total - (total * f_percent/100.0))
        }
    }
    

    fn is_fat_percent_higher(&self, percent: f32) -> bool {
        if let Some(fat) = self.fat_percent {
            fat >= percent
        }
        else {false}
    }
}


pub enum Sex {
    Male,
    Female
}

#[derive(Clone, PartialEq)]
pub enum Activity {
    Sedentary, // No activity, office work
    Light,     // Little daily activity, exercise 1-3 times/week
    Moderate,  // Moderate daily activity, exercise 3-5 times/week
    Vigorous,  // Vigorous daily activity, exercise 6-7 times/week
    Extreme   // Intense daily workout, tiring physical job
}

impl Activity {
    pub fn from_int(i:u8) -> Self {
        match i {
            0 => Self::Sedentary,
            1 => Self::Light,
            2 => Self::Moderate,
            3 => Self::Vigorous,
            4 => Self::Extreme,
            _ => panic!("Invalid activity value")
        }
    }
    fn adjust(&self) -> (f32, f32) {
        use Activity::*;
        match self {
            Sedentary => (1.2, 1.2),
            Light => (1.3, 1.4) ,
            Moderate => (1.5, 1.6),
            Vigorous => (1.7, 1.8),
            Extreme => (1.9, 2.0)
        }
    }
}

impl Default for Activity {
    fn default() -> Self {
        Activity::Sedentary
    }
}

impl EnumIter for Activity {
    fn iter_all() -> &'static [Self] {
        &[
            Self::Sedentary, Self::Light, Self::Moderate, Self::Vigorous,
            Self::Extreme
        ]
    }
}

impl EnumToString for Activity {
    fn to_string(&self) -> String {
        use Activity::*;
        match self {
            Sedentary => "Sedentary".to_string(),
            Light => "1-3 times".to_string(),
            Moderate => "3-5 times".to_string(),
            Vigorous => "6-7 times".to_string(),
            Extreme => "Extreme".to_string()
        }
    }
}

#[derive(Clone, Debug)]
pub enum GoalIntensity{
    Light,    // 10%
    Moderate, // 15%
    High,     // 20%
    Extreme   // 30%
}

impl GoalIntensity {
    pub fn from_int(i: u8) -> GoalIntensity {
        match i {
            0=> GoalIntensity::Light,
            1=> GoalIntensity::Moderate,
            2=> GoalIntensity::High,
            3=> GoalIntensity::Extreme,
            _ => panic!("Invalid intensity")
        }
    }
}

impl Default for GoalIntensity {
    fn default() -> Self {
        GoalIntensity::Light
    }
}

impl EnumIter for GoalIntensity {
    fn iter_all() -> &'static [Self] {
        &[
            GoalIntensity::Light, GoalIntensity::Moderate, GoalIntensity::High,
            GoalIntensity::Extreme
        ]
    }
}

impl EnumToString for GoalIntensity {
    fn to_string(&self) -> String {
        use GoalIntensity::*;
        match self {
            Light => "Light".to_string(),
            Moderate => "Moderate".to_string(),
            High => "High".to_string(),
            Extreme => "Extreme".to_string()
        }
    }
}

pub enum Goal {
    None,
    WeightLoss(GoalIntensity),
    WeightGain(GoalIntensity)
}

impl Goal {
    fn adjust(&self) -> f32 {
        fn intensity_adj(intensity: &GoalIntensity) -> f32 {
            use GoalIntensity::*;
            match intensity {
                Moderate => 0.15,
                Light => 0.1,
                High => 0.2,
                Extreme => 0.3
            }
        }

        match self {
            Goal::None => 1.0,
            Goal::WeightLoss(intensity) => 1.0 - intensity_adj(intensity),
            Goal::WeightGain(intensity) => 1.0 + intensity_adj(intensity)
        }
    }

}

pub enum MuscleTrainingKind {
    Strength,
    Resistance   
}

pub enum AthleteKind {
    KindaActive,
    VeryActive,
    IntenseActivity,
}

/***** Calories ***************************************************************/

pub enum CaloriesSpecialCases {
    MifflinFormula
}

impl EnumToString for CaloriesSpecialCases {
    fn to_string(&self) -> String {
        use CaloriesSpecialCases::*;
        match self {
            MifflinFormula => MIFFLIN_FORMULA.to_string()
        }
    }
}


fn mifflin_st_jeor(weight: f32, height: f32, age: u8, sex: &Sex) -> f32 {
    let sex_adj: f32 = match sex {
        Sex::Male => 5.0,
        Sex::Female => -161.0
        
    };

    (9.99 * weight) + (6.25 * height) - (4.92 * (age as f32)) + sex_adj
}

fn katch_mcardle(weight: f32, fat_percent: f32) -> f32 {
    let imcm = (weight * (100.0 - fat_percent)) / 100.0;
    370.0 + (21.6 * imcm)
}
/***** Proteins ***************************************************************/

pub enum ProteinSpecialCases {
    Teen,
    LowCarbs,
    HighActivity,
    ReallyLean, // So lean, needs extra protein
    Sedentary, // Can ingest low volume of protein
    HighFat, // Can ingest low volume of protein
    GeneralCase((f32, f32)) // In general, is good to eat as much protein as a body builder
}

impl EnumToString for ProteinSpecialCases {
    fn to_string(&self) -> String {
        use ProteinSpecialCases::*;
        match self {
            Teen=> PROTEIN_TEEN.to_string(),
            LowCarbs => LOW_CARBS.to_string(),
            HighActivity => HIGH_ACTIVITY.to_string(),
            ReallyLean => REALLY_LEAN.to_string(),
            Sedentary => SEDENTARY.to_string(),
            HighFat => HIGH_FAT.to_string(),
            GeneralCase((std_min, std_max)) => general_case(*std_min,*std_max)
        }
    }
}

fn proteins_bodybuilder(weight: &Weight) -> (f32, f32) {
    if let Some(lean) = weight.lean {
        (lean * 2.0, lean * 3.0)
    }
    else {
        (weight.total * 2.0, weight.total * 2.5)
    }
}

/***** Fat ********************************************************************/

pub enum FatSpecialCases {
    LowCarbsDiet((f32, f32))
}

impl EnumToString for FatSpecialCases {
    fn to_string(&self) -> String {
        use FatSpecialCases::*;
        match self {
            LowCarbsDiet((min, max)) => low_carbs_diet_fat(*min, *max)
        }
    }
}

pub fn fat(weight: &Weight, goal: &Goal ) -> (f32, f32, Vec<FatSpecialCases>) {
    fn is_goal_low_carbs(goal: &Goal) -> bool {
        fn is_intensity_moderate(intensity: &GoalIntensity) -> bool {
            matches!(intensity,  GoalIntensity::Moderate | GoalIntensity::High | GoalIntensity::Extreme)
        }
        
        if let Goal::WeightLoss(intensity) = goal {is_intensity_moderate(intensity)}
        else {false}
    }

    if is_goal_low_carbs(goal) {
        let normal_case = fat_normal_case(weight);
        (weight.total * 0.66, normal_case.1, vec![FatSpecialCases::LowCarbsDiet(normal_case)])
    }
    else {
        let (min, max) = fat_normal_case(weight);
        (min, max, vec![])
    }
}

fn fat_normal_case(weight: &Weight) -> (f32, f32) {
    if weight.is_fat_percent_higher(HIGH_FAT_THRESHOLD) {
        let lean = weight.lean.unwrap();
        (lean * 1.0, lean * 2.0)
    }
    else {
        (weight.total * 1.0, weight.total * 2.0)
    }
}


/***** Carbs ******************************************************************/

fn carbs_normal((min_cals, max_cals): (f32, f32), (min_pro, max_pro): (f32, f32), (min_fat, max_fat): (f32, f32)) -> (f32, f32) {
    fn to_g(c: f32) -> f32 {
        c / 4.0
    }

    // We calculate the carbohidrates as the remnant towards our calories,
    // so we get calories and transform that to grams.
    (
        to_g(max_cals - (max_pro * 4.0 + max_fat * 9.0)),
        to_g(min_cals - (min_pro * 4.0 + min_fat * 9.0))
    )
    
}

fn carbs_athlete(weight_total: f32, kind: AthleteKind) -> (f32, f32) {
    match kind {
        AthleteKind::KindaActive => (weight_total * 4.5, weight_total * 6.5),
        AthleteKind::VeryActive => (weight_total * 6.5, weight_total * 8.5),
        AthleteKind::IntenseActivity => (weight_total * 8.5, 0.0)
    }
}