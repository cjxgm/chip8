extern crate chip;
use chip::{Video, Audio, Input};

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
    }
}

pub struct Peripheral;

impl Video for Peripheral {
    fn clear() {
    }

    fn flip(x: usize, y: usize) -> bool {
        x + y > 10
    }
}

impl Audio for Peripheral {
}

impl Input for Peripheral {
}

