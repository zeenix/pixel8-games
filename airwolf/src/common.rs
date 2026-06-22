use rico8::SpriteId;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl From<(f32, f32)> for Position {
    fn from(value: (f32, f32)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Sprite {
    pub id: SpriteId,
    pub size: Size,
}

impl Sprite {
    pub fn size_in_blocks(&self) -> Size {
        Size {
            width: self.size.width / 8.0,
            height: self.size.height / 8.0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,

    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}
