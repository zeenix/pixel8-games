use heapless::Vec;
use pixel8::{Color, Context, Graphics};

use crate::common::Position;

#[derive(Debug)]
pub struct Explosion {
    pos: Position,
    sparks: Vec<Spark, MAX_SPARKS>,
}

impl Explosion {
    pub fn new(pos: Position, ctx: &mut Context) -> Self {
        let mut sparks = Vec::new();

        for _ in 0..MAX_SPARKS {
            sparks
                .push(Spark {
                    x_offset: 0,
                    y_offset: 0,
                    x_velocity: ctx.random_integer(MIN_VELOCIY..MAX_VELOCIY) as i8,
                    y_velocity: ctx.random_integer(MIN_VELOCIY..MAX_VELOCIY) as i8,
                    radius: ctx.random_integer(MIN_RADIUS..MAX_RADIUS) as u8,
                    mass: ctx.random_integer(MIN_MASS..MAX_MASS) as u8,
                })
                // We add exactly `MAX_SPARKS` sparks so this can't be `Err`.
                .unwrap();
        }

        Self { pos, sparks }
    }

    pub fn update(&mut self, _ctx: &mut Context) {
        if self.disappeared() {
            return;
        }

        self.sparks.retain_mut(
            |Spark {
                 x_offset,
                 y_offset,
                 radius,
                 x_velocity,
                 y_velocity,
                 mass,
             }| {
                *x_offset += *x_velocity / (*mass as i8);
                *y_offset += *y_velocity / (*mass as i8);
                *radius = radius.saturating_sub(RADIUS_SHRINK_SPEED);

                *radius > 0
            },
        );
    }

    pub fn draw(&self, gfx: &mut Graphics) {
        self.sparks.iter().for_each(
            |Spark {
                 x_offset,
                 y_offset,
                 radius,
                 ..
             }| {
                let x = self.pos.x + *x_offset as i16;
                let y = self.pos.y + *y_offset as i16;
                let radius = *radius as f32 / u8::MAX as f32 * 2.0;
                gfx.circle_fill(x, y, radius as u16, SPARK_COLOR);
            },
        );
    }

    pub fn disappeared(&self) -> bool {
        self.sparks.is_empty()
    }
}

#[derive(Debug)]
struct Spark {
    x_offset: i8,
    y_offset: i8,
    x_velocity: i8,
    y_velocity: i8,
    radius: u8,
    mass: u8,
}

const MAX_SPARKS: usize = 50;
const RADIUS_SHRINK_SPEED: u8 = u8::MAX / 15;
const SPARK_COLOR: Color = Color::LIGHT_GREY;

const MIN_VELOCIY: i32 = -6;
const MAX_VELOCIY: i32 = 6;
const MIN_RADIUS: i32 = (u8::MAX / 15) as i32;
const MAX_RADIUS: i32 = u8::MAX as i32;
const MIN_MASS: i32 = 4;
const MAX_MASS: i32 = 8;
