use heapless::VecView;
use pixel8::{physics::Member, plume::Explosion, Context, SfxId, SpriteId};

use crate::{
    common::{AIRCRAFT, LADY},
    entity::{self, Entity},
    Sky,
};

#[derive(Debug)]
pub struct Bullet {
    /// The bullet's seat: where it is, and the one thing it ever asks the world afterwards —
    /// whether it has arrived at what it was fired at.
    member: Member,
    entity_type: entity::Type,
    alive: bool,
}

impl Bullet {
    pub fn new_friendly(x: f32, y: f32, ctx: &mut Context, world: &mut Sky) -> Option<Self> {
        Self::new(x, y, entity::Type::FriendlyBullet, ctx, world)
    }

    pub fn new_enemy(x: f32, y: f32, ctx: &mut Context, world: &mut Sky) -> Option<Self> {
        Self::new(x, y, entity::Type::EnemyBullet, ctx, world)
    }

    fn new(
        x: f32,
        y: f32,
        entity_type: entity::Type,
        ctx: &mut Context,
        world: &mut Sky,
    ) -> Option<Self> {
        ctx.sfx(SFX_ID);

        let is_enemy = matches!(entity_type, entity::Type::EnemyBullet);
        // A bullet is aimed once and never steered again: the enemy's goes down the screen and
        // ours up it, at the same pace, until one of them hits something or runs off the edge.
        let dy = if is_enemy { SPEED } else { -SPEED };
        let (width, height) = if is_enemy { ENEMY_SIZE } else { FRIENDLY_SIZE };
        // The cell it is drawn from, and with it the flag that tells the side it is fired at that
        // a shot has arrived.
        let sprite = if is_enemy {
            ENEMY_SPRITE_ID
        } else {
            FRIENDLY_SPRITE_ID
        };
        // The one thing a shot is spent on — the same target `react` asks about below, said once
        // here so the world never works out anything else this shot flew past.
        let heeds = if is_enemy { LADY } else { AIRCRAFT };

        let member = world
            .enlist(x, y, width, height)?
            .moving(0.0, dy)
            .wearing(sprite)
            .heeding(heeds)
            .member();

        Some(Self {
            member,
            entity_type,
            alive: true,
        })
    }
}

impl Entity for Bullet {
    fn entity_type(&self) -> entity::Type {
        self.entity_type
    }

    fn member(&self) -> Member {
        self.member
    }

    fn alive(&self) -> bool {
        self.alive
    }
    fn alive_mut(&mut self) -> &mut bool {
        &mut self.alive
    }

    fn react(&mut self, ctx: &mut Context, world: &mut Sky, explosions: &mut VecView<Explosion>) {
        // Each side's shot is spent on the other side's target and on nothing else — not on its
        // own kind, and not on the thing that fired it.
        let target = if self.is_enemy() { LADY } else { AIRCRAFT };
        if world.contacts(self.member).touches(target) {
            self.destroy(ctx, world, explosions);
        }
    }
}

const FRIENDLY_SPRITE_ID: SpriteId = SpriteId(64);
const FRIENDLY_SIZE: (u16, u16) = (8, 8);
const ENEMY_SPRITE_ID: SpriteId = SpriteId(65);
const ENEMY_SIZE: (u16, u16) = (1, 7);
const SPEED: f32 = 2.0;
const SFX_ID: SfxId = SfxId::new(0).unwrap();
