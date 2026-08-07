use core::num::NonZeroU8;

use heapless::VecView;
use pixel8::{
    physics::{Member, Velocity},
    plume::Explosion,
    Button, Color, Context, SfxId, SpriteId, SCREEN_HEIGHT, SCREEN_WIDTH,
};

use crate::{
    common::{Position, AIRCRAFT, ENEMY_SHOT},
    entity::{self, Entity},
    rotor::Rotor,
    shooter::{BulletProps, Shooter},
    CartState, Scene, Sky,
};

#[derive(Debug)]
pub struct TheLady {
    /// Her seat: where she is, how fast, and what her last step ran into — an aircraft, or a shot
    /// of theirs. The world writes it and `react` reads it, in the same update.
    member: Member,
    main_rotor: Rotor,
    tail_rotor: Rotor,
    last_bullet: f32,
    alive: bool,
    /// Where she last drew, cached the moment she dies. Dead, her seat is retired — nothing meets
    /// her wreck — but an aircraft still chases wherever she went down, and that is read off here
    /// rather than off a seat that no longer exists.
    last_pos: (i16, i16),
}

impl TheLady {
    /// The lady as the cart ships: everything about her settled except where she stands, which is
    /// the world's and needs a seat. A constant, so she can sit in the cart's opening state as
    /// written rather than built.
    pub const fn waiting() -> Self {
        Self {
            member: Member::NOBODY,
            main_rotor: Rotor::new(MAIN_ROTOR_OFFSET, MAIN_ROTOR_LENGTH),
            tail_rotor: Rotor::new(TAIL_ROTOR_OFFSET, TAIL_ROTOR_LENGTH),
            last_bullet: 0.0,
            alive: true,
            last_pos: (0, 0),
        }
    }

    /// Seats her in `world`, at boot and at the start of every run after.
    pub fn new(world: &mut Sky) -> Self {
        let member = world
            .enlist(
                STARTING_POSITION.x as f32,
                STARTING_POSITION.y as f32,
                WIDTH,
                HEIGHT,
            )
            .expect("a seat for the lady")
            // The cell she is drawn from, whose `LADY` flag is what an aircraft hunting her —
            // and every shot aimed at her — is told it met.
            .wearing(SPRITE_ID)
            // A ram or a shot, which is the whole of what can end her — the same pair `react`
            // asks about. Our own shots stream past her every update and are none of her
            // business.
            .heeding(AIRCRAFT | ENEMY_SHOT)
            .member();

        Self {
            member,
            main_rotor: Rotor::new(MAIN_ROTOR_OFFSET, MAIN_ROTOR_LENGTH),
            tail_rotor: Rotor::new(TAIL_ROTOR_OFFSET, TAIL_ROTOR_LENGTH),
            last_bullet: 0.0,
            alive: true,
            last_pos: (0, 0),
        }
    }

    /// Where she draws: her seat's, while she is in the cast, and the position she died at once
    /// she has been retired out of it.
    pub fn draw_pos(&self, world: &Sky) -> (i16, i16) {
        if self.alive {
            world.draw_pos(self.member)
        } else {
            self.last_pos
        }
    }

    fn move_it(&mut self, ctx: &mut Context, world: &mut Sky) {
        let bounds = world.bounds(self.member);
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

        world.set_velocity(self.member, velocity);
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

impl Entity for TheLady {
    fn entity_type(&self) -> entity::Type {
        entity::Type::Protoganist
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

    fn steer(&mut self, ctx: &mut Context, state: &CartState, world: &mut Sky) {
        if !self.alive {
            return;
        }
        if matches!(state.scene, Scene::Game { .. }) {
            self.move_it(ctx, world);
        } else {
            // Nothing to fly her with between games; she holds her hover.
            world.set_velocity(self.member, Velocity::default());
        }
    }

    fn react(&mut self, ctx: &mut Context, world: &mut Sky, explosions: &mut VecView<Explosion>) {
        if !self.alive {
            return;
        }

        // Read before `destroy` can retire the seat, so the rotors still have somewhere to draw
        // themselves from on the update she dies, and so a dead lady's last position survives the
        // retiring for `draw_pos` to hand back afterwards.
        let pos = world.draw_pos(self.member);

        // A ram or a shot; either is the end of her, and she asks after nothing else.
        if world.contacts(self.member).touches(AIRCRAFT | ENEMY_SHOT) {
            self.last_pos = pos;
            self.destroy(ctx, world, explosions);
            ctx.sfx(DESTROY_SFX);
        }

        self.main_rotor.update(pos.into());
        self.tail_rotor.update(pos.into());
    }

    fn draw(&self, gfx: &mut pixel8::Graphics, state: &CartState, world: &Sky) {
        if !self.alive {
            return;
        }

        gfx.set_transparent_color(Color::BLACK, false);
        gfx.set_transparent_color(Color::DARK_GREY, true);
        self.draw_default(gfx, state, world);
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
