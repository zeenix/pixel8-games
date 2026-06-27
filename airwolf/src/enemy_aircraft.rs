use core::num::NonZeroU8;

use heapless::VecView;
use rico8::{Body, Context, SfxId, SpriteId, SCREEN_HEIGHT, SCREEN_WIDTH};

use crate::{
    common::{Direction, Position, Size, Sprite},
    entity::{self, Entity},
    explosion::Explosion,
    rotor::Rotor,
    shooter::{BulletProps, Shooter},
    CartState, Scene,
};

#[derive(Debug)]
pub struct EnemyAircraft {
    body: Body,
    main_rotor: Rotor,
    tail_rotor: Rotor,
    last_bullet: f32,
    alive: bool,
}

impl EnemyAircraft {
    pub fn new(ctx: &mut Context) -> Self {
        let x = ctx.random(0.0..SCREEN_WIDTH as f32);
        Self {
            body: Body::new(x, STARTING_Y),
            main_rotor: Rotor::new(MAIN_ROTOR_OFFSET, MAIN_ROTOR_LENGTH),
            tail_rotor: Rotor::new(TAIL_ROTOR_OFFSET, TAIL_ROTOR_LENGTH),
            last_bullet: 0.0,
            alive: true,
        }
    }

    fn move_it(&mut self, _ctx: &mut Context, state: &CartState) {
        let (x, _) = self.body.draw_pos();

        // Enemy aircraft just moves slowly down the screen but horizontally towards the player.
        let dir = if x < state.protoganist_pos.x {
            Direction::DownRight
        } else if x > state.protoganist_pos.x {
            Direction::DownLeft
        } else {
            Direction::Down
        };

        self.go(dir, SPEED);
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

impl Entity for EnemyAircraft {
    fn body(&self) -> Body {
        self.body
    }

    fn body_mut(&mut self) -> &mut Body {
        &mut self.body
    }

    fn sprite(&self) -> Sprite {
        Sprite {
            id: SPRITE_ID,
            size: SIZE,
        }
    }

    fn entity_type(&self) -> entity::Type {
        entity::Type::Enemy
    }

    fn alive(&self) -> bool {
        self.alive
    }
    fn alive_mut(&mut self) -> &mut bool {
        &mut self.alive
    }

    fn update(&mut self, ctx: &mut Context, state: &CartState) {
        if matches!(state.scene, Scene::Game { .. }) {
            self.move_it(ctx, state);
        }

        let pos = self.body().draw_pos().into();
        self.main_rotor.update(pos);
        self.tail_rotor.update(pos);
    }

    fn draw(&self, gfx: &mut rico8::Graphics, state: &CartState) {
        self.draw_default(gfx, state);

        self.main_rotor.draw(gfx);
        self.tail_rotor.draw(gfx);
    }

    // Override the "outside" definition since the aircraft is spawned above the screen.
    fn outside(&self) -> bool {
        let (x, y) = self.body().draw_pos();
        let size = self.sprite().size;

        x >= SCREEN_HEIGHT as i16 || (x + size.width.get() as i16) < 0 || y >= SCREEN_HEIGHT as i16
    }

    fn hit(&mut self, ctx: &mut Context, explosions: &mut VecView<Explosion>) {
        self.destroy(ctx, explosions);
        ctx.sfx(DESTROY_SFX);
    }
}

const SPRITE_ID: SpriteId = SpriteId(32);
const SIZE: Size = unsafe { Size::new_unchecked(6, 8) };
const MAIN_ROTOR_OFFSET: Position = Position { x: 2, y: 4 };
const MAIN_ROTOR_LENGTH: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(3) };
const TAIL_ROTOR_OFFSET: Position = Position { x: 2, y: 0 };
const TAIL_ROTOR_LENGTH: NonZeroU8 = unsafe { NonZeroU8::new_unchecked(2) };
const STARTING_Y: f32 = -8.0;
const SPEED: f32 = 0.3;
const DESTROY_SFX: SfxId = SfxId::new(2).unwrap();
