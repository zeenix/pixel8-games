use core::any::Any;

use libm::floorf;
use rico8::Graphics;

use crate::common::{Direction, Position, Size, Sprite};

pub trait Entity: 'static {
    fn position(&self) -> Position;
    fn position_mut(&mut self) -> &mut Position;
    fn sprite(&self) -> Sprite;
    fn entity_type(&self) -> Type;

    /// Returns `true` if the entity is outside the screen after the update.
    fn update(&mut self) -> bool;

    fn draw(&self, gfx: &mut Graphics) {
        let pos = self.position();
        let size = self.sprite().size;

        // Ensure the entity is always at the middle of the pixel to avoid the
        // "cobblestone effect" in case of diagonal movements.
        let x = floorf(pos.x) + 0.5;
        let y = floorf(pos.y) + 0.5;

        gfx.spr_ext(
            self.sprite().id,
            x,
            y,
            size.width.into(),
            size.height.into(),
            false,
            false,
        );
    }

    fn go(&mut self, dir: Option<Direction>, distance: u8) {
        let Some(dir) = dir else {
            return;
        };
        let pos = self.position_mut();
        let distance: f32 = distance.into();
        match dir {
            Direction::Left => pos.x -= distance,
            Direction::Right => pos.x += distance,
            Direction::Up => pos.y -= distance,
            Direction::Down => pos.y += distance,
            Direction::LeftUp => {
                pos.x -= distance;
                pos.y -= distance;
            }
            Direction::LeftDown => {
                pos.x -= distance;
                pos.y += distance;
            }
            Direction::RightUp => {
                pos.x += distance;
                pos.y -= distance;
            }
            Direction::RightDown => {
                pos.x += distance;
                pos.y += distance;
            }
        }

        // TODO: Handle collision.
    }

    fn collided(&self, other: &dyn Entity) -> bool {
        // Skip self and collision with explosion.
        if self.type_id() == other.type_id() && self.entity_type() == Type::Explosion {
            return false;
        }

        let Position { x: our_x, y: our_y } = self.position();
        let Position {
            x: other_x,
            y: other_y,
        } = other.position();
        let Size {
            width: our_width,
            height: our_height,
        } = self.sprite().size;
        let (our_width, our_height) = (our_width as f32, our_height as f32);
        let Size {
            width: other_width,
            height: other_height,
        } = other.sprite().size;
        let (other_width, other_height) = (other_width as f32, other_height as f32);

        our_x < other_x + other_width
            && our_x + our_width > other_x
            && our_y < other_y + other_height
            && our_y + our_height > other_y
    }

    fn is_enemy(&self) -> bool {
        matches!(self.entity_type(), Type::Enemy | Type::EnemyBullet)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Type {
    Protoganist,
    Enemy,
    FriendlyBullet,
    EnemyBullet,
    Explosion,
}
