extern crate rustbox;
extern crate unicode_width;
extern crate chip;

mod ui;
mod audio;
mod modulo;
use ui::Terminal;
use audio::Buzzer;
use chip::{Timer, Video, Audio, Input};

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}

#[derive(Default)]
pub struct Peripheral {
    term: Terminal,
    buzzer: Buzzer,
}

impl Timer for Peripheral {
    fn wait_next_frame(&self) {

    }
}

