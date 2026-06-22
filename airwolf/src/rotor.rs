use core::f32::consts::PI;

use libm::{cosf, fmodf, sinf};
use rico8::Color;

use crate::common::Position;

#[derive(Debug)]
pub struct Rotor {
    angle: f32,
    offset: Position,
    length: f32,

    // Position of edges.
    edges_pos: (Position, Position),
}

impl Rotor {
    pub fn new(offset: Position, length: f32) -> Self {
        Self {
            angle: 0.0,
            offset,
            length,
            edges_pos: (Position { x: 0.0, y: 0.0 }, Position { x: 0.0, y: 0.0 }),
        }
    }

    pub fn update(&mut self, aircraft_pos: Position) {
        self.angle = fmodf(self.angle + SPEED as f32, 2.0 * PI);

        // FIXME: Do we need this?
        let angle = self.angle + PI;
        let arm_x = cosf(angle) * self.length;
        let arm_y = sinf(angle) * self.length;
        self.edges_pos = (
            Position {
                x: aircraft_pos.x + arm_x + self.offset.x,
                y: aircraft_pos.y + arm_y + self.offset.y,
            },
            Position {
                x: aircraft_pos.x - arm_x + self.offset.x,
                y: aircraft_pos.y - arm_y + self.offset.y,
            },
        )
    }

    pub fn draw(&self, gfx: &mut rico8::Graphics) {
        let (Position { x: x0, y: y0 }, Position { x: x1, y: y1 }) = self.edges_pos;

        gfx.line(x0, y0, x1, y1, COLOR);
    }
}

const SPEED: u8 = 1;
const COLOR: Color = Color::LAVENDER;
