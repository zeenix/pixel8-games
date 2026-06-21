use rico8::{BitFlags, Color, Context, Graphics, SCREEN_H};

#[derive(Debug)]
pub struct ScrollingMap {
    scroll: f32,
    scrolling: bool,
}

impl ScrollingMap {
    pub fn new() -> Self {
        Self {
            scroll: -SCREEN_H as f32,
            scrolling: true,
        }
    }

    pub fn stop_scrolling(&mut self) {
        self.scrolling = false;
    }

    pub fn update(&mut self, _ctx: &mut Context) {
        if !self.scrolling {
            return;
        }

        // FIXME: Constant for the `16`.
        let last_y = SCREEN_H * 16 - SCREEN_H;
        if self.scroll >= last_y as f32 {
            self.scroll = -SCREEN_HEIGHT_TWICE;
        }

        self.scroll += SCROLL_SPEED;
    }

    pub fn draw(&self, gfx: &mut Graphics) {
        gfx.clear(Color::DARK_GREY);

        for i in 0..=7 {
            let tile_x = i * 16;
            let map_y = i as f32 * -SCREEN_HEIGHT_TWICE + self.scroll;

            gfx.map(tile_x, 0, 0.0, map_y, 16, 32, BitFlags::empty());
        }
    }
}

const SCREEN_HEIGHT_TWICE: f32 = SCREEN_H as f32 * 2.0;
const SCROLL_SPEED: f32 = 0.3;
