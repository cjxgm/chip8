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

#[derive(Default)]
pub struct Peripheral {
    term: Terminal,
    buzzer: Buzzer,
}

impl Timer for Peripheral {
    fn wait_next_frame(&self) {

    }
}

