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
use pixel8::{physics::World, plume::Explosion, *};

use crate::{
    bullet::Bullet, common::Position, enemy_aircraft::EnemyAircraft, entity::Entity,
    scrolling_map::ScrollingMap, shooter::Shooter, the_lady::TheLady,
};

pixel8::game!(Cart = Cart::new());

/// The one world everything that flies is seated in: sixty-four seats, exactly enough for the
/// lady, every aircraft in the air and every shot either side has in flight — see [`MAX_CAST`].
pub(crate) type Sky = World<MAX_CAST>;

struct Cart {
    /// The one thing that moves anything in this cart, and the one thing that owns where
    /// everybody is: bullets, aircraft and the lady each keep a `Member` handle into it beside
    /// their own game data, rather than a position of their own.
    world: Sky,
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
    /// The state the cart ships in, and every bit of it a constant: an empty, mapless sky, a lady
    /// with no seat yet, no cast, no score and no high score read yet. The whole of it is written
    /// into the cart's memory image and placed there by the loader — nothing builds it, and `boot`
    /// below opens a game in it rather than making one.
    const fn new() -> Self {
        Self {
            // The level scrolls past behind the fight and nothing on it is in anybody's way, so
            // the world never asks the map a question.
            world: Sky::mapless(),
            bullets: Vec::new(),
            explosions: Vec::new(),
            the_lady: TheLady::waiting(),
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

        // Everything the last run seated gives its seat back before this one takes new ones, so a
        // restart never grows the cast: spent bullets and aircraft still standing get retired
        // here, and so does the lady's — whether she is still alive (the very first `start`,
        // nothing having killed her yet) or was already retired when she died.
        for bullet in self.bullets.drain(..) {
            self.world.retire(bullet.member());
        }
        for aircraft in self.enemy_aircrafts.drain(..) {
            self.world.retire(aircraft.member());
        }
        if self.world.seated(self.the_lady.member()) {
            self.world.retire(self.the_lady.member());
        }

        self.explosions.clear();
        self.the_lady = TheLady::new(&mut self.world);
        self.smap = ScrollingMap::new();
        self.score = 0;
        self.playing_music = ctx
            .music(MUSIC_ID)
            .reserve_channels(Channel::Channel0 | Channel::Channel1 | Channel::Channel2)
            .play()
            .inspect_err(|&e| {
                logf!(64; ctx, "Music failed: {e}");
            })
            .ok();
        self.scene = Scene::Game {
            start_time: ctx.time(),
        };
    }

    fn running_update(&mut self, ctx: &mut Context) {
        let time = ctx.time();
        let in_game = matches!(self.scene, Scene::Game { .. });

        // The dead were already retired the moment the world stepped them (`react`, through
        // `Entity::destroy`), so this pass only drops the local record for those; whoever merely
        // flew off screen without being hit is retired here instead, on the way out.
        let world = &mut self.world;
        self.bullets.retain(|bullet| {
            if !bullet.alive() {
                return false;
            }
            if bullet.outside(world) {
                world.retire(bullet.member());
                return false;
            }
            true
        });

        let world = &mut self.world;
        let bullets = &mut self.bullets;
        let score = &mut self.score;
        self.enemy_aircrafts.retain_mut(|aircraft| {
            if in_game {
                aircraft.shoot(ctx, world, bullets);
            }
            let keep = aircraft.alive() && !aircraft.outside(world);
            // The aircraft is the only one that knows a shot of ours took it down rather than a
            // ram, and it is dropped exactly once — so the bump rides the drop. A dead one
            // already gave its seat back in `destroy`.
            if !keep && aircraft.died_to_shot() {
                *score += DESTORY_SCORE_BUMP as u32;
            }
            // Only a live escape earns the let-go points, and only a live escape still holds a
            // seat to give back here — a shot-down or rammed aircraft retired itself already.
            if aircraft.alive() && aircraft.outside(world) {
                *score += LET_GO_SCORE_BUMP as u32;
                world.retire(aircraft.member());
            }

            keep
        });
        self.explosions.retain_mut(|explosion| {
            explosion.update(ctx);
            !explosion.finished()
        });

        self.the_lady.shoot(ctx, &mut self.world, &mut self.bullets);

        if in_game {
            // Spawn an enemy aircraft every 1-4 seconds in game mode.
            let timeout = ctx.random(1.0..4.0);
            if time - self.last_enemy_ts > timeout {
                match EnemyAircraft::new(ctx, &mut self.world) {
                    Some(aircraft) => {
                        // The vec can be full while the world still had a seat: the refused
                        // aircraft comes back out of the error so its seat can be given back,
                        // or the seat would be orphaned past every restart.
                        if let Err(aircraft) = self.enemy_aircrafts.push(aircraft) {
                            self.world.retire(aircraft.member());
                            logf!(ctx, "Err: Too many aircrafts: {}", MAX_ENEMY_AIRCRAFTS);
                        }
                    }
                    None => {
                        logf!(ctx, "Err: Too many aircrafts: {}", MAX_ENEMY_AIRCRAFTS);
                    }
                }
                self.last_enemy_ts = time;
            }
        }
    }

    /// One update of everything that flies: each of them writes down where it means to go, and
    /// the world takes the whole cast there together.
    fn fly(&mut self, ctx: &mut Context, state: &CartState) {
        self.the_lady.steer(ctx, state, &mut self.world);
        for aircraft in &mut self.enemy_aircrafts {
            aircraft.steer(ctx, state, &mut self.world);
        }
        // A bullet holds the course it was fired on, so there is nothing to steer it with.

        // The world owns the whole cast now — bullets, aircraft and the lady each keep a seat in
        // it rather than being gathered here — so what used to be *cast order* is *seat order*:
        // whoever holds the lowest empty seat when it is enlisted is stepped first, and meets
        // everybody stepped before it where it has *just* moved to. Bullets and aircraft come and
        // go every update, and a freed seat is the next one `enlist` hands out, so which seat
        // ends up ahead of which shifts update to update — there is no "shots, then aircraft,
        // then the lady" left to preserve, and nothing here tries to force one back with seat
        // gymnastics. What does not shift: the world tells both parties of a meeting, whichever
        // one's movement made it, so a hit lands mutually however the cast happens to be seated,
        // and neither party has to survive the step for the other to hear of it.
        self.world.step(ctx);
    }

    /// What each of them makes of the step it has just taken, read from its own contacts.
    ///
    /// Same frame and both ways round: the shot that lands and the target it lands on are told of
    /// each other here, and either may die of it — and give its seat back — before anything is
    /// drawn.
    fn react(&mut self, ctx: &mut Context) {
        self.the_lady
            .react(ctx, &mut self.world, &mut self.explosions);
        for aircraft in &mut self.enemy_aircrafts {
            aircraft.react(ctx, &mut self.world, &mut self.explosions);
        }
        for bullet in &mut self.bullets {
            bullet.react(ctx, &mut self.world, &mut self.explosions);
        }
    }

    fn end_game(&mut self, ctx: &mut Context) {
        self.scene = Scene::GameOver {
            ts: Some(ctx.time()),
        };
        if let Some(p) = self.playing_music.take() {
            p.stop()
        }
        self.smap.stop_scrolling();

        if self.score > self.high_score {
            self.high_score = self.score;
            ctx.storage_set("high-score", self.score).unwrap();
        }
    }

    fn state(&self) -> CartState {
        CartState {
            scene: self.scene.clone(),
            protoganist_pos: self.the_lady.draw_pos(&self.world).into(),
        }
    }

    fn show_score(&self, gfx: &mut Graphics) {
        if matches!(self.scene, Scene::Game { .. } | Scene::GameOver { .. }) {
            printf!(gfx, SCORE_POS.x, SCORE_POS.y, SCORE_COLOR, "{}", self.score);
        }

        if self.high_score > 0 {
            printf!(
                gfx,
                HIGH_SCORE_POS.x,
                HIGH_SCORE_POS.y,
                SCORE_COLOR,
                "{:5}",
                self.high_score,
            );
        }
    }
}

impl Game for Cart {
    fn boot(&mut self, ctx: &mut Context) {
        // Everything the constant preset could not say: the high score is the store's to give,
        // and the lady's first seat is the world's — the very thing `start` already does on every
        // restart after this one.
        self.high_score = ctx
            .storage_get("high-score")
            .as_ref()
            .and_then(StorageValue::as_i64)
            .unwrap_or(0) as u32;
        self.the_lady = TheLady::new(&mut self.world);
    }

