use heapless::VecView;
use pixel8::{logf, Context};

use crate::{bullet::Bullet, entity::Entity};

pub trait Shooter: Entity {
    /// The bullet properties.
    fn bullet_props(&self) -> BulletProps;

    /// Time last bullet was fired by this shooter.
    fn last_bullet(&self) -> f32;
    /// Reset the time last bullet was fired by this shooter to the current time.
    fn reset_last_bullet(&mut self, ctx: &Context);

    fn shoot(&mut self, ctx: &mut Context, bullets: &mut VecView<Bullet>) {
        if !self.alive()
            || !self.bullet_cool_down(ctx)
            || !self.is_enemy() && !ctx.btn(pixel8::Button::O)
        {
            return;
        }

        let bprops = self.bullet_props();
        let (x, y) = self.body().draw_pos();
        let x = x as f32 + bprops.x_offset;
        let y = y as f32 + bprops.y_offset;
        let bullet = if self.is_enemy() {
            Bullet::new_enemy(x, y, ctx)
        } else {
            Bullet::new_friendly(x, y, ctx)
        };
        self.reset_last_bullet(ctx);

        bullets.push(bullet).unwrap_or_else(|_| {
            logf!(ctx, "Err: Too many bullets: {}", super::MAX_BULLETS);
        })
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
