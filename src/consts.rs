pub mod msgs {

    mod formatting {
        use std::collections::HashMap;
        use std::convert::TryInto;
        use std::fmt::{self};

        use rt_format::{Format, FormatArgument, ParsedFormat, Specifier};

        #[derive(Debug, PartialEq)]
        pub enum ParseArg {
            Int(i32),
            Float(f32),
        }

        impl FormatArgument for ParseArg {
            fn supports_format(&self, spec: &Specifier) -> bool {
                match self {
                    Self::Int(_) => true,
                    Self::Float(_) => matches!(spec.format, Format::Display | Format::Debug | Format::LowerExp | Format::UpperExp)
                }
            }
        
            fn fmt_display(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    Self::Int(val) => fmt::Display::fmt(&val, f),
                    Self::Float(val) => fmt::Display::fmt(&val, f),
                }
            }
        
            fn fmt_debug(&self, f: &mut fmt::Formatter) -> fmt::Result {
                fmt::Debug::fmt(self, f)
            }
        
            fn fmt_octal(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    Self::Int(val) => fmt::Octal::fmt(&val, f),
                    _ => Err(fmt::Error),
                }
            }
        
            fn fmt_lower_hex(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    Self::Int(val) => fmt::LowerHex::fmt(&val, f),
                    _ => Err(fmt::Error),
                }
            }
        
            fn fmt_upper_hex(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    Self::Int(val) => fmt::UpperHex::fmt(&val, f),
                    _ => Err(fmt::Error),
                }
            }
        
            fn fmt_binary(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    Self::Int(val) => fmt::Binary::fmt(&val, f),
                    _ => Err(fmt::Error),
                }
            }
        
            fn fmt_lower_exp(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    Self::Int(val) => fmt::LowerExp::fmt(&val, f),
                    Self::Float(val) => fmt::LowerExp::fmt(&val, f),
                }
            }
        
            fn fmt_upper_exp(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    Self::Int(val) => fmt::UpperExp::fmt(&val, f),
                    Self::Float(val) => fmt::UpperExp::fmt(&val, f),
                }
            }
        }

        impl TryInto<usize> for &ParseArg {
            type Error = ();
            fn try_into(self) -> Result<usize, Self::Error> {
                match self {
                    ParseArg::Int(val) => (*val).try_into().map_err(|_| ()),
                    ParseArg::Float(_) => Err(()),
                }
            }
        }

        pub fn min_max_prot(label: &str, min_prot: f32, max_prot: f32) -> String {

            let mut map : HashMap<&str, ParseArg> = HashMap::new();
            map.insert("min_prot", ParseArg::Float(min_prot));
            map.insert("max_prot", ParseArg::Float(max_prot));
            let pos_args: &[ParseArg] = &[];
            format!("{}", ParsedFormat::parse(label, pos_args, &map).unwrap())
        }

        pub fn min_max_fat(label: &str, min_fat: f32, max_fat: f32) -> String {

            let mut map : HashMap<&str, ParseArg> = HashMap::new();
            map.insert("min_fat", ParseArg::Float(min_fat));
            map.insert("max_fat", ParseArg::Float(max_fat));
            let pos_args: &[ParseArg] = &[];
            format!("{}", ParsedFormat::parse(label, pos_args, &map).unwrap())
        }
    }
    

    // Calories
    pub const MIFFLIN_FORMULA:&str = "Using mifflin formula, this formula is imprecise and tends to overstimate, specially in cases with high body fat.";

    // Proteins
    pub const PROTEIN_TEEN: &str = "You are a teenager, so you need more proteins to grow properly.";
    pub const LOW_CARBS: &str = "For your goal, you need a very low carb diet, what we show you here is the bare minimum, but is perfectly okay to just follow the standard cuantity: A-B since proteins are good for reducing hanger and regulating sugar.";
    pub const HIGH_ACTIVITY: &str = "You do a lot in your day, so you need more proteins too.";
    pub const REALLY_LEAN: &str = "People who are really lean need a greater ammount of proteins in their diet.";
    pub const SEDENTARY: &str = "Sedentary people can just ingest less proteins, since they do less work.";
    pub const HIGH_FAT: &str = "Humans with high body fat can ingest a lower cuantity of proteins.";

    pub fn general_case(min_prot: f32, max_prot: f32) -> String {
        let label = "The number show here is the number tailored for you, but since proteins are so useful (big numbers reduce hunger, help with calories management, blood sugar control and hipertrofia) consuming as much as a bodybuilder would with your weight is a good general suggestion: ({min_prot$}-{max_prot$}).";
        formatting::min_max_prot(label, min_prot, max_prot)
    }

    // Fat
    pub fn low_carbs_diet_fat(min_fat: f32, max_fat:f32) -> String {
        let label = "This is the bare minimum for a diet with low carbohidrates, but the general case still applies: ({min-prot$}-{max-prot$}).";
        formatting::min_max_fat(label, min_fat, max_fat)
    }

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