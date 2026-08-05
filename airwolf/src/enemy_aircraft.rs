use core::num::NonZeroU8;

use heapless::VecView;
use pixel8::{
    physics::{Bounds, Contacts, Kinetic, Velocity},
    plume::Explosion,
    BitFlags, Body, Context, SfxId, SpriteFlag, SpriteId, SCREEN_HEIGHT, SCREEN_WIDTH,
};

use crate::{
    common::{Position, FRIENDLY_SHOT, LADY},
    entity::{self, Entity},
    rotor::Rotor,
    shooter::{BulletProps, Shooter},
    CartState, Scene,
};

#[derive(Debug)]
pub struct EnemyAircraft {
    body: Body,
    velocity: Velocity,
    /// What the world's last step ran into: a shot of ours, or the lady herself.
    contacts: Contacts,
    main_rotor: Rotor,
    tail_rotor: Rotor,
    last_bullet: f32,
    alive: bool,
    /// Set the update a shot of ours took this aircraft down, and never on the ram. The aircraft
    /// is the only thing that knows what killed it, and the score is the cart's, so this is how
    /// the one reaches the other — read in the retain pass that drops it, which happens exactly
    /// once.
    died_to_shot: bool,
}

impl EnemyAircraft {
    pub fn new(ctx: &mut Context) -> Self {
        let x = ctx.random(0.0..SCREEN_WIDTH as f32);
        Self {
            body: Body::new(x, STARTING_Y),
            velocity: Velocity::default(),
            contacts: Contacts::default(),
            main_rotor: Rotor::new(MAIN_ROTOR_OFFSET, MAIN_ROTOR_LENGTH),
            tail_rotor: Rotor::new(TAIL_ROTOR_OFFSET, TAIL_ROTOR_LENGTH),
            last_bullet: 0.0,
            alive: true,
            died_to_shot: false,
        }
    }

    /// Whether this aircraft was shot down, as opposed to rammed or still flying.
    pub fn died_to_shot(&self) -> bool {
        self.died_to_shot
    }

    fn chase(&mut self, state: &CartState) {
        let (x, _) = self.body.draw_pos();

        // Enemy aircraft just moves slowly down the screen but horizontally towards the player.
        let dx = if x < state.protoganist_pos.x {
            SPEED
        } else if x > state.protoganist_pos.x {
            -SPEED
        } else {
            0.0
        };

        self.velocity = Velocity::new(dx, SPEED);
    }
}

impl Shooter for EnemyAircraft {
    /// The bullet properties.
    fn bullet_props(&self) -> BulletProps {
        BulletProps {
            x_offset: 3.0,
            y_offset: 4.0,
            interval: 1.0,
        }
    }

    fn last_bullet(&self) -> f32 {
        self.last_bullet
    }

    fn reset_last_bullet(&mut self, ctx: &Context) {
        self.last_bullet = ctx.time();
    }
}

impl Kinetic for EnemyAircraft {
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
        Bounds::of(&self.body, WIDTH, HEIGHT)
    }

    /// The cell it is drawn from, whose `AIRCRAFT` flag is what the lady is told she met when one
    /// of these flies into her, and what our shots are spent on.
    fn sprite(&self) -> Option<SpriteId> {
        Some(SPRITE_ID)
    }

    /// What can end it: our shots, and the lady it flies into. Its own side's shots fill the sky
    /// around it and never concern it.
    fn heeds(&self) -> BitFlags<SpriteFlag> {
        FRIENDLY_SHOT | LADY
    }
}

impl Entity for EnemyAircraft {
    fn entity_type(&self) -> entity::Type {
        entity::Type::Enemy
    }

    fn alive(&self) -> bool {
        self.alive
    }
    fn alive_mut(&mut self) -> &mut bool {
        &mut self.alive
    }

    fn steer(&mut self, _ctx: &mut Context, state: &CartState) {
        if matches!(state.scene, Scene::Game { .. }) {
            self.chase(state);
        } else {
            // Whatever is in the air when the game ends hangs there.
            self.velocity = Velocity::default();
        }
    }

    fn react(&mut self, ctx: &mut Context, explosions: &mut VecView<Explosion>) {
        if self.contacts.touches(FRIENDLY_SHOT) {
            self.died_to_shot = true;
            self.destroy(ctx, explosions);
            ctx.sfx(DESTROY_SFX);
        } else if self.contacts.touches(LADY) {
            // The ram, which has already cost her the same.
            self.destroy(ctx, explosions);
            ctx.sfx(DESTROY_SFX);
        }

        let pos = self.body.draw_pos().into();
        self.main_rotor.update(pos);
        self.tail_rotor.update(pos);
    }

    fn draw(&self, gfx: &mut pixel8::Graphics, state: &CartState) {
        self.draw_default(gfx, state);

        self.main_rotor.draw(gfx);
        self.tail_rotor.draw(gfx);
    }

    // Override the "outside" definition since the aircraft is spawned above the screen.
    fn outside(&self) -> bool {
        let bounds = self.bounds();

        bounds.x() >= SCREEN_WIDTH as i16
            || bounds.right() <= 0
            || bounds.y() >= SCREEN_HEIGHT as i16
    }
}

const SPRITE_ID: SpriteId = SpriteId(32);
const WIDTH: u16 = 6;
const HEIGHT: u16 = 8;
const MAIN_ROTOR_OFFSET: Position = Position { x: 2, y: 4 };
const MAIN_ROTOR_LENGTH: NonZeroU8 = NonZeroU8::new(3).unwrap();
const TAIL_ROTOR_OFFSET: Position = Position { x: 2, y: 0 };
const TAIL_ROTOR_LENGTH: NonZeroU8 = NonZeroU8::new(2).unwrap();
const STARTING_Y: f32 = -8.0;
const SPEED: f32 = 0.3;
const DESTROY_SFX: SfxId = SfxId::new(2).unwrap();
