use rico8::SpriteId;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: u8,
    pub height: u8,
}

#[derive(Debug, Clone, Copy)]
pub struct Sprite {
    pub id: SpriteId,
    pub size: Size,
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,

    LeftUp,
    LeftDown,
    RightUp,
    RightDown,
}
