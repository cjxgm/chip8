//! `chip` implements the chip-8 CPU and RAM.
//! Peripherals (IO) are decoupled with the chip and are handled by the user.

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        decode(0x7234);
        let mut chip = Chip::new();
        chip.reset();
    }
}

//---- modules ----
#[macro_use]
mod decoder;

mod ram;
use ram::{Ram, Write, Read};

pub mod peripheral;
pub use peripheral::{Timer, Video, Audio, Input};

//---- Chip ----

#[derive(Default)]
pub struct Chip {
    ram: Ram,
    stack: Vec<u16>,
    pc: usize,
}

impl Chip {
    pub fn new() -> Chip {
        Default::default()
    }
    pub fn reset(&mut self) {
        *self = Chip::new();
    }
    pub fn load(&mut self, addr: usize, data: &[u8]) {
        let mut i = addr;
        for &x in data {
            self.ram.write(i, x);
            i += 1;
        }
    }
    pub fn cycle(&mut self) {
        let inst: u16 = self.ram.read(self.pc);
    }
    pub fn frame<P>(&mut self, num_cycle: usize, peripheral: &P)
            where P: Timer {
        for _ in 0..num_cycle { self.cycle() }
        peripheral.wait_next_frame();
    }
}

impl Drop for Chip {
    fn drop(&mut self) {
        println!("drop");
    }
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

