use tetra::graphics::text::Font;
use tetra::graphics::{NineSlice, Rectangle, Texture};
use tetra::Context;

pub struct Icons {
    pub male: Rectangle,
    pub female: Rectangle,
    pub queer: Rectangle,
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
    pub icons: Icons,
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
            icons: Icons {
                male: Rectangle::new(0.0, 0.0, 10.0, 10.0),
                female: Rectangle::new(10.0, 0.0, 10.0, 10.0),
                queer: Rectangle::new(20.0, 0.0, 10.0, 10.0),
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
