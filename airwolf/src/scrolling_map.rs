use rico8::{BitFlags, Color, Context, Graphics, SCREEN_HEIGHT};

#[derive(Debug)]
pub struct ScrollingMap {
    scroll: f32,
    scrolling: bool,
}

impl ScrollingMap {
    pub fn new() -> Self {
        Self {
            scroll: -(SCREEN_HEIGHT as f32),
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
        let last_y = SCREEN_HEIGHT * 16 - SCREEN_HEIGHT;
        if self.scroll >= last_y as f32 {
            self.scroll = -(SCREEN_HEIGHT_TWICE as f32);
        }

        self.scroll += SCROLL_SPEED;
    }

    pub fn draw(&self, gfx: &mut Graphics) {
        gfx.clear(Color::DARK_GREY);

        let scroll = self.scroll as i16;
        for i in 0..=7 {
            let tile_x = i * 16;
            let map_y = i * -SCREEN_HEIGHT_TWICE + scroll;

            gfx.map(tile_x, 0, 0, map_y, 16, 32, BitFlags::empty())
                .unwrap();
        }
    }
}

const SCREEN_HEIGHT_TWICE: i16 = SCREEN_HEIGHT as i16 * 2;
const SCROLL_SPEED: f32 = 0.3;
