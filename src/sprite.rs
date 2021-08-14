use sdl2::image::LoadTexture;
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
            default_color: Color::RGB(0, 0, 0),
        }
    }

    pub fn image_size(&self, path: &str) -> (u32, u32) {
        let query = self.texture_creator.load_texture(path).unwrap().query();
        (query.width, query.height)
    }

    pub fn load_image(&mut self, sprite: &ImgSprite) -> &Texture {
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

    pub fn text_size(&self, text: &str) -> (u32, u32) {
        let default_font = self
            .font_context
            .load_font("res/fonts/consolab.ttf", 16)
            .unwrap();
        default_font.size_of(text).unwrap()
    }

    pub fn render_text(&mut self, sprite: &TextSprite) -> &Texture {
        let default_font = self
            .font_context
            .load_font("res/fonts/consolab.ttf", 16)
            .unwrap();
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
                        default_font
                            .render(sprite.text.as_str())
                            .blended(color)
                            .unwrap(),
                    )
                    .unwrap(),
            );
        }
        self.textures.get(hash.as_str()).unwrap()
    }

    pub fn render_button(&mut self, button: &Button) -> &Texture {
        let default_font = self
            .font_context
            .load_font("res/fonts/consolab.ttf", 16)
            .unwrap();
        let (fg_color, bg_color) = match button.state {
            ButtonState::Hovered => (Color::RGB(255, 255, 255), Color::RGBA(0, 50, 0, 200)),
            ButtonState::Focused => (Color::RGB(255, 255, 255), Color::RGBA(94, 75, 47, 200)),
            ButtonState::Pressed => (Color::RGB(0, 0, 0), Color::RGBA(0, 255, 0, 200)),
            ButtonState::Disabled => (Color::RGB(33, 33, 33), Color::RGBA(128, 128, 128, 200)),
            ButtonState::Default => (Color::RGB(0, 255, 0), Color::RGBA(0, 0, 0, 200)),
        };
        let hash = format!(
            "button:{}:{}:{}:{}",
            button.text, button.size.0, button.size.1, button.state as i32
        );
        if !self.textures.contains_key(hash.as_str()) {
            let mut surface =
                Surface::new(button.size.0, button.size.1, PixelFormatEnum::RGBA32).unwrap();
            draw_rect(&mut surface, Some(fg_color), Some(bg_color), 2);

            let text_surface = default_font
                .render(button.text.as_str())
                .blended(fg_color)
                .unwrap();
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
pub struct ImgSprite {
    pub path: String,
    pub position: (i32, i32),
}

#[derive(Hash, Eq, PartialEq)]
pub struct TextSprite {
    pub text: String,
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
    pub text: String,
    pub size: (u32, u32),
    pub position: (i32, i32),
    pub state: ButtonState,
}
