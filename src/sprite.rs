use sdl2::pixels::Color;
use sdl2::rect::Rect;

#[derive(Hash, Eq, PartialEq)]
pub struct ImgSprite {
    pub path: String,
    pub position: Rect,
}

#[derive(Hash, Eq, PartialEq)]
pub struct TextSprite {
    pub text: String,
    pub color: Option<Color>,
    pub position: Rect,
}
