use tetra::graphics::text::Font;
use tetra::graphics::Texture;
use tetra::Context;

pub struct Assets {
    pub default: Font,
    pub header1: Font,
    pub header2: Font,
    pub logo: Texture,
    pub bg: Texture,
    pub button: Texture,
    pub button_disabled: Texture,
    pub button_pressed: Texture,
    pub button_hovered: Texture,
    pub male_names: Vec<&'static str>,
    pub female_names: Vec<&'static str>,
    pub names: Vec<&'static str>,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
        let mut male_names = Vec::with_capacity(259);
        let mut female_names = Vec::with_capacity(199);
        let mut names = Vec::with_capacity(458);
        for row in include_str!("../res/data/names.txt").split('\n') {
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
        Ok(Assets {
            default: Font::from_vector_file_data(
                ctx,
                include_bytes!("../res/fonts/consolab.ttf"),
                16.0,
            )?,
            header1: Font::from_vector_file_data(
                ctx,
                include_bytes!("../res/fonts/avqest.ttf"),
                86.0,
            )?,
            header2: Font::from_vector_file_data(
                ctx,
                include_bytes!("../res/fonts/avqest.ttf"),
                32.0,
            )?,
            logo: Texture::from_file_data(ctx, include_bytes!("../res/img/logo.png"))?,
            bg: Texture::from_file_data(ctx, include_bytes!("../res/img/bg.jpg"))?,
            button: Texture::from_file_data(ctx, include_bytes!("../res/img/button.png"))?,
            button_disabled: Texture::from_file_data(
                ctx,
                include_bytes!("../res/img/button_disabled.png"),
            )?,
            button_pressed: Texture::from_file_data(
                ctx,
                include_bytes!("../res/img/button_pressed.png"),
            )?,
            button_hovered: Texture::from_file_data(
                ctx,
                include_bytes!("../res/img/button_hovered.png"),
            )?,
            male_names,
            female_names,
            names,
        })
    }
}
