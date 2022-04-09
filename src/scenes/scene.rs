use scenes::transition::Transition;
use sprites::sprite::Sprite;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::{Context, Event};

pub trait Scene {
    fn update(&mut self, _ctx: &mut Context) -> Vec<Transition> {
        vec![]
    }
    fn event(&mut self, _ctx: &mut Context, _event: Event) -> Vec<Transition> {
        vec![]
    }
    fn before_draw(&mut self, _ctx: &mut Context) {}
    fn after_draw(&mut self, _ctx: &mut Context) {}
    fn on_open(&mut self, _ctx: &mut Context) {}
    fn on_resize(&mut self, _ctx: &mut Context, _window_size: (i32, i32)) {}
    fn sprites(&mut self) -> Option<&Vec<Rc<RefCell<dyn Sprite>>>> {
        None
    }
    fn custom_event(&mut self, _ctx: &mut Context, _event: &str) -> Vec<Transition> {
        vec![]
    }
}

pub fn reposition_all_sprites(
    scene: &mut Box<dyn Scene>,
    ctx: &mut Context,
    window_size: (i32, i32),
) {
    if let Some(sprites) = scene.sprites() {
        for sprite in sprites.iter() {
            sprite.borrow_mut().positionate(ctx, window_size);
        }
    }
}

pub fn is_there_focused_sprite(scene: &mut Box<dyn Scene>) -> bool {
    scene
        .sprites()
        .map(|sprites| sprites.iter().any(|s| s.borrow().focused()))
        .unwrap_or(false)
}
