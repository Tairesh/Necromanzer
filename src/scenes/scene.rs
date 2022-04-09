use scenes::transition::SomeTransitions;
use sprites::SomeSprites;
use tetra::{Context, Event};

pub trait Scene {
    fn update(&mut self, _ctx: &mut Context) -> SomeTransitions {
        None
    }
    fn event(&mut self, _ctx: &mut Context, _event: Event) -> SomeTransitions {
        None
    }
    fn before_draw(&mut self, _ctx: &mut Context) {}
    fn after_draw(&mut self, _ctx: &mut Context) {}
    fn on_open(&mut self, _ctx: &mut Context) {}
    fn on_resize(&mut self, _ctx: &mut Context, _window_size: (i32, i32)) {}
    fn sprites(&self) -> SomeSprites {
        None
    }
    fn custom_event(&mut self, _ctx: &mut Context, _event: &str) -> SomeTransitions {
        None
    }

    fn is_there_focused_sprite(&self) -> bool {
        self.sprites()
            .map(|sprites| sprites.iter().any(|s| s.borrow().focused()))
            .unwrap_or(false)
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