    fn update(&mut self, ctx: &mut Context) {
        self.smap.update(ctx);

        // The scene as the cast finds it — which is where the aircrafts read the lady off, so it
        // is taken before anything has moved.
        let state = self.state();
        self.fly(ctx, &state);
        self.react(ctx);

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
                    if let Some(p) = self.playing_music.take() {
                        p.fade_out(MUSIC_FAID_OUT_DURATION).stop()
                    }
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

        self.the_lady.draw(gfx, &self.state(), &self.world);

        self.bullets
            .iter()
            .for_each(|b| b.draw(gfx, &self.state(), &self.world));
        self.explosions.iter().for_each(|e| e.draw(gfx));
        self.enemy_aircrafts
            .iter()
            .for_each(|b| b.draw(gfx, &self.state(), &self.world));

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
            printf!(gfx, x, y, GAME_OVER_MSG_COLOR, "{}", msg);
        }

        self.show_score(gfx);
    }
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

// Sized so the whole sky fits in one world: the lady, every aircraft and every shot in flight sum
// to exactly the sixty-four seats a `World` can have. Forty-seven shots at once is far past what
// the fire rates can put in the air; a shot past the cap is refused where it is fired.
const MAX_BULLETS: usize = 47;
const MAX_ENEMY_AIRCRAFTS: usize = 16;
/// The lady, every aircraft in the air and every shot either side has in flight: everybody the
/// world can seat at once, and so the size of the `Sky` that seats them.
const MAX_CAST: usize = 1 + MAX_ENEMY_AIRCRAFTS + MAX_BULLETS;
const MAX_EXPLOSIONS: usize = MAX_ENEMY_AIRCRAFTS + 8;
// 3 seconds.
const GAME_OVER_TIMEOUT: f32 = 3.0;
const GAME_OVER_MSG_POS: Position = Position { x: 30, y: 70 };
const GAME_OVER_MSG_COLOR: Color = Color::WHITE;
// 30 seconds.
const MUSIC_DURATION: f32 = 30.0;
const MUSIC_FAID_OUT_DURATION: u32 = 5000;
// More points for letting an enemy aircraft go.
const LET_GO_SCORE_BUMP: u8 = 20;
const DESTORY_SCORE_BUMP: u8 = 10;
const SCORE_POS: Position = Position {
    x: 1,
    y: (SCREEN_HEIGHT as i16 - 8),
};
const HIGH_SCORE_POS: Position = Position {
    // 5 = length of the string printed.
    // 4 = pixels of each character.
    x: (SCREEN_WIDTH as i16 - 5 * 4),
    y: (SCREEN_HEIGHT as i16 - 8),
};
const SCORE_COLOR: Color = Color::WHITE;
const MUSIC_ID: MusicId = MusicId::new(0).unwrap();
