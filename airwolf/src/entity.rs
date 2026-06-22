use core::any::Any;

use rico8::{Body, Context, Graphics, SCREEN_H};

use crate::{
    common::{Direction, Size, Sprite},
    Scene,
};

pub trait Entity: 'static {
    fn body(&self) -> Body;
    fn body_mut(&mut self) -> &mut Body;
    fn sprite(&self) -> Sprite;
    fn entity_type(&self) -> Type;

    /// Returns `true` if the entity is outside the screen.
    fn outside(&self) -> bool {
        let (x, y) = self.body().draw_pos();

        x >= SCREEN_H as f32 || x < 0.0 || y >= SCREEN_H as f32 || y < 0.0
    }

    fn update(&mut self, ctx: &mut Context, scene: &Scene);

    fn draw(&self, gfx: &mut Graphics, scene: &Scene) {
        self.draw_default(gfx, scene);
    }

    fn draw_default(&self, gfx: &mut Graphics, scene: &Scene) {
        let (x, y) = self.body().draw_pos();
        let size = self.sprite().size_in_blocks();

        gfx.sprite_ext(
            self.sprite().id,
            x,
            y,
            size.width,
            size.height,
            false,
            false,
        );
    }

    fn go(&mut self, dir: Direction, distance: f32) {
        self.go_default(dir, distance);
    }

    fn go_default(&mut self, dir: Direction, distance: f32) {
        let body = self.body_mut();
        match dir {
            Direction::Left => body.move_by(-distance, 0.0),
            Direction::Right => body.move_by(distance, 0.0),
            Direction::Up => body.move_by(0.0, -distance),
            Direction::Down => body.move_by(0.0, distance),
            Direction::UpLeft => body.move_by(-distance, -distance),
            Direction::DownLeft => body.move_by(-distance, distance),
            Direction::UpRight => body.move_by(distance, -distance),
            Direction::DownRight => body.move_by(distance, distance),
        }

        // TODO: Handle collision.
    }

    fn collided(&self, other: &dyn Entity) -> bool {
        // Skip self and collision with explosion.
        if self.type_id() == other.type_id() && self.entity_type() == Type::Explosion {
            return false;
        }

        let (our_x, our_y) = self.body().draw_pos();
        let (other_x, other_y) = other.body().draw_pos();
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
