#![allow(dead_code)]

use scenes::scene::Scene;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Transition {
    Push(Scene),
    Pop,
    GoMainMenu,     // unload world and pop until one
    Replace(Scene), // pop and push
    LoadWorld(PathBuf),
    UnloadWorld,
    CustomEvent(u8),
    Quit,
}

pub type SomeTransitions = Option<Vec<Transition>>;
