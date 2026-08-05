use core::num::NonZeroU8;

use heapless::VecView;
use pixel8::{
    physics::{Bounds, Contacts, Kinetic, Velocity},
    plume::Explosion,
    BitFlags, Body, Button, Color, Context, SfxId, SpriteFlag, SpriteId, SCREEN_HEIGHT,
    SCREEN_WIDTH,
};

use crate::{
    common::{Position, AIRCRAFT, ENEMY_SHOT},
    entity::{self, Entity},
    rotor::Rotor,
    shooter::{BulletProps, Shooter},
    CartState, Scene,
};

#[derive(Debug)]
pub struct TheLady {
    body: Body,
    velocity: Velocity,
    /// What the world's last step ran into: an aircraft, or a shot of theirs. The world writes it
    /// and she reads it, in the same update.
    contacts: Contacts,
    main_rotor: Rotor,
    tail_rotor: Rotor,
    last_bullet: f32,
    alive: bool,
}

impl TheLady {
    pub fn new() -> Self {
        Self {
            body: Body::new(STARTING_POSITION.x as f32, STARTING_POSITION.y as f32),
            velocity: Velocity::default(),
            contacts: Contacts::default(),
            main_rotor: Rotor::new(MAIN_ROTOR_OFFSET, MAIN_ROTOR_LENGTH),
            tail_rotor: Rotor::new(TAIL_ROTOR_OFFSET, TAIL_ROTOR_LENGTH),
            last_bullet: 0.0,
            alive: true,
        }
    }

    fn move_it(&mut self, ctx: &mut Context) {
        let bounds = self.bounds();
        let buttons = ctx.buttons_down();

        // An axis at a time, and each of them only while there is screen left on that side, so a
        // diagonal held into an edge carries on along it.
        let mut velocity = Velocity::default();
        if buttons.contains(Button::Left) && bounds.x() > -1 {
            velocity.dx -= SPEED;
        }
        if buttons.contains(Button::Right) && bounds.right() < SCREEN_WIDTH as i16 - 2 {
            velocity.dx += SPEED;
        }
        if buttons.contains(Button::Up) && bounds.y() > 0 {
            velocity.dy -= SPEED;
        }
        if buttons.contains(Button::Down) && bounds.bottom() < SCREEN_HEIGHT as i16 {
            velocity.dy += SPEED;
        }

        self.velocity = velocity;
    }
}

impl Shooter for TheLady {
    /// The bullet properties.
    fn bullet_props(&self) -> BulletProps {
        BulletProps {
            x_offset: 0.0,
            y_offset: -1.0,
            interval: 0.4,
        }
    }

    fn last_bullet(&self) -> f32 {
        self.last_bullet
    }

    fn reset_last_bullet(&mut self, ctx: &Context) {
        self.last_bullet = ctx.time();
    }
}

impl Kinetic for TheLady {
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

    /// The cell she is drawn from, whose `LADY` flag is what an aircraft hunting her — and every
    /// shot aimed at her — is told it met.
    fn sprite(&self) -> Option<SpriteId> {
        Some(SPRITE_ID)
    }

    /// A ram or a shot, which is the whole of what can end her — the same pair `react` asks
    /// about. Our own shots stream past her every update and are none of her business.
    fn heeds(&self) -> BitFlags<SpriteFlag> {
        AIRCRAFT | ENEMY_SHOT
    }
}

impl Entity for TheLady {
    fn entity_type(&self) -> entity::Type {
        entity::Type::Protoganist
    }

    fn alive(&self) -> bool {
        self.alive
    }
    fn alive_mut(&mut self) -> &mut bool {
        &mut self.alive
    }

    fn steer(&mut self, ctx: &mut Context, state: &CartState) {
        if !self.alive {
            return;
        }
        if matches!(state.scene, Scene::Game { .. }) {
            self.move_it(ctx);
        } else {
            // Nothing to fly her with between games; she holds her hover.
            self.velocity = Velocity::default();
        }
    }

    fn react(&mut self, ctx: &mut Context, explosions: &mut VecView<Explosion>) {
        if !self.alive {
            return;
        }
        // A ram or a shot; either is the end of her, and she asks after nothing else.
        if self.contacts.touches(AIRCRAFT | ENEMY_SHOT) {
            self.destroy(ctx, explosions);
            ctx.sfx(DESTROY_SFX);
        }

        let pos = self.body.draw_pos().into();
        self.main_rotor.update(pos);
        self.tail_rotor.update(pos);
    }

    fn draw(&self, gfx: &mut pixel8::Graphics, state: &CartState) {
        if !self.alive {
            return;
        }

        gfx.set_transparent_color(Color::BLACK, false);
        gfx.set_transparent_color(Color::DARK_GREY, true);
        self.draw_default(gfx, state);
        gfx.reset_transparency();

        self.main_rotor.draw(gfx);
        self.tail_rotor.draw(gfx);
    }
}

const SPRITE_ID: SpriteId = SpriteId(1);
const WIDTH: u16 = 8;
const HEIGHT: u16 = 8;
const MAIN_ROTOR_OFFSET: Position = Position { x: 4, y: 3 };
const MAIN_ROTOR_LENGTH: NonZeroU8 = NonZeroU8::new(3).unwrap();
const TAIL_ROTOR_OFFSET: Position = Position { x: 4, y: 7 };
const TAIL_ROTOR_LENGTH: NonZeroU8 = NonZeroU8::new(2).unwrap();
const STARTING_POSITION: Position = Position { x: 63, y: 111 };
const SPEED: f32 = 0.7;
const DESTROY_SFX: SfxId = SfxId::new(1).unwrap();
