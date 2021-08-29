use colors::Colors;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use tetra::graphics::Color;

pub enum SkinTone {
    PaleIvory,
    WarmIvory,
    Sand,
    RoseBeige,
    Sienna,
    Limestone,
    Beige,
    Amber,
    Honey,
    Band,
    Almond,
    Umber,
    Bronze,
    Golden,
    Espresso,
    Chocolate,
}

impl SkinTone {
    pub fn name(&self) -> &str {
        match self {
            SkinTone::PaleIvory => "Pale Ivory",
            SkinTone::WarmIvory => "Warm Ivory",
            SkinTone::Sand => "Sandy",
            SkinTone::RoseBeige => "Rose Beige",
            SkinTone::Sienna => "Sienna",
            SkinTone::Limestone => "Limestone",
            SkinTone::Beige => "Beige",
            SkinTone::Amber => "Amber",
            SkinTone::Honey => "Honey",
            SkinTone::Band => "Band",
            SkinTone::Almond => "Almond",
            SkinTone::Umber => "Umber",
            SkinTone::Bronze => "Bronze",
            SkinTone::Golden => "Golden",
            SkinTone::Espresso => "Espresso",
            SkinTone::Chocolate => "Chocolate",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            SkinTone::PaleIvory => SkinTone::WarmIvory,
            SkinTone::WarmIvory => SkinTone::Sand,
            SkinTone::Sand => SkinTone::RoseBeige,
            SkinTone::RoseBeige => SkinTone::Sienna,
            SkinTone::Sienna => SkinTone::Limestone,
            SkinTone::Limestone => SkinTone::Beige,
            SkinTone::Beige => SkinTone::Amber,
            SkinTone::Amber => SkinTone::Honey,
            SkinTone::Honey => SkinTone::Band,
            SkinTone::Band => SkinTone::Almond,
            SkinTone::Almond => SkinTone::Umber,
            SkinTone::Umber => SkinTone::Bronze,
            SkinTone::Bronze => SkinTone::Golden,
            SkinTone::Golden => SkinTone::Espresso,
            SkinTone::Espresso => SkinTone::Chocolate,
            SkinTone::Chocolate => SkinTone::PaleIvory,
        }
    }

    pub fn prev(&self) -> Self {
        match self {
            SkinTone::PaleIvory => SkinTone::Chocolate,
            SkinTone::WarmIvory => SkinTone::PaleIvory,
            SkinTone::Sand => SkinTone::WarmIvory,
            SkinTone::RoseBeige => SkinTone::Sand,
            SkinTone::Sienna => SkinTone::RoseBeige,
            SkinTone::Limestone => SkinTone::Sienna,
            SkinTone::Beige => SkinTone::Limestone,
            SkinTone::Amber => SkinTone::Beige,
            SkinTone::Honey => SkinTone::Amber,
            SkinTone::Band => SkinTone::Honey,
            SkinTone::Almond => SkinTone::Band,
            SkinTone::Umber => SkinTone::Almond,
            SkinTone::Bronze => SkinTone::Umber,
            SkinTone::Golden => SkinTone::Bronze,
            SkinTone::Espresso => SkinTone::Golden,
            SkinTone::Chocolate => SkinTone::Espresso,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            SkinTone::PaleIvory => Colors::PALE_IVORY,
            SkinTone::WarmIvory => Colors::WARM_IVORY,
            SkinTone::Sand => Colors::SAND,
            SkinTone::RoseBeige => Colors::ROSE_BEIGE,
            SkinTone::Sienna => Colors::SIENNA,
            SkinTone::Limestone => Colors::LIMESTONE,
            SkinTone::Beige => Colors::BEIGE,
            SkinTone::Amber => Colors::AMBER,
            SkinTone::Honey => Colors::HONEY,
            SkinTone::Band => Colors::BAND,
            SkinTone::Almond => Colors::ALMOND,
            SkinTone::Umber => Colors::UMBER,
            SkinTone::Bronze => Colors::BRONZE,
            SkinTone::Golden => Colors::GOLDEN,
            SkinTone::Espresso => Colors::ESPRESSO,
            SkinTone::Chocolate => Colors::CHOCOLATE,
        }
    }

    pub fn text_color(&self) -> Color {
        match self {
            SkinTone::Almond
            | SkinTone::Umber
            | SkinTone::Bronze
            | SkinTone::Golden
            | SkinTone::Espresso
            | SkinTone::Chocolate => Colors::LIGHT_YELLOW,
            _ => Colors::DARK_BROWN,
        }
    }
}

impl Distribution<SkinTone> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SkinTone {
        match rng.gen_range(0..=15) {
            0 => SkinTone::PaleIvory,
            1 => SkinTone::WarmIvory,
            2 => SkinTone::Sand,
            3 => SkinTone::RoseBeige,
            4 => SkinTone::Sienna,
            5 => SkinTone::Limestone,
            6 => SkinTone::Beige,
            7 => SkinTone::Amber,
            8 => SkinTone::Honey,
            9 => SkinTone::Band,
            10 => SkinTone::Almond,
            11 => SkinTone::Umber,
            12 => SkinTone::Bronze,
            13 => SkinTone::Golden,
            14 => SkinTone::Espresso,
            15 => SkinTone::Chocolate,
            _ => {
                panic!("Rust is the memory safe language with zero cost abstractions!");
            }
        }
    }
}
