pub mod msgs {
    // Calories
    pub const MIFFLIN_FORMULA:&str = "Using mifflin formula, this formula is imprecise and tends to overstimate, specially in cases with high body fat";

    // Proteins
    pub const PROTEIN_TEEN: &str = "You are a teenager, so you need more proteins to grow properly";
    pub const LOW_CARBS: &str = "For your goal, you need a very low carb diet, what we show you here is the bare minimum, but is perfectly okay to just follow the standard cuantity: A-B since proteins are good for reducing hanger and regulating sugar.";
    pub const HIGH_ACTIVITY: &str = "You do a lot in your day, so you need more proteins too";
    pub const REALLY_LEAN: &str = "People who are really lean need a greater ammount of proteins in their diet";
    pub const SEDENTARY: &str = "Sedentary people can just ingest less proteins, since they do less work";
    pub const HIGH_FAT: &str = "Humans with high body fat can ingest a lower cuantity of proteins";
    pub const GENERAL_CASE: &str = "The number show here is the number tailored for you, but since proteins are so useful (big numbers reduce hunger, help with calories management, blood sugar control and hypertofia) consuming as much as a bodybuilder would with your weight is a good general suggestion: (X-Y)";

    // Fat
    pub const LOW_CARBS_DIET_FAT: &str = "This is the bare minimum for a diet with low carbohidrates, but the general case still applies: (X-Y)";

}

pub mod nums {
    pub const HIGH_FAT_THRESHOLD: f32 = 0.2; // TODO: Hablar con Carlos
    pub const HIGHER_FAT_THRESHOLD: f32 = 0.3; // TODO: Hablar con Carlos

    pub const REALLY_LEAN_MALE: f32 = 0.1;
    pub const REALLY_LEAN_FEMALE: f32 = 0.15;

    pub const TEEN_MAX_AGE: u8 = 17;
}

pub mod gui {
    pub const RADIO_GROUPS_BORDER: u32 = 3;
    pub const RADIO_GROUPS_SPACING: i32 = 17;
}