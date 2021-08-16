use colors;
use sdl2::image::LoadTexture;
use sdl2::keyboard::Scancode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator};
use sdl2::surface::Surface;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::video::WindowContext;
use std::collections::HashMap;

fn draw_rect(
    surface: &mut Surface,
    border_color: Option<Color>,
    bg_color: Option<Color>,
    border_width: u32,
) {
    let size = surface.size();
    if let Some(bg_color) = bg_color {
        surface
            .fill_rect(Rect::new(0, 0, size.0, size.1), bg_color)
            .ok();
    }
    if border_width > 0 {
        if let Some(border_color) = border_color {
            surface
                .fill_rects(
                    [
                        Rect::new(0, 0, size.0, border_width),
                        Rect::new(0, 0, border_width, size.1),
                        Rect::new((size.0 - border_width) as i32, 0, border_width, size.1),
                        Rect::new(0, (size.1 - border_width) as i32, size.0, border_width),
                    ]
                    .as_ref(),
                    border_color,
                )
                .ok();
        }
    }
}

fn font_path_and_size<'a>(font: LabelFont) -> (&'a str, u16) {
    match font {
        LabelFont::Default => ("res/fonts/consolab.ttf", 16),
        LabelFont::Title => ("res/fonts/avqest.ttf", 64),
    }
}

pub struct SpritesManager {
    font_context: Sdl2TtfContext,
    texture_creator: TextureCreator<WindowContext>,
    textures: HashMap<String, Texture>,
    default_color: Color,
}

impl SpritesManager {
    pub fn new(
        font_context: Sdl2TtfContext,
        texture_creator: TextureCreator<WindowContext>,
    ) -> SpritesManager {
        SpritesManager {
            font_context,
            texture_creator,
            textures: HashMap::new(),
            default_color: colors::rgb(colors::BLACK),
        }
    }

    pub fn image_size(&self, path: &str) -> (u32, u32) {
        let query = self.texture_creator.load_texture(path).unwrap().query();
        (query.width, query.height)
    }

    pub fn text_size(&self, text: &str, font: LabelFont) -> (u32, u32) {
        let (path, size) = font_path_and_size(font);
        let font = self.font_context.load_font(path, size).unwrap();
        font.size_of(text).unwrap()
    }

    pub fn render_sprite(&mut self, sprite: &Sprite) -> &Texture {
        match sprite {
            Sprite::Image(img) => self.render_image(img),
            Sprite::Label(label) => self.render_label(label),
            Sprite::Button(btn) => self.render_button(btn),
        }
    }

    fn render_image(&mut self, sprite: &Image) -> &Texture {
        if !self.textures.contains_key(sprite.path.as_str()) {
            self.textures.insert(
                sprite.path.clone(),
                self.texture_creator
                    .load_texture(sprite.path.as_str())
                    .unwrap(),
            );
        }
        self.textures.get(sprite.path.as_str()).unwrap()
    }

    fn render_label(&mut self, sprite: &Label) -> &Texture {
        let (path, size) = font_path_and_size(sprite.font);
        let font = self.font_context.load_font(path, size).unwrap();
        let color = sprite.color.unwrap_or(self.default_color);
        let hash = format!(
            "{}:{}:{}:{}:{}",
            sprite.text, color.r, color.g, color.b, color.a
        );
        if !self.textures.contains_key(hash.as_str()) {
            self.textures.insert(
                hash.clone(),
                self.texture_creator
                    .create_texture_from_surface(
                        font.render(sprite.text.as_str()).blended(color).unwrap(),
                    )
                    .unwrap(),
            );
        }
        self.textures.get(hash.as_str()).unwrap()
    }

    fn render_button(&mut self, button: &Button) -> &Texture {
        let (path, size) = font_path_and_size(LabelFont::Default);
        let font = self.font_context.load_font(path, size).unwrap();
        let (fg_color, bg_color) = match button.state {
            ButtonState::Hovered => (
                colors::rgb(colors::WHITE),
                colors::rgba(colors::DARK_GREEN, 200),
            ),
            ButtonState::Focused => (
                colors::rgb(colors::WHITE),
                colors::rgba(colors::DARK_SEPIA, 200),
            ),
            ButtonState::Pressed => (colors::rgb(colors::BLACK), colors::rgba(colors::LIME, 200)),
            ButtonState::Disabled => (
                colors::rgb(colors::DARK_GRAY),
                colors::rgba(colors::GRAY, 200),
            ),
            ButtonState::Default => (colors::rgb(colors::LIME), colors::rgba(colors::BLACK, 200)),
        };
        let hash = format!(
            "button:{}:{}:{}:{}",
            button.text, button.size.0, button.size.1, button.state as i32
        );
        if !self.textures.contains_key(hash.as_str()) {
            let mut surface =
                Surface::new(button.size.0, button.size.1, PixelFormatEnum::RGBA32).unwrap();
            draw_rect(&mut surface, Some(fg_color), Some(bg_color), 2);

            let text_surface = font.render(button.text.as_str()).blended(fg_color).unwrap();
            let (w, h) = text_surface.size();
            text_surface
                .blit(
                    None,
                    &mut surface,
                    Rect::new(
                        button.size.0 as i32 / 2 - w as i32 / 2,
                        button.size.1 as i32 / 2 - h as i32 / 2,
                        w,
                        h,
                    ),
                )
                .ok();

            self.textures.insert(
                hash.clone(),
                self.texture_creator
                    .create_texture_from_surface(surface)
                    .unwrap(),
            );
        }
        self.textures.get(hash.as_str()).unwrap()
    }
}

#[derive(Hash, Eq, PartialEq)]
pub struct SceneSprites {
    pub sprites: Vec<Sprite>,
}

impl SceneSprites {
    pub fn new() -> SceneSprites {
        SceneSprites { sprites: vec![] }
    }

    pub fn add_sprite(&mut self, sprite: Sprite) {
        self.sprites.push(sprite);
    }
}

#[derive(Hash, Eq, PartialEq)]
pub struct Image {
    pub path: String,
    pub position: (i32, i32),
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum LabelFont {
    Default,
    Title,
}

#[derive(Hash, Eq, PartialEq)]
pub struct Label {
    pub text: String,
    pub font: LabelFont,
    pub color: Option<Color>,
    pub position: (i32, i32),
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum ButtonState {
    Default,
    Hovered,
    Focused,
    Pressed,
    Disabled,
}

#[derive(Hash, Eq, PartialEq)]
pub struct Button {
    pub id: String,
    pub key: Scancode,
    pub text: String,
    pub size: (u32, u32),
    pub position: (i32, i32),
    pub state: ButtonState,
}

#[enum_dispatch::enum_dispatch]
pub trait SpriteT {
    fn position(&self) -> (i32, i32);
}
impl SpriteT for Image {
    fn position(&self) -> (i32, i32) {
        self.position
    }
}
impl SpriteT for Label {
    fn position(&self) -> (i32, i32) {
        self.position
    }
}
impl SpriteT for Button {
    fn position(&self) -> (i32, i32) {
        self.position
    }
}

#[enum_dispatch::enum_dispatch(SpriteT)]
#[derive(Hash, Eq, PartialEq)]
pub enum Sprite {
    Image,
    Label,
    Button,
}
