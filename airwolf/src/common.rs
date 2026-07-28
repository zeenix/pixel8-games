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
