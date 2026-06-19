#![no_std]

mod bullet;
mod common;
mod entity;
mod shooter;
mod the_lady;

use heapless::Vec;
use rico8::*;

use crate::{bullet::Bullet, entity::Entity, shooter::Shooter, the_lady::TheLady};

rico8::game!(Cart = Cart::new());

#[derive(Debug)]
struct Cart {
    friendly_bullets: Vec<Bullet, MAX_FRIENDLY_BULLETS>,

    the_lady: TheLady,
}

impl Cart {
    fn new() -> Self {
        Self {
            friendly_bullets: Vec::new(),
            the_lady: TheLady::new(),
        }
    }
}

impl Game for Cart {
    fn update(&mut self, ctx: &mut Context) {
        self.the_lady.update(ctx);

        self.friendly_bullets.retain_mut(|b| {
            b.update(ctx);

            !b.outside()
        });

        if let Some(b) = self.the_lady.shoot(ctx) {
            self.friendly_bullets.push(b).unwrap_or_else(|_| {
                logf!(ctx, "Err: Too many bullets: {}", MAX_FRIENDLY_BULLETS);
            })
        }
    }

    fn draw(&self, gfx: &mut Graphics) {
        gfx.clear(Color::BLACK);
        // TODO: Scrolling map.
        gfx.map(0, 0, 0.0, 0.0, 16, 32, BitFlags::empty());

        self.the_lady.draw(gfx);

        for b in &self.friendly_bullets {
            b.draw(gfx);
        }
    }
}

rico8::game!(
    Cart = Cart {
        friendly_bullets: Vec::new(),

        the_lady: TheLady::new(),
    }
);

const MAX_FRIENDLY_BULLETS: usize = 16;
