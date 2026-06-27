#![no_std]

mod bullet;
mod common;
mod enemy_aircraft;
mod entity;
mod explosion;
mod rotor;
mod scrolling_map;
mod shooter;
mod the_lady;

use heapless::Vec;
use rico8::*;

use crate::{
    bullet::Bullet, common::Position, enemy_aircraft::EnemyAircraft, entity::Entity,
    explosion::Explosion, scrolling_map::ScrollingMap, shooter::Shooter, the_lady::TheLady,
};

rico8::game!(Cart = Cart::new());

#[derive(Debug)]
struct Cart {
    bullets: Vec<Bullet, MAX_BULLETS>,
    explosions: Vec<Explosion, MAX_EXPLOSIONS>,

    the_lady: TheLady,
    enemy_aircrafts: Vec<EnemyAircraft, MAX_ENEMY_AIRCRAFTS>,
    last_enemy_ts: f32,

    smap: ScrollingMap,
    scene: Scene,
    score: u32,
    high_score: u32,
    playing_music: Option<PlayingMusic>,
}

impl Cart {
    fn new() -> Self {
        Self {
            bullets: Vec::new(),
            explosions: Vec::new(),
            the_lady: TheLady::new(),
            enemy_aircrafts: Vec::new(),
            last_enemy_ts: 0.0,
            smap: ScrollingMap::new(),
            scene: Scene::Start,
            score: 0,
            high_score: 0,
            playing_music: None,
        }
    }

    fn start(&mut self, ctx: &mut Context) {
        if !ctx.is_button_down(Button::O) {
            return;
        }

        self.bullets.clear();
        self.explosions.clear();
        self.enemy_aircrafts.clear();
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
                entity::Type::EnemyBullet => {
                    bullet.handle_collision(&mut self.the_lady, ctx, &mut self.explosions)
                }
                entity::Type::FriendlyBullet => {
                    for aircraft in &mut self.enemy_aircrafts {
                        bullet.handle_collision(aircraft, ctx, &mut self.explosions);
                        if !aircraft.alive() {
                            self.score += DESTORY_SCORE_BUMP as u32;
                        } else if aircraft.outside() {
                            self.score += LET_GO_SCORE_BUMP as u32;
                        }
                    }
                }
                _ => unreachable!("unknown bullet type encountered"),
            }

            retain_fn(bullet, ctx, &state)
        });
        self.enemy_aircrafts.retain_mut(|aircraft| {
            debug_assert!(matches!(aircraft.entity_type(), entity::Type::Enemy));
            aircraft.handle_collision(&mut self.the_lady, ctx, &mut self.explosions);
            if matches!(self.scene, Scene::Game { .. }) {
                aircraft.shoot(ctx, &mut self.bullets);
            }

            retain_fn(aircraft, ctx, &state)
        });
        self.explosions.retain_mut(|explosion| {
            explosion.update(ctx);
            !explosion.disappeared()
        });

        self.the_lady.shoot(ctx, &mut self.bullets);

        if matches!(self.scene, Scene::Game { .. }) {
            // Spawn an enemy aircraft every 1-4 seconds in game mode.
            let timeout = ctx.random(1.0..4.0);
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
        self.scene = Scene::GameOver {
            ts: Some(ctx.time()),
        };
        self.playing_music.take().map(|p| p.stop());
        self.smap.stop_scrolling();
        if self.score > self.high_score {
            self.high_score = self.score;
        }
    }

    fn state(&self) -> CartState {
        CartState {
            scene: self.scene.clone(),
            protoganist_pos: self.the_lady.body().draw_pos().into(),
        }
    }

    fn show_score(&self, gfx: &mut Graphics) {
        if matches!(self.scene, Scene::Game { .. } | Scene::GameOver { .. }) {
            printf!(
                gfx,
                SCORE_POS.x as i16,
                SCORE_POS.y as i16,
                SCORE_COLOR,
                "{}",
                self.score
            );
        }

        if self.high_score > 0 {
            printf!(
                gfx,
                HIGH_SCORE_POS.x as i16,
                HIGH_SCORE_POS.y as i16,
                SCORE_COLOR,
                "{:5}",
                self.high_score
            );
        }
    }
}

impl Game for Cart {
    fn update(&mut self, ctx: &mut Context) {
        self.smap.update(ctx);
        self.the_lady.update(ctx, &self.state());

        match self.scene {
            Scene::Start => self.start(ctx),
            Scene::GameOver { ts: Some(ts) } if ctx.time() - ts > GAME_OVER_TIMEOUT => {
                self.scene = Scene::GameOver { ts: None };
                self.start(ctx);
            }
            Scene::GameOver { ts: Some(_) } => self.running_update(ctx),
            Scene::GameOver { ts: None } => {
                // Game's been over and we already waited for `GAME_OVER_TIMEOUT` after that.
                // Keep updating the scene, so animations continue and continue to try starting the
                // game.
                self.running_update(ctx);
                self.start(ctx);
            }
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
        self.explosions.iter().for_each(|e| e.draw(gfx));
        self.enemy_aircrafts
            .iter()
            .for_each(|b| b.draw(gfx, &self.state()));

        let msg = match self.scene {
            Scene::Start => Some("Press O to start"),
            Scene::GameOver { ts: None } => {
                // `ts` being `None` means we already waited for `GAME_OVER_TIMEOUT` already.
                Some("Press O to restart")
            }
            _ => None,
        };
        if let Some(msg) = msg {
            let Position { x, y } = GAME_OVER_MSG_POS;
            printf!(gfx, x as i16, y as i16, GAME_OVER_MSG_COLOR, "{}", msg);
        }

        self.show_score(gfx);
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
    Game {
        start_time: f32,
    },
    GameOver {
        /// When the game was over. It's set to `None` after `GAME_OVER_TIMEOUT`.
        ts: Option<f32>,
    },
}

const MAX_BULLETS: usize = 64;
const MAX_ENEMY_AIRCRAFTS: usize = 16;
const MAX_EXPLOSIONS: usize = MAX_ENEMY_AIRCRAFTS + 8;
// 3 seconds.
const GAME_OVER_TIMEOUT: f32 = 3.0;
const GAME_OVER_MSG_POS: Position = Position { x: 30.0, y: 70.0 };
const GAME_OVER_MSG_COLOR: Color = Color::WHITE;
// 30 seconds.
const MUSIC_DURATION: f32 = 30.0;
const MUSIC_FAID_OUT_DURATION: u32 = 5000;
// More points for letting an enemy aircraft go.
const LET_GO_SCORE_BUMP: u8 = 20;
const DESTORY_SCORE_BUMP: u8 = 10;
const SCORE_POS: Position = Position {
    x: 1.0,
    y: (SCREEN_HEIGHT - 8) as f32,
};
const HIGH_SCORE_POS: Position = Position {
    // 5 = length of the string printed.
    // 4 = pixels of each character.
    x: (SCREEN_WIDTH - 5 * 4) as f32,
    y: (SCREEN_HEIGHT - 8) as f32,
};
const SCORE_COLOR: Color = Color::WHITE;
