use tetra::graphics::{Rectangle, Texture};
use tetra::Context;

pub struct Tileset {
    pub texture: Texture,

    pub male: Rectangle,
    pub female: Rectangle,
    pub queer: Rectangle,
    pub zombie: Rectangle,
    pub zombie_child: Rectangle,
    pub skeleton: Rectangle,
    pub skeleton_child: Rectangle,
    pub raw_zombie: Rectangle,
    pub raw_zombie_child: Rectangle,
    pub highlight: Rectangle,

    pub dirt1: Rectangle,
    pub dirt2: Rectangle,
    pub dirt3: Rectangle,
    pub dirt4: Rectangle,
    pub dirt5: Rectangle,

    pub boulder_huge: Rectangle,
    pub boulder_middle: Rectangle,
    pub boulder_small: Rectangle,
    pub pit: Rectangle,

    pub grass1: Rectangle,
    pub grass2: Rectangle,
    pub grass3: Rectangle,
    pub grass4: Rectangle,
    pub grass5: Rectangle,
    pub grass6: Rectangle,
    pub grass7: Rectangle,

    pub grass8: Rectangle,
    pub grass9: Rectangle,
    pub grass10: Rectangle,
    pub grass11: Rectangle,
    pub grass12: Rectangle,
    pub grass13: Rectangle,
    pub grass14: Rectangle,

    pub dead_grass1: Rectangle,
    pub dead_grass2: Rectangle,
    pub dead_grass3: Rectangle,
    pub dead_grass4: Rectangle,
    pub dead_grass5: Rectangle,
    pub dead_grass6: Rectangle,
    pub dead_grass7: Rectangle,

    pub dead_grass8: Rectangle,
    pub dead_grass9: Rectangle,
    pub dead_grass10: Rectangle,
    pub dead_grass11: Rectangle,
    pub dead_grass12: Rectangle,
    pub dead_grass13: Rectangle,
    pub dead_grass14: Rectangle,

    pub grave_new: Rectangle,
    pub grave_old: Rectangle,
    pub grave_stone: Rectangle,
    pub corpse: Rectangle,
    pub flesh: Rectangle,

    pub shovel: Rectangle,
    pub knife: Rectangle,
    pub axe: Rectangle,

    pub lt: Rectangle,
    pub mt: Rectangle,
    pub plus: Rectangle,
    pub minus: Rectangle,
}

impl Tileset {
    pub fn load(ctx: &mut Context) -> tetra::Result<Self> {
        Ok(Self {
            texture: Texture::from_encoded(ctx, include_bytes!("../../inc/img/tileset.png"))?,

            female: Rectangle::new(0.0, 0.0, 10.0, 10.0),
            male: Rectangle::new(10.0, 0.0, 10.0, 10.0),
            queer: Rectangle::new(20.0, 0.0, 10.0, 10.0),
            zombie: Rectangle::new(30.0, 0.0, 10.0, 10.0),
            zombie_child: Rectangle::new(40.0, 0.0, 10.0, 10.0),
            skeleton: Rectangle::new(50.0, 0.0, 10.0, 10.0),
            skeleton_child: Rectangle::new(60.0, 0.0, 10.0, 10.0),
            raw_zombie: Rectangle::new(70.0, 0.0, 10.0, 10.0),
            raw_zombie_child: Rectangle::new(80.0, 0.0, 10.0, 10.0),
            highlight: Rectangle::new(90.0, 0.0, 10.0, 10.0),

            dirt1: Rectangle::new(0.0, 10.0, 10.0, 10.0),
            dirt2: Rectangle::new(10.0, 10.0, 10.0, 10.0),
            dirt3: Rectangle::new(20.0, 10.0, 10.0, 10.0),
            dirt4: Rectangle::new(30.0, 10.0, 10.0, 10.0),
            dirt5: Rectangle::new(40.0, 10.0, 10.0, 10.0),

            boulder_huge: Rectangle::new(50.0, 10.0, 10.0, 10.0),
            boulder_middle: Rectangle::new(60.0, 10.0, 10.0, 10.0),
            boulder_small: Rectangle::new(70.0, 10.0, 10.0, 10.0),
            pit: Rectangle::new(80.0, 10.0, 10.0, 10.0),

            grass1: Rectangle::new(0.0, 20.0, 10.0, 10.0),
            grass2: Rectangle::new(10.0, 20.0, 10.0, 10.0),
            grass3: Rectangle::new(20.0, 20.0, 10.0, 10.0),
            grass4: Rectangle::new(30.0, 20.0, 10.0, 10.0),
            grass5: Rectangle::new(40.0, 20.0, 10.0, 10.0),
            grass6: Rectangle::new(50.0, 20.0, 10.0, 10.0),
            grass7: Rectangle::new(60.0, 20.0, 10.0, 10.0),
            grass8: Rectangle::new(0.0, 40.0, 10.0, 10.0),
            grass9: Rectangle::new(10.0, 40.0, 10.0, 10.0),
            grass10: Rectangle::new(20.0, 40.0, 10.0, 10.0),
            grass11: Rectangle::new(30.0, 40.0, 10.0, 10.0),
            grass12: Rectangle::new(40.0, 40.0, 10.0, 10.0),
            grass13: Rectangle::new(50.0, 40.0, 10.0, 10.0),
            grass14: Rectangle::new(60.0, 40.0, 10.0, 10.0),
            dead_grass1: Rectangle::new(0.0, 30.0, 10.0, 10.0),
            dead_grass2: Rectangle::new(10.0, 30.0, 10.0, 10.0),
            dead_grass3: Rectangle::new(20.0, 30.0, 10.0, 10.0),
            dead_grass4: Rectangle::new(30.0, 30.0, 10.0, 10.0),
            dead_grass5: Rectangle::new(40.0, 30.0, 10.0, 10.0),
            dead_grass6: Rectangle::new(50.0, 30.0, 10.0, 10.0),
            dead_grass7: Rectangle::new(60.0, 30.0, 10.0, 10.0),
            dead_grass8: Rectangle::new(0.0, 50.0, 10.0, 10.0),
            dead_grass9: Rectangle::new(10.0, 50.0, 10.0, 10.0),
            dead_grass10: Rectangle::new(20.0, 50.0, 10.0, 10.0),
            dead_grass11: Rectangle::new(30.0, 50.0, 10.0, 10.0),
            dead_grass12: Rectangle::new(40.0, 50.0, 10.0, 10.0),
            dead_grass13: Rectangle::new(50.0, 50.0, 10.0, 10.0),
            dead_grass14: Rectangle::new(60.0, 50.0, 10.0, 10.0),

            grave_new: Rectangle::new(0.0, 60.0, 10.0, 10.0),
            grave_old: Rectangle::new(10.0, 60.0, 10.0, 10.0),
            grave_stone: Rectangle::new(20.0, 60.0, 10.0, 10.0),
            corpse: Rectangle::new(30.0, 60.0, 10.0, 10.0),
            flesh: Rectangle::new(40.0, 60.0, 10.0, 10.0),

            shovel: Rectangle::new(0.0, 70.0, 10.0, 10.0),
            knife: Rectangle::new(10.0, 70.0, 10.0, 10.0),
            axe: Rectangle::new(20.0, 70.0, 10.0, 10.0),

            mt: Rectangle::new(0.0, 90.0, 10.0, 10.0),
            lt: Rectangle::new(10.0, 90.0, 10.0, 10.0),
            minus: Rectangle::new(20.0, 90.0, 10.0, 10.0),
            plus: Rectangle::new(30.0, 90.0, 10.0, 10.0),
        })
    }
}
