use assets::Assets;
use scenes::bg;
use scenes::scene::Scene;
use sprites::image::Image;
use sprites::position::{Position, Vertical};
use sprites::sprite::Sprite;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::Context;

pub struct MainMenu {
    sprites: Vec<Rc<RefCell<dyn Sprite>>>,
}

impl MainMenu {
    pub fn new(_ctx: &mut Context, assets: &Assets) -> Self {
        let bg = bg(assets);
        let logo = Rc::new(RefCell::new(Image::new(
            assets.images.logo.clone(),
            Position::horizontal_center(0.0, Vertical::AtWindowCenterByBottom { offset: -100.0 }),
        )));

        Self {
            sprites: vec![bg, logo],
        }
    }
}

impl Scene for MainMenu {
    fn sprites(&mut self) -> Option<&Vec<Rc<RefCell<dyn Sprite>>>> {
        Some(&self.sprites)
    }
}
