use heapless::VecView;
use pixel8::{
    physics::{Bounds, Kinetic, Velocity},
    plume::Explosion,
    Body, Context, SfxId, SpriteId,
};

use crate::{
    entity::{self, Entity},
    CartState,
};

#[derive(Debug)]
pub struct Bullet {
    body: Body,
    velocity: Velocity,
    entity_type: entity::Type,
    alive: bool,
}

impl Bullet {
    pub fn new_friendly(x: f32, y: f32, ctx: &mut Context) -> Self {
        Self::new(x, y, entity::Type::FriendlyBullet, ctx)
    }

    pub fn new_enemy(x: f32, y: f32, ctx: &mut Context) -> Self {
        Self::new(x, y, entity::Type::EnemyBullet, ctx)
    }

    fn new(x: f32, y: f32, entity_type: entity::Type, ctx: &mut Context) -> Self {
        ctx.sfx(SFX_ID);

        // A bullet is aimed once and never steered again: the enemy's goes down the screen and
        // ours up it, at the same pace, until one of them hits something or runs off the edge.
        let dy = match entity_type {
            entity::Type::EnemyBullet => SPEED,
            _ => -SPEED,
        };

        Self {
            body: Body::new(x, y),
            velocity: Velocity::new(0.0, dy),
            entity_type,
            alive: true,
        }
    }
}

impl Kinetic for Bullet {
    fn body(&self) -> &Body {
        &self.body
    }

    fn body_mut(&mut self) -> &mut Body {
        &mut self.body
    }

    fn velocity_mut(&mut self) -> &mut Velocity {
        &mut self.velocity
    }

    fn bounds(&self) -> Bounds {
        let (width, height) = if self.is_enemy() {
            ENEMY_SIZE
        } else {
            FRIENDLY_SIZE
        };

        Bounds::of(&self.body, width, height)
    }
}

impl Entity for Bullet {
    fn sprite(&self) -> SpriteId {
        if self.is_enemy() {
            ENEMY_SPRITE_ID
        } else {
            FRIENDLY_SPRITE_ID
        }
    }

    fn entity_type(&self) -> entity::Type {
        self.entity_type
    }

    fn alive(&self) -> bool {
        self.alive
    }
    fn alive_mut(&mut self) -> &mut bool {
        &mut self.alive
    }

    fn update(&mut self, ctx: &mut Context, _state: &CartState) {
        self.step(ctx, &[]);
    }

    fn hit(&mut self, ctx: &mut Context, explosions: &mut VecView<Explosion>) {
        self.destroy(ctx, explosions);
    }
}

const FRIENDLY_SPRITE_ID: SpriteId = SpriteId(64);
const FRIENDLY_SIZE: (u16, u16) = (8, 8);
const ENEMY_SPRITE_ID: SpriteId = SpriteId(65);
const ENEMY_SIZE: (u16, u16) = (1, 7);
const SPEED: f32 = 2.0;
const SFX_ID: SfxId = SfxId::new(0).unwrap();
