#![no_std]

mod bullet;
mod common;
mod entity;

use rico8::*;

use crate::{bullet::Bullet, entity::Entity};

#[derive(Default)]
struct Cart {
    enemy_bullet: Option<Bullet>,
    friendly_bullet: Option<Bullet>,
}

impl Game for Cart {
    fn update(&mut self, ctx: &mut Context) {
        match &mut self.enemy_bullet {
            Some(b) => {
                if b.update(ctx) {
                    self.enemy_bullet = None;
                }
            }
            None => self.enemy_bullet = Some(Bullet::new_enemy(20.0, 0.0, ctx)),
        }

        match &mut self.friendly_bullet {
            Some(b) => {
                if b.update(ctx) {
                    self.friendly_bullet = None;
                }
            }
            None => {
                self.friendly_bullet = Some(Bullet::new_friendly(80.0, SCREEN_H as f32 - 1.0, ctx))
            }
        }
    }

    fn draw(&self, gfx: &mut Graphics) {
        gfx.clear(Color::BLACK);

        if let Some(b) = &self.enemy_bullet {
            b.draw(gfx);
        }

        if let Some(b) = &self.friendly_bullet {
            b.draw(gfx);
        }
    }
}

rico8::game!(Cart);
