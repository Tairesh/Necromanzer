#![allow(dead_code)]
use scenes::scene::Scene;

#[derive(Debug, Clone)]
pub enum Transition {
    Push(Scene),
    Pop,
    GoMainMenu,     // pop until one
    Replace(Scene), // pop and push
    CustomEvent(u8),
    ChangeSettings(SettingsChange),
    Quit,
}

#[derive(Debug, Copy, Clone)]
pub enum SettingsChange {
    FullscreenMode,
    WindowMode,
}

pub type SomeTransitions = Option<Vec<Transition>>;
