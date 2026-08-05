use heapless::VecView;
use pixel8::{
    physics::{Bounds, Contacts, Kinetic, Velocity},
    plume::Explosion,
    BitFlags, Body, Context, SfxId, SpriteFlag, SpriteId,
};

use crate::{
    common::{AIRCRAFT, LADY},
    entity::{self, Entity},
};

#[derive(Debug)]
pub struct Bullet {
    body: Body,
    velocity: Velocity,
    /// What the world's last step ran into, of which a shot asks after one thing only: whether it
    /// has arrived at what it was fired at.
    contacts: Contacts,
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
            contacts: Contacts::default(),
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

    fn contacts(&self) -> &Contacts {
        &self.contacts
    }

    fn contacts_mut(&mut self) -> &mut Contacts {
        &mut self.contacts
    }

    fn bounds(&self) -> Bounds {
        let (width, height) = if self.is_enemy() {
            ENEMY_SIZE
        } else {
            FRIENDLY_SIZE
        };

        Bounds::of(&self.body, width, height)
    }

    /// The cell it is drawn from, and with it the flag that tells the side it is fired at that a
    /// shot has arrived.
    fn sprite(&self) -> Option<SpriteId> {
        Some(if self.is_enemy() {
            ENEMY_SPRITE_ID
        } else {
            FRIENDLY_SPRITE_ID
        })
    }

    /// The one thing a shot is spent on — the same target `react` asks about below, said once here
    /// so the world never works out anything else this shot flew past.
    fn heeds(&self) -> BitFlags<SpriteFlag> {
        if self.is_enemy() { LADY } else { AIRCRAFT }.into()
    }
}

impl Entity for Bullet {
    fn entity_type(&self) -> entity::Type {
        self.entity_type
    }

    fn alive(&self) -> bool {
        self.alive
    }
    fn alive_mut(&mut self) -> &mut bool {
        &mut self.alive
    }

    fn react(&mut self, ctx: &mut Context, explosions: &mut VecView<Explosion>) {
        // Each side's shot is spent on the other side's target and on nothing else — not on its
        // own kind, and not on the thing that fired it.
        let target = if self.is_enemy() { LADY } else { AIRCRAFT };
        if self.contacts.touches(target) {
            self.destroy(ctx, explosions);
        }
    }
}

const FRIENDLY_SPRITE_ID: SpriteId = SpriteId(64);
const FRIENDLY_SIZE: (u16, u16) = (8, 8);
const ENEMY_SPRITE_ID: SpriteId = SpriteId(65);
const ENEMY_SIZE: (u16, u16) = (1, 7);
const SPEED: f32 = 2.0;
const SFX_ID: SfxId = SfxId::new(0).unwrap();
