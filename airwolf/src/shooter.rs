use heapless::VecView;
use pixel8::{logf, Button, Context};

use crate::{bullet::Bullet, entity::Entity, Sky};

pub trait Shooter: Entity {
    /// The bullet properties.
    fn bullet_props(&self) -> BulletProps;

    /// Time last bullet was fired by this shooter.
    fn last_bullet(&self) -> f32;
    /// Reset the time last bullet was fired by this shooter to the current time.
    fn reset_last_bullet(&mut self, ctx: &Context);

    fn shoot(&mut self, ctx: &mut Context, world: &mut Sky, bullets: &mut VecView<Bullet>) {
        if !self.alive() || !self.bullet_cool_down(ctx) || !self.is_enemy() && !ctx.btn(Button::O) {
            return;
        }

        let bprops = self.bullet_props();
        let (x, y) = world.draw_pos(self.member());
        let x = x as f32 + bprops.x_offset;
        let y = y as f32 + bprops.y_offset;
        // The world is asked for a seat before the shooter's own cooldown is spent, but either
        // way it is spent once this decides to fire — a shot refused for want of a seat still
        // costs the cooldown, exactly as a shot that made it into the sky always has.
        let bullet = if self.is_enemy() {
            Bullet::new_enemy(x, y, ctx, world)
        } else {
            Bullet::new_friendly(x, y, ctx, world)
        };
        self.reset_last_bullet(ctx);

        match bullet {
            Some(bullet) => {
                // The vec can be full while the world still had a seat to give. The refused
                // bullet comes back out of the error, and its seat goes back with it — dropped
                // unretired, the seat would be nobody's for good, since no handle to it survives.
                if let Err(bullet) = bullets.push(bullet) {
                    world.retire(bullet.member());
                    logf!(ctx, "Err: Too many bullets: {}", super::MAX_BULLETS);
                }
            }
            None => {
                logf!(ctx, "Err: Too many bullets: {}", super::MAX_BULLETS);
            }
        }
    }

    /// Returns true if there has been sufficient time since the last bullet.
    fn bullet_cool_down(&self, ctx: &Context) -> bool {
        ctx.time() - self.last_bullet() > self.bullet_props().interval
    }
}

#[derive(Debug)]
pub struct BulletProps {
    pub x_offset: f32,
    pub y_offset: f32,
    pub interval: f32,
}
