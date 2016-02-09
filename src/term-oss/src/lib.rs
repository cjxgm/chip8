extern crate chip;
use chip::{Timer, Video, Audio, Input};

mod ui;
mod audio;
use ui::Terminal;
use audio::Buzzer;

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

