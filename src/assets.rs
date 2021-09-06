use tetra::graphics::text::Font;
use tetra::graphics::{NineSlice, Rectangle, Texture};
use tetra::Context;

pub struct TilesetRegions {
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

    pub shovel: Rectangle,
    pub knife: Rectangle,
    pub axe: Rectangle,

    pub lt: Rectangle,
    pub mt: Rectangle,
    pub plus: Rectangle,
    pub minus: Rectangle,
}

pub struct Assets {
    pub default: Font,
    pub default2: Font,
    pub header1: Font,
    pub header2: Font,

    pub logo: Texture,
    pub bg: Texture,
    pub tileset: Texture,
    pub regions: TilesetRegions,
    pub button: Texture,
    pub button_default: NineSlice,
    pub button_disabled: NineSlice,
    pub button_pressed: NineSlice,
    pub button_hovered: NineSlice,
    pub alert: Texture,
    pub alert_nineslice: NineSlice,
    pub hat: Texture,
    pub bars: Texture,
    pub bar_red: NineSlice,
    pub bar_blue: NineSlice,

    pub male_names: Vec<&'static str>,
    pub female_names: Vec<&'static str>,
    pub names: Vec<&'static str>,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
        let mut male_names = Vec::with_capacity(259);
        let mut female_names = Vec::with_capacity(199);
        let mut names = Vec::with_capacity(458);
        for row in include_str!("../res/data/names.txt").lines() {
            let mut split = row.split(',');
            let name = split.next().unwrap();
            if name.is_empty() {
                continue;
            }
            let sex = split.next().expect(name);
            if sex == "1" {
                male_names.push(name);
            } else {
                female_names.push(name);
            }
            names.push(name);
        }
        let consolab = include_bytes!("../res/fonts/consolab.ttf");
        let avqest = include_bytes!("../res/fonts/avqest.ttf");
        Ok(Assets {
            default: Font::from_vector_file_data(ctx, consolab, 16.0)?,
            default2: Font::from_vector_file_data(ctx, consolab, 24.0)?,
            header1: Font::from_vector_file_data(ctx, avqest, 86.0)?,
            header2: Font::from_vector_file_data(ctx, avqest, 32.0)?,
            logo: Texture::from_file_data(ctx, include_bytes!("../res/img/logo.png"))?,
            bg: Texture::from_file_data(ctx, include_bytes!("../res/img/bg.jpg"))?,
            tileset: Texture::from_file_data(ctx, include_bytes!("../res/img/tileset.png"))?,
            regions: TilesetRegions {
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

                shovel: Rectangle::new(0.0, 70.0, 10.0, 10.0),
                knife: Rectangle::new(10.0, 70.0, 10.0, 10.0),
                axe: Rectangle::new(20.0, 70.0, 10.0, 10.0),

                mt: Rectangle::new(0.0, 90.0, 10.0, 10.0),
                lt: Rectangle::new(10.0, 90.0, 10.0, 10.0),
                minus: Rectangle::new(20.0, 90.0, 10.0, 10.0),
                plus: Rectangle::new(30.0, 90.0, 10.0, 10.0),
            },
            button: Texture::from_file_data(ctx, include_bytes!("../res/img/button.png"))?,
            button_default: NineSlice::new(
                Rectangle::new(0.0, 0.0, 46.0, 14.0),
                3.0,
                3.0,
                3.0,
                4.0,
            ),
            button_hovered: NineSlice::new(
                Rectangle::new(0.0, 14.0, 46.0, 14.0),
                3.0,
                3.0,
                3.0,
                4.0,
            ),
            button_pressed: NineSlice::new(
                Rectangle::new(0.0, 28.0, 46.0, 14.0),
                3.0,
                3.0,
                4.0,
                3.0,
            ),
            button_disabled: NineSlice::new(
                Rectangle::new(0.0, 42.0, 46.0, 14.0),
                3.0,
                3.0,
                3.0,
                4.0,
            ),
            alert: Texture::from_file_data(ctx, include_bytes!("../res/img/alert.png"))?,
            alert_nineslice: NineSlice::new(
                Rectangle::new(0.0, 0.0, 48.0, 32.0),
                6.0,
                6.0,
                6.0,
                5.0,
            ),
            hat: Texture::from_file_data(ctx, include_bytes!("../res/img/hat.png"))?,
            bars: Texture::from_file_data(ctx, include_bytes!("../res/img/bars.png"))?,
            bar_red: NineSlice::new(Rectangle::new(0.0, 0.0, 7.0, 3.0), 1.0, 4.0, 0.0, 0.0),
            bar_blue: NineSlice::new(Rectangle::new(0.0, 3.0, 7.0, 3.0), 1.0, 4.0, 0.0, 0.0),
            male_names,
            female_names,
            names,
        })
    }
}
