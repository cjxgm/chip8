//! `chip` implements the chip-8 CPU and RAM.
//! Peripherals (IO) are decoupled with the chip and are handled by the user.

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        decode(0x7234);
    }
}

#[macro_use]
mod decoder;
pub mod peripheral;
pub use peripheral::{Video, Audio, Input};

pub struct Chip {
    ram: [u8; 4 * 1024],
}


pub fn decode(inst: u16) {
    println!("decode {:04X}", inst);
    let decode = decoder! {
        7XNN(x, n) { println!("  x:{:X} n:{:X}", x, n) }
        8XY1(x, y) { println!("  x:{:X} y:{:X}", x, y) }
        00E0() { println!("  E") }
        00E1() { println!("  F") }
        0NNN(n) { println!("  n:{:X}", n) }
    };
    decode(inst);
}

