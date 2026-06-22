use rico8::{Body, Button, Color, Context, SpriteId, SCREEN_H, SCREEN_W};

use crate::{
    common::{Direction, Position, Size, Sprite},
    entity::{self, Entity},
    rotor::Rotor,
    shooter::{BulletProps, Shooter},
};

#[derive(Debug)]
pub struct TheLady {
    body: Body,
    main_rotor: Rotor,
    tail_rotor: Rotor,
    last_bullet: f32,
}

impl TheLady {
    pub fn new() -> Self {
        // TODO: Rotors.
        Self {
            body: Body::new(STARTING_POSITION.x, STARTING_POSITION.y),
            main_rotor: Rotor::new(MAIN_ROTOR_OFFSET, MAIN_ROTOR_LENGTH),
            tail_rotor: Rotor::new(TAIL_ROTOR_OFFSET, TAIL_ROTOR_LENGTH),
            last_bullet: 0.0,
        }
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
        entity::Type::Protoganist
    }

    fn update(&mut self, ctx: &mut Context) {
        let (x, y) = self.body.draw_pos();
        let Size { width, height } = self.sprite().size;

        let can_left = x > -1.0;
        let can_right = x + width < SCREEN_W as f32 - 2.0;
        let can_up = y > 0.0;
        let can_down = y + height < SCREEN_H as f32;
        let can_up_left = can_left && can_up;
        let can_down_left = can_left && can_down;
        let can_up_right = can_right && can_up;
        let can_down_right = can_right && can_down;

        let buttons = ctx.buttons_down();
        let dir = if buttons.contains(Button::UP_LEFT) && can_up_left {
            Some(Direction::UpLeft)
        } else if buttons.contains(Button::UP_RIGHT) && can_up_right {
            Some(Direction::UpRight)
        } else if buttons.contains(Button::DOWN_LEFT) && can_down_left {
            Some(Direction::DownLeft)
        } else if buttons.contains(Button::DOWN_RIGHT) && can_down_right {
            Some(Direction::DownRight)
        } else if buttons.contains(Button::Up) && can_up {
            Some(Direction::Up)
        } else if buttons.contains(Button::Down) && can_down {
            Some(Direction::Down)
        } else if buttons.contains(Button::Left) && can_left {
            Some(Direction::Left)
        } else if buttons.contains(Button::Right) && can_right {
            Some(Direction::Right)
        } else {
            None
        };

        if let Some(dir) = dir {
            self.go(dir, SPEED);
        }

        let pos = self.body().draw_pos().into();
        self.main_rotor.update(pos);
        self.tail_rotor.update(pos);
    }

    fn draw(&self, gfx: &mut rico8::Graphics) {
        gfx.set_transparent_color(Color::BLACK, false);
        gfx.set_transparent_color(Color::DARK_GREY, true);
        self.draw_default(gfx);
        gfx.reset_transparency();

        self.main_rotor.draw(gfx);
        self.tail_rotor.draw(gfx);
    }
}

const SPRITE_ID: SpriteId = SpriteId(1);
const SIZE: Size = Size {
    width: 8.0,
    height: 8.0,
};
const MAIN_ROTOR_OFFSET: Position = Position { x: 4.5, y: 3.5 };
const MAIN_ROTOR_LENGTH: f32 = 2.0;
const TAIL_ROTOR_OFFSET: Position = Position { x: 4.0, y: 7.0 };
const TAIL_ROTOR_LENGTH: f32 = 1.0;
const STARTING_POSITION: Position = Position { x: 63.0, y: 111.0 };
const SPEED: f32 = 0.7;
