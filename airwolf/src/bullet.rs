use heapless::VecView;
use rico8::{Body, Context, SfxId, SpriteId};

use crate::{
    common::{Direction, Size, Sprite},
    entity::{self, Entity},
    explosion::Explosion,
    CartState,
};

#[derive(Debug)]
pub struct Bullet {
    body: Body,
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

        Self {
            body: Body::new(x, y),
            entity_type,
            alive: true,
        }
    }
}

impl Entity for Bullet {
    fn body(&self) -> Body {
        self.body
    }

    fn body_mut(&mut self) -> &mut Body {
        &mut self.body
    }

    fn sprite(&self) -> Sprite {
        match self.entity_type {
            entity::Type::EnemyBullet => Sprite {
                id: ENEMY_SPRITE_ID,
                size: ENEMY_SIZE,
            },
            entity::Type::FriendlyBullet => Sprite {
                id: FRIENDLY_SPRITE_ID,
                size: FRIENDLY_SIZE,
            },
            _ => unreachable!("unknown bullet type"),
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

    fn update(&mut self, _ctx: &mut Context, _state: &CartState) {
        let dir = if self.is_enemy() {
            Direction::Down
        } else {
            Direction::Up
        };

        self.go(dir, SPEED);
    }

    fn hit(&mut self, ctx: &mut Context, explosions: &mut VecView<Explosion>) {
        self.destroy(ctx, explosions);
    }
}

const FRIENDLY_SPRITE_ID: SpriteId = SpriteId(64);
const FRIENDLY_SIZE: Size = Size {
    width: 8.0,
    height: 8.0,
};
const ENEMY_SPRITE_ID: SpriteId = SpriteId(65);
const ENEMY_SIZE: Size = Size {
    width: 1.0,
    height: 7.0,
};
const SPEED: f32 = 2.0;
const SFX_ID: SfxId = SfxId(0);
