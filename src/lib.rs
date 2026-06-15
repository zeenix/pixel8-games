#![no_std]

mod bullet;
mod common;
mod entity;

use rico8::*;

#[derive(Default)]
struct Cart;

impl Game for Cart {
    fn update(&mut self, _ctx: &mut Context) {}

    fn draw(&self, gfx: &mut Graphics) {
        gfx.clear(Color::BLACK);
        gfx.print("imported from pico-8", 12.0, 54.0, Color::WHITE);
        gfx.print("write your game here", 16.0, 64.0, Color::LIGHT_GREY);
    }
}

rico8::game!(Cart);
