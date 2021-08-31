use rand::distributions::{Distribution, Standard};
use rand::Rng;

pub enum DirtVariant {
    Dirt1,
    Dirt2,
    Dirt3,
    Dirt4,
    Dirt5,
}

impl Distribution<DirtVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> DirtVariant {
        if rng.gen_bool(0.9) {
            DirtVariant::Dirt3
        } else {
            match rng.gen_range(0..4) {
                0 => DirtVariant::Dirt1,
                1 => DirtVariant::Dirt2,
                2 => DirtVariant::Dirt4,
                3 => DirtVariant::Dirt5,
                _ => unreachable!(),
            }
        }
    }
}

pub enum BoulderVariant {
    One1,
    One2,
    One3,
    Two1,
    Two2,
    Three1,
    Three2,
}

impl Distribution<BoulderVariant> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> BoulderVariant {
        match rng.gen_range(0..7) {
            0 => BoulderVariant::One1,
            1 => BoulderVariant::One2,
            2 => BoulderVariant::One3,
            3 => BoulderVariant::Two1,
            4 => BoulderVariant::Two2,
            5 => BoulderVariant::Three1,
            6 => BoulderVariant::Three2,
            _ => unreachable!(),
        }
    }
}

pub enum TileBase {
    Dirt(DirtVariant),
    Boulder(BoulderVariant),
}
