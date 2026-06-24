use heapless::Vec;
use rico8::{Color, Context, Graphics};

use crate::common::Position;

#[derive(Debug)]
pub struct Explosion {
    sparks: Vec<Spark, MAX_SPARKS>,
}

impl Explosion {
    pub fn new(pos: Position, ctx: &mut Context) -> Self {
        let mut sparks = Vec::new();

        for _ in 0..MAX_SPARKS {
            sparks
                .push(Spark {
                    pos: pos.clone(),
                    x_velocity: ctx.random(2.0) - 1.0,
                    y_velocity: ctx.random(2.0) - 1.0,
                    radius: ctx.random(1.0) + 0.5,
                    mass: ctx.random(2.0) + 0.5,
                })
                // We add exactly `MAX_SPARKS` sparks so this can't be `Err`.
                .unwrap();
        }

        Self { sparks }
    }

    pub fn update(&mut self, _ctx: &mut Context) {
        if self.disappeared() {
            return;
        }

        self.sparks.retain_mut(
            |Spark {
                 pos: Position { x, y },
                 radius,
                 x_velocity,
                 y_velocity,
                 mass,
             }| {
                *x += *x_velocity / *mass;
                *y += *y_velocity / *mass;
                *radius -= RADIUS_SHRINK_SPEED;

                *radius > 0.0
            },
        );
    }

    pub fn draw(&self, gfx: &mut Graphics) {
        self.sparks.iter().for_each(
            |Spark {
                 pos: Position { x, y },
                 radius,
                 ..
             }| {
                gfx.circle_fill(*x, *y, *radius, SPARK_COLOR);
            },
        );
    }

    pub fn disappeared(&self) -> bool {
        self.sparks.is_empty()
    }
}

#[derive(Debug)]
struct Spark {
    pos: Position,
    x_velocity: f32,
    y_velocity: f32,
    radius: f32,
    mass: f32,
}

const MAX_SPARKS: usize = 50;
const RADIUS_SHRINK_SPEED: f32 = 0.1;
const SPARK_COLOR: Color = Color::LIGHT_GREY;
