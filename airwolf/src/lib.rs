#![no_std]

mod bullet;
mod common;
mod enemy_aircraft;
mod entity;
mod rotor;
mod scrolling_map;
mod shooter;
mod the_lady;

use heapless::Vec;
use rico8::*;

use crate::{
    bullet::Bullet, common::Position, enemy_aircraft::EnemyAircraft, entity::Entity,
    scrolling_map::ScrollingMap, shooter::Shooter, the_lady::TheLady,
};

rico8::game!(Cart = Cart::new());

#[derive(Debug)]
struct Cart {
    bullets: Vec<Bullet, MAX_BULLETS>,

    the_lady: TheLady,
    enemy_aircrafts: Vec<EnemyAircraft, MAX_ENEMY_AIRCRAFTS>,
    last_enemy_ts: f32,

    smap: ScrollingMap,
    scene: Scene,
    score: u32,
    playing_music: Option<PlayingMusic>,
}

impl Cart {
    fn new() -> Self {
        Self {
            bullets: Vec::new(),
            the_lady: TheLady::new(),
            enemy_aircrafts: Vec::new(),
            last_enemy_ts: 0.0,
            smap: ScrollingMap::new(),
            scene: Scene::Start,
            score: 0,
            playing_music: None,
        }
    }

    fn start(&mut self, ctx: &mut Context) {
        if !ctx.is_button_down(Button::O) {
            return;
        }

        self.bullets.clear();
        self.the_lady = TheLady::new();
        self.smap = ScrollingMap::new();
        self.score = 0;
        self.playing_music = ctx
            .music(MusicId(0))
            .reserve_channels(
                MusicChannel::Channel0 | MusicChannel::Channel1 | MusicChannel::Channel2,
            )
            .play()
            .map_err(|e| {
                logf!(64; ctx, "Music failed: {e}");

                e
            })
            .ok();
        self.scene = Scene::Game {
            start_time: ctx.time(),
        };
    }

    fn running_update(&mut self, ctx: &mut Context) {
        let time = ctx.time();

        let state = self.state();
        self.bullets.retain_mut(|bullet| {
            match bullet.entity_type() {
                entity::Type::EnemyBullet => bullet.handle_collision(&mut self.the_lady, ctx),
                entity::Type::FriendlyBullet => {
                    for aircraft in &mut self.enemy_aircrafts {
                        bullet.handle_collision(aircraft, ctx);
                    }
                }
                _ => unreachable!("unknown bullet type encountered"),
            }

            retain_fn(bullet, ctx, &state)
        });
        self.enemy_aircrafts.retain_mut(|aircraft| {
            debug_assert!(matches!(aircraft.entity_type(), entity::Type::Enemy));
            aircraft.handle_collision(&mut self.the_lady, ctx);

            retain_fn(aircraft, ctx, &state)
        });

        if let Some(b) = self.the_lady.shoot(ctx) {
            self.bullets.push(b).unwrap_or_else(|_| {
                logf!(ctx, "Err: Too many bullets: {}", MAX_BULLETS);
            })
        }

        if matches!(self.scene, Scene::Game { .. }) {
            // Spawn an enemy aircraft every 1-4 seconds in game mode.
            let timeout = ctx.random(3.0) + 1.0;
            if time - self.last_enemy_ts > timeout {
                self.enemy_aircrafts
                    .push(EnemyAircraft::new(ctx))
                    .unwrap_or_else(|_| {
                        logf!(ctx, "Err: Too many aircrafts: {}", MAX_ENEMY_AIRCRAFTS);
                    });
                self.last_enemy_ts = time;
            }
        }
    }

    fn end_game(&mut self, ctx: &mut Context) {
        self.scene = Scene::GameOver { ts: ctx.time() };
        self.playing_music.take().map(|p| p.stop());
        self.smap.stop_scrolling();
    }

    fn state(&self) -> CartState {
        CartState {
            scene: self.scene.clone(),
            protoganist_pos: self.the_lady.body().draw_pos().into(),
        }
    }
}

impl Game for Cart {
    fn update(&mut self, ctx: &mut Context) {
        self.smap.update(ctx);
        self.the_lady.update(ctx, &self.state());

        match self.scene {
            Scene::Start => {
                self.start(ctx);
            }
            Scene::GameOver { ts } if ctx.time() - ts > GAME_OVER_TIMEOUT => {
                self.start(ctx);
            }
            Scene::GameOver { .. } => self.running_update(ctx),
            Scene::Game { start_time } => {
                self.running_update(ctx);

                if self.playing_music.is_some() && ctx.time() - start_time > MUSIC_DURATION {
                    self.playing_music
                        .take()
                        .map(|p| p.fade_out(MUSIC_FAID_OUT_DURATION).stop());
                }

                if !self.the_lady.alive() {
                    self.end_game(ctx);
                }
            }
        }
    }

    fn draw(&self, gfx: &mut Graphics) {
        gfx.clear(Color::BLACK);
        self.smap.draw(gfx);

        self.the_lady.draw(gfx, &self.state());

        self.bullets.iter().for_each(|b| b.draw(gfx, &self.state()));
        self.enemy_aircrafts
            .iter()
            .for_each(|b| b.draw(gfx, &self.state()));
    }
}

fn retain_fn<E: Entity>(entity: &mut E, ctx: &mut Context, state: &CartState) -> bool {
    entity.update(ctx, state);

    !entity.outside() && entity.alive()
}

#[derive(Debug, Clone)]
pub(crate) struct CartState {
    scene: Scene,
    protoganist_pos: Position,
}

#[derive(Debug, Clone)]
pub(crate) enum Scene {
    Start,
    Game { start_time: f32 },
    GameOver { ts: f32 },
}

const MAX_BULLETS: usize = 64;
const MAX_ENEMY_AIRCRAFTS: usize = 16;
// 3 seconds.
const GAME_OVER_TIMEOUT: f32 = 3.0;
// 30 seconds.
const MUSIC_DURATION: f32 = 30.0;
const MUSIC_FAID_OUT_DURATION: u32 = 5000;
