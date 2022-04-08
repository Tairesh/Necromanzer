#![allow(dead_code)]
use scenes::game_scene::GameScene;

#[derive(Debug, Clone)]
pub enum Transition {
    Push(GameScene),
    Pop,
    Replace(GameScene), // pop and push
    CustomEvent(String),
    Quit,
}
