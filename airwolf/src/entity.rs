use heapless::VecView;
use pixel8::{logf, physics::Member, plume::Explosion, Context, Graphics};

use crate::{CartState, Sky};

/// What the game makes of an entity, on top of the seat the world gives it.
///
/// Where a thing is, what it covers and what it has just run into are the world's, asked for with
/// [`member`](Self::member). This is what airwolf itself has to say about one — which side of the
/// fight it is on, where it means to go this update, and what the world's report of that step costs
/// it.
pub trait Entity: 'static {
    fn entity_type(&self) -> Type;

    /// This entity's seat in the world: where it is, how fast, what it covers and what its last
    /// step ran into are all asked for with this.
    fn member(&self) -> Member;

    /// Wether this entity is still alive.
    fn alive(&self) -> bool;
    fn alive_mut(&mut self) -> &mut bool;

    /// Where the entity means to go this update, written into its own velocity in the world before
    /// the world runs. Nothing here moves anything: the world does that for the whole cast at once.
    ///
    /// A bullet is aimed once and never steered again, which is what this default is.
    fn steer(&mut self, _ctx: &mut Context, _state: &CartState, _world: &mut Sky) {}

    /// What the entity makes of what its step met, read from the world's own contacts.
    ///
    /// The world writes them and this reads them, the same update, so a shot that lands and the
    /// target it lands on settle it between them before the frame is drawn. `explosions` is the
    /// scene's, handed round rather than held, because dying is the one thing an entity does that
    /// shows up outside itself. `world` is `&mut` because dying is answered here too — see
    /// [`destroy`](Self::destroy).
    fn react(&mut self, ctx: &mut Context, world: &mut Sky, explosions: &mut VecView<Explosion>);

    fn draw(&self, gfx: &mut Graphics, state: &CartState, world: &Sky) {
        self.draw_default(gfx, state, world);
    }

    fn draw_default(&self, gfx: &mut Graphics, _state: &CartState, world: &Sky) {
        let bounds = world.bounds(self.member());
        // The cell an entity wears is the cell it is drawn from, and the world owns it: what the
        // player sees and what everybody else meets can never be two different sprites. Everything
        // here wears one.
        let Some(sprite) = world.sprite(self.member()) else {
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
    fn outside(&self, world: &Sky) -> bool {
        !world.bounds(self.member()).on_screen()
    }

    /// Die, leave a blast where it was, and give the seat back.
    ///
    /// Retiring here, rather than in a retain pass later, is what keeps a dead entity out of the
    /// cast from this update on: nothing enlisted after this meets it, wherever its wreck sits.
    fn destroy(&mut self, ctx: &mut Context, world: &mut Sky, explosions: &mut VecView<Explosion>) {
        *self.alive_mut() = false;
        let (x, y) = world.draw_pos(self.member());
        world.retire(self.member());
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
