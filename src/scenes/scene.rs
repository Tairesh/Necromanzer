use scenes::transition::Transition;
use sprites::sprite::Sprite;
use std::cell::RefCell;
use std::rc::Rc;
use tetra::{Context, Event};

pub trait Scene {
    fn update(&mut self, _ctx: &mut Context, _focused: bool) -> Vec<Transition> {
        vec![]
    }
    fn event(&mut self, _ctx: &mut Context, _event: Event, _focused: bool) -> Vec<Transition> {
        vec![]
    }
    fn before_draw(&mut self, _ctx: &mut Context) {}
    fn after_draw(&mut self, _ctx: &mut Context) {}
    fn on_open(&mut self, _ctx: &mut Context) {}
    fn on_resize(&mut self, _ctx: &mut Context) {}
    fn sprites(&mut self) -> Option<&Vec<Rc<RefCell<dyn Sprite>>>> {
        None
    }
    fn custom_event(&mut self, _ctx: &mut Context, _event: &str) -> Vec<Transition> {
        vec![]
    }
}
