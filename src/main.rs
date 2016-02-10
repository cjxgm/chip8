extern crate chip;
extern crate term_oss;

use chip::Chip;
use term_oss::Peripheral;

fn main() {
    let mut chip = Chip::default();
    let mut pe = Peripheral::default();

    chip.load(0x200, &[0x12, 0x00]);

    while !chip.frame(1000, &mut pe) {
    }
}

