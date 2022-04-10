#![allow(dead_code)]
use scenes::game_scene::GameScene;

#[derive(Debug, Clone)]
pub enum Transition {
    Push(GameScene),
    Pop,
    GoMainMenu,         // pop until one
    Replace(GameScene), // pop and push
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
