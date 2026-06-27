use core::any::Any;

use heapless::VecView;
use rico8::{logf, Body, Context, Graphics, SCREEN_HEIGHT};

use crate::{
    common::{Direction, Size, Sprite},
    explosion::Explosion,
    CartState,
};

pub trait Entity: 'static {
    fn body(&self) -> Body;
    fn body_mut(&mut self) -> &mut Body;
    fn sprite(&self) -> Sprite;
    fn entity_type(&self) -> Type;

    /// Returns `true` if the entity is outside the screen.
    fn outside(&self) -> bool {
        let (x, y) = self.body().draw_pos();
        let (x, y) = (x as f32, y as f32);
        let size = self.sprite().size;

        x >= SCREEN_HEIGHT as f32
            || (x + size.width) < 0.0
            || y >= SCREEN_HEIGHT as f32
            || (y + size.height) < 0.0
    }

    /// Wether this entity is still alive.
    fn alive(&self) -> bool;
    fn alive_mut(&mut self) -> &mut bool;

    fn update(&mut self, ctx: &mut Context, state: &CartState);

    fn draw(&self, gfx: &mut Graphics, state: &CartState) {
        self.draw_default(gfx, state);
    }

    fn draw_default(&self, gfx: &mut Graphics, _state: &CartState) {
        let (x, y) = self.body().draw_pos();
        let size = self.sprite().size;
        gfx.sprite_ext(
            self.sprite().id,
            x,
            y,
            size.width as u16,
            size.height as u16,
            false,
            false,
        )
        .unwrap();
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
    }

    /// Check for collision and act on it.
    fn handle_collision(
        &mut self,
        other: &mut dyn Entity,
        ctx: &mut Context,
        explosions: &mut VecView<Explosion>,
    ) {
        if !self.alive() || !other.alive() || !self.collided(other) {
            return;
        }

        self.hit(ctx, explosions);
        other.hit(ctx, explosions);
    }

    fn collided(&self, other: &dyn Entity) -> bool {
        // Skip self and collision with explosion.
        if self.type_id() == other.type_id() && self.entity_type() == Type::Explosion {
            return false;
        }

        let (our_x, our_y) = self.body().draw_pos();
        let (our_x, our_y) = (our_x as f32, our_y as f32);
        let (other_x, other_y) = other.body().draw_pos();
        let (other_x, other_y) = (other_x as f32, other_y as f32);
        let Size {
            width: our_width,
            height: our_height,
        } = self.sprite().size;
        let Size {
            width: other_width,
            height: other_height,
        } = other.sprite().size;

        our_x < other_x + other_width
            && our_x + our_width > other_x
            && our_y < other_y + other_height
            && our_y + our_height > other_y
    }

    fn hit(&mut self, ctx: &mut Context, explosions: &mut VecView<Explosion>);

    fn destroy(&mut self, ctx: &mut Context, explosions: &mut VecView<Explosion>) {
        *self.alive_mut() = false;
        explosions
            .push(Explosion::new(self.body().pos().into(), ctx))
            .unwrap_or_else(|_| {
                logf!(ctx, "Err: Too many explosions: {}", super::MAX_EXPLOSIONS);
            });
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
