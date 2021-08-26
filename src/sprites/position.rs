#![allow(dead_code)]
use tetra::TetraVec2;

#[derive(Copy, Clone)]
pub enum Horizontal {
    ByLeft { x: f32 },
    ByCenter { x: f32 },
    ByRight { x: f32 },
    AtWindowCenter { offset: f32 },
    AtWindowRight { offset: f32 },
}

#[derive(Copy, Clone)]
pub enum Vertical {
    ByTop { y: f32 },
    ByCenter { y: f32 },
    ByBottom { y: f32 },
    AtWindowCenter { offset: f32 },
    AtWindowBottom { offset: f32 },
}

#[derive(Copy, Clone)]
pub struct Position {
    pub x: Horizontal,
    pub y: Vertical,
}

pub enum AnchorX {
    Left,
    Center,
    Right,
}

impl AnchorX {
    pub fn to_position(&self, x: f32) -> Horizontal {
        match self {
            AnchorX::Left => Horizontal::ByLeft { x },
            AnchorX::Center => Horizontal::ByCenter { x },
            AnchorX::Right => Horizontal::ByRight { x },
        }
    }
}

pub enum AnchorY {
    Top,
    Center,
    Bottom,
}

impl AnchorY {
    pub fn to_position(&self, y: f32) -> Vertical {
        match self {
            AnchorY::Top => Vertical::ByTop { y },
            AnchorY::Center => Vertical::ByCenter { y },
            AnchorY::Bottom => Vertical::ByBottom { y },
        }
    }
}

impl Position {
    pub fn new(x: f32, y: f32, anchor_x: AnchorX, anchor_y: AnchorY) -> Position {
        Position {
            x: anchor_x.to_position(x),
            y: anchor_y.to_position(y),
        }
    }

    pub fn empty() -> Position {
        Position {
            x: Horizontal::ByLeft { x: 0.0 },
            y: Vertical::ByTop { y: 0.0 },
        }
    }

    pub fn center() -> Position {
        Position {
            x: Horizontal::AtWindowCenter { offset: 0.0 },
            y: Vertical::AtWindowCenter { offset: 0.0 },
        }
    }

    pub fn horizontal_center(y: f32, anchor_y: AnchorY) -> Position {
        Position {
            x: Horizontal::AtWindowCenter { offset: 0.0 },
            y: anchor_y.to_position(y),
        }
    }

    pub fn vertical_center(x: f32, anchor_x: AnchorX) -> Position {
        Position {
            x: anchor_x.to_position(x),
            y: Vertical::AtWindowCenter { offset: 0.0 },
        }
    }

    pub fn vec(&self, owner_size: TetraVec2, window_size: (i32, i32)) -> TetraVec2 {
        let x = match self.x {
            Horizontal::ByLeft { x } => x,
            Horizontal::ByCenter { x } => x - owner_size.x / 2.0,
            Horizontal::ByRight { x } => x - owner_size.x,
            Horizontal::AtWindowCenter { offset } => {
                (window_size.0 / 2) as f32 - (owner_size.x / 2.0) + offset
            }
            Horizontal::AtWindowRight { offset } => window_size.0 as f32 - owner_size.x + offset,
        };
        let y = match self.y {
            Vertical::ByTop { y } => y,
            Vertical::ByCenter { y } => y - owner_size.y / 2.0,
            Vertical::ByBottom { y } => y - owner_size.y,
            Vertical::AtWindowCenter { offset } => {
                (window_size.1 / 2) as f32 - (owner_size.y / 2.0) + offset
            }
            Vertical::AtWindowBottom { offset } => window_size.1 as f32 - owner_size.y + offset,
        };
        TetraVec2::new(x, y)
    }
}
