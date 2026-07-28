use heapless::VecView;
use pixel8::{logf, physics::Kinetic, plume::Explosion, Context, Graphics, SpriteId};

use crate::CartState;

/// What the game makes of an entity, on top of the movement `Kinetic` lends it.
///
/// Where a thing is, what it covers and whether two of them have run into each other are the
/// SDK's; this is what airwolf itself has to say about one: which sprite it wears, which side it
/// is on, and what a hit costs it.
pub trait Entity: Kinetic + 'static {
    /// The sprite to draw. How big it is, is the rectangle it hands the SDK.
    fn sprite(&self) -> SpriteId;
    fn entity_type(&self) -> Type;

    /// Wether this entity is still alive.
    fn alive(&self) -> bool;
    fn alive_mut(&mut self) -> &mut bool;

    fn update(&mut self, ctx: &mut Context, state: &CartState);

    fn draw(&self, gfx: &mut Graphics, state: &CartState) {
        self.draw_default(gfx, state);
    }

    fn draw_default(&self, gfx: &mut Graphics, _state: &CartState) {
        let bounds = self.bounds();

        gfx.sprite_ext(
            self.sprite(),
            bounds.x(),
            bounds.y(),
            bounds.width(),
            bounds.height(),
            false,
            false,
        )
        .unwrap();
    }

    /// Returns `true` if the entity is outside the screen.
    fn outside(&self) -> bool {
        !self.bounds().on_screen()
    }

    /// Check for collision and act on it.
    fn handle_collision(
        &mut self,
        other: &mut dyn Entity,
        ctx: &mut Context,
        explosions: &mut VecView<Explosion>,
    ) {
        if !self.alive() || !other.alive() || !self.overlaps(other.bounds()) {
            return;
        }

        self.hit(ctx, explosions);
        other.hit(ctx, explosions);
    }

    fn hit(&mut self, ctx: &mut Context, explosions: &mut VecView<Explosion>);

    fn destroy(&mut self, ctx: &mut Context, explosions: &mut VecView<Explosion>) {
        *self.alive_mut() = false;
        let (x, y) = self.body().draw_pos();
        explosions.push(Explosion::new(x, y)).unwrap_or_else(|_| {
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
}
