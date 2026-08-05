use pixel8::SpriteFlag;

/// The lady, and everything a hit on her costs.
pub const LADY: SpriteFlag = SpriteFlag::Flag0;
/// An enemy aircraft, whichever one it is.
pub const AIRCRAFT: SpriteFlag = SpriteFlag::Flag1;
/// A shot of ours, going up the screen.
pub const FRIENDLY_SHOT: SpriteFlag = SpriteFlag::Flag2;
/// And one of theirs, coming down it.
pub const ENEMY_SHOT: SpriteFlag = SpriteFlag::Flag3;

// The four flags above are written on the sprites each side of the fight is drawn from, in the
// sprite editor, and they are the whole of airwolf's collision: the world reads them off the cell
// each entity wears, writes what it met into that entity's own contacts, and each of them settles
// for itself what the flags it met cost it. Nothing here walks a pair.
//
// None of them is ever named in a `solid()`. Nothing in this game shoves anything — a bullet flies
// through the aircraft it kills, and the lady is never pushed off her hover — so every one of them
// is a sensor and the step only ever reports. And nobody asks after its own flag: each entity asks
// only after the kinds that can hurt it.

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
