#![no_std]

mod bullet;
mod common;
mod entity;
mod scrolling_map;
mod shooter;
mod the_lady;

use heapless::Vec;
use rico8::*;

use crate::{
    bullet::Bullet, entity::Entity, scrolling_map::ScrollingMap, shooter::Shooter,
    the_lady::TheLady,
};

rico8::game!(Cart = Cart::new());

#[derive(Debug)]
struct Cart {
    friendly_bullets: Vec<Bullet, MAX_FRIENDLY_BULLETS>,

    the_lady: TheLady,

    smap: ScrollingMap,
    scene: Scene,
    score: u32,
}

impl Cart {
    fn new() -> Self {
        Self {
            friendly_bullets: Vec::new(),
            the_lady: TheLady::new(),
            smap: ScrollingMap::new(),
            scene: Scene::Start,
            score: 0,
        }
    }

    fn start(&mut self, ctx: &mut Context) {
        if !ctx.is_button_down(Button::O) {
            return;
        }

        self.friendly_bullets = Vec::new();
        self.the_lady = TheLady::new();
        self.smap = ScrollingMap::new();
        self.scene = Scene::Game {
            start_time: ctx.time(),
        };
        self.score = 0;
        // FIXME: We probably need a channelmask like in pico8.
        ctx.music(MusicId(0));
    }
}

impl Game for Cart {
    fn update(&mut self, ctx: &mut Context) {
        self.smap.update(ctx);
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

        match self.scene {
            Scene::Start => {
                self.start(ctx);
            }
            Scene::GameOver { ts } if ctx.time() - ts > GAME_OVER_TIMEOUT => {
                self.start(ctx);
            }
            Scene::GameOver { .. } => {}
            Scene::Game { start_time } if ctx.time() - start_time > MUSIC_DURATION => {
                // FIXME: fade-out like pico8 when rico8 supports it.
                ctx.music_stop();
            }
            Scene::Game { .. } => {}
        }
    }

    fn draw(&self, gfx: &mut Graphics) {
        gfx.clear(Color::BLACK);
        self.smap.draw(gfx);

        self.the_lady.draw(gfx);

        for b in &self.friendly_bullets {
            b.draw(gfx);
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Scene {
    Start,
    Game { start_time: f32 },
    GameOver { ts: f32 },
}

const MAX_FRIENDLY_BULLETS: usize = 16;
// 3 seconds.
const GAME_OVER_TIMEOUT: f32 = 3.0;
// 30 seconds.
const MUSIC_DURATION: f32 = 30.0;
