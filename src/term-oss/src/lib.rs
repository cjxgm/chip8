extern crate chip;
extern crate time;
extern crate rustbox;
extern crate unicode_width;

mod ui;
mod audio;
mod modulo;
use ui::Terminal;
use audio::Buzzer;
use chip::{Timer, Video, Audio, Input};
use std::time::Duration;

#[derive(Default)]
pub struct Peripheral {
    term: Terminal,
    buzzer: Buzzer,
}

impl Timer for Peripheral {
    fn pump(&mut self) -> bool {
        const NANOS_PER_FRAME: u32 = 1_000_000_000 / 60;
        self.term.pump_events(Duration::new(0, NANOS_PER_FRAME))
    }
}

impl Video for Peripheral {
    fn clear(&mut self) {
        self.term.clear();
    }
    fn draw(&mut self, x: isize, y: isize, sprite: &[u8]) -> bool {
        self.term.flip_sprites(x, y, sprite)
    }
    fn present(&self) {
        self.term.paint();
    }
}

impl Audio for Peripheral {
    fn buzz(&self, on: bool) {
        self.buzzer.buzz(on);
    }
}

impl Input for Peripheral {
    fn keydown(&self, which: usize) -> bool {
        self.term.keydown(which)
    }
}

