use rico8::{Body, Context, SfxId, SpriteId, SCREEN_H, SCREEN_W};

use crate::{
    common::{Direction, Position, Size, Sprite},
    entity::{self, Entity},
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
        let x = ctx.random(SCREEN_W as f32);
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
        let size = self.sprite().size_in_blocks();

        x >= SCREEN_H as f32 || (x + size.width) < 0.0 || y >= SCREEN_H as f32
    }

    fn hit(&mut self, ctx: &mut Context) {
        ctx.sfx(DESTROY_SFX);
        self.alive = false;
    }
}

const SPRITE_ID: SpriteId = SpriteId(32);
const SIZE: Size = Size {
    width: 6.0,
    height: 8.0,
};
const MAIN_ROTOR_OFFSET: Position = Position { x: 2.5, y: 4.0 };
const MAIN_ROTOR_LENGTH: f32 = 2.0;
const TAIL_ROTOR_OFFSET: Position = Position { x: 2.0, y: 0.0 };
const TAIL_ROTOR_LENGTH: f32 = 1.0;
const STARTING_Y: f32 = -8.0;
const SPEED: f32 = 0.3;
const DESTROY_SFX: SfxId = SfxId(2);
