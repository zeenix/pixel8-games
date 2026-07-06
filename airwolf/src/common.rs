use core::num::NonZeroU16;

use pixel8::SpriteId;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

impl From<(i16, i16)> for Position {
    fn from(value: (i16, i16)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: NonZeroU16,
    pub height: NonZeroU16,
}

impl Size {
    /// Doesn't check the dimensions to be non-zero and hence unsafe.
    pub const unsafe fn new_unchecked(width: u16, height: u16) -> Self {
        Self {
            width: NonZeroU16::new_unchecked(width),
            height: NonZeroU16::new_unchecked(height),
        }
    }
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

    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}
