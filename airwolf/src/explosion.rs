use heapless::Vec;
use pixel8::{Color, Context, Graphics};

use crate::common::Position;

#[derive(Debug)]
pub struct Explosion {
    sparks: Vec<Spark, MAX_SPARKS>,
}

impl Explosion {
    pub fn new(pos: Position, ctx: &mut Context) -> Self {
        let mut sparks = Vec::new();

        for _ in 0..MAX_SPARKS {
            // We add exactly `MAX_SPARKS` sparks so this can't be `Err`.
            sparks.push(Spark::new(pos, ctx)).unwrap();
        }

        Self { sparks }
    }

    pub fn update(&mut self, _ctx: &mut Context) {
        self.sparks.retain_mut(|spark| {
            spark.x += spark.x_step as i16;
            spark.y += spark.y_step as i16;
            spark.radius = spark.radius.saturating_sub(RADIUS_SHRINK_SPEED);

            spark.radius > 0
        });
    }

    pub fn draw(&self, gfx: &mut Graphics) {
        for spark in &self.sparks {
            let x = spark.x / SUBPIXELS as i16;
            let y = spark.y / SUBPIXELS as i16;
            let radius = (spark.radius / SUBPIXELS as u8) as u16;
            gfx.circle_fill(x, y, radius, SPARK_COLOR);
        }
    }

    pub fn disappeared(&self) -> bool {
        self.sparks.is_empty()
    }
}

/// A single spark. All spatial values are fixed-point sub-pixel units (see [`SUBPIXELS`]).
///
/// Sub-pixel resolution is essential: most sparks move less than a pixel per frame, so
/// whole-pixel positions would truncate their movement to zero and they'd never leave the center.
#[derive(Debug)]
struct Spark {
    x: i16,
    y: i16,
    /// Per-frame movement, i.e. `velocity / mass`. Neither of the two changes during a spark's
    /// lifetime so only their ratio is stored.
    x_step: i8,
    y_step: i8,
    radius: u8,
}

impl Spark {
    fn new(pos: Position, ctx: &mut Context) -> Self {
        Self {
            x: pos.x * SUBPIXELS as i16,
            y: pos.y * SUBPIXELS as i16,
            x_step: random_step(ctx),
            y_step: random_step(ctx),
            radius: ctx.random_integer(MIN_RADIUS..MAX_RADIUS) as u8,
        }
    }
}

/// A random per-frame step, matching the original effect's `(-1 + rnd(2)) / (0.5 + rnd(2))`.
fn random_step(ctx: &mut Context) -> i8 {
    let velocity = ctx.random_integer(-SUBPIXELS..SUBPIXELS);
    let mass = ctx.random_integer(MIN_MASS..MAX_MASS);

    (velocity * SUBPIXELS / mass) as i8
}

const MAX_SPARKS: usize = 50;
const SPARK_COLOR: Color = Color::LIGHT_GREY;

/// Spatial values are stored as fixed-point numbers, in units of 1/32th of a pixel: multiply
/// by this to go from pixels to sub-pixel units, divide to go back.
const SUBPIXELS: i32 = 32;

// The original effect's parameters: velocity ∈ [-1, 1) px/frame, mass ∈ [0.5, 2.5) and
// radius ∈ [0.5, 1.5) px, shrinking by ~0.1 px/frame.
const MIN_MASS: i32 = SUBPIXELS / 2;
const MAX_MASS: i32 = SUBPIXELS * 5 / 2;
const MIN_RADIUS: i32 = SUBPIXELS / 2;
const MAX_RADIUS: i32 = SUBPIXELS * 3 / 2;
const RADIUS_SHRINK_SPEED: u8 = (SUBPIXELS / 10) as u8;
