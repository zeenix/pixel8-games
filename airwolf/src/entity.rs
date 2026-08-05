use heapless::VecView;
use pixel8::{logf, physics::Kinetic, plume::Explosion, Context, Graphics};

use crate::CartState;

/// What the game makes of an entity, on top of the movement `Kinetic` lends it.
///
/// Where a thing is, what it covers and what it has just run into are the SDK's. This is what
/// airwolf itself has to say about one — which side of the fight it is on, where it means to go
/// this update, and what the world's report of that step costs it.
pub trait Entity: Kinetic + 'static {
    fn entity_type(&self) -> Type;

    /// Wether this entity is still alive.
    fn alive(&self) -> bool;
    fn alive_mut(&mut self) -> &mut bool;

    /// Where the entity means to go this update, written into its own velocity before the world
    /// runs. Nothing here moves anything: the world does that for the whole cast at once.
    ///
    /// A bullet is aimed once and never steered again, which is what this default is.
    fn steer(&mut self, _ctx: &mut Context, _state: &CartState) {}

    /// What the entity makes of what its step met, read from its own contacts.
    ///
    /// The world writes them and this reads them, the same update, so a shot that lands and the
    /// target it lands on settle it between them before the frame is drawn. `explosions` is the
    /// scene's, handed round rather than held, because dying is the one thing an entity does that
    /// shows up outside itself.
    fn react(&mut self, ctx: &mut Context, explosions: &mut VecView<Explosion>);

    fn draw(&self, gfx: &mut Graphics, state: &CartState) {
        self.draw_default(gfx, state);
    }

    fn draw_default(&self, gfx: &mut Graphics, _state: &CartState) {
        let bounds = self.bounds();
        // The cell an entity wears is the cell it is drawn from: what the player sees and what
        // everybody else meets are the one sprite, so everything here has one.
        let Some(sprite) = self.sprite() else {
            return;
        };

        gfx.sprite_ext(
            sprite,
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

    /// Die, and leave a blast where it was.
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
