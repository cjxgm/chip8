//! `chip` implements the chip-8 CPU and RAM.
//! Peripherals (IO) are decoupled with the chip and are handled by the user.

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn no_inst() {
        let mut chip = Chip::default();
        chip.cycle();
        chip.cycle();
    }

    #[test]
    fn ping_pong_jmp() {
        let mut chip = Chip::default();
        chip.load(0x200, &[0x10, 0x00]);
        for _ in 0..100 {
            chip.cycle();
        }
    }

    #[test]
    fn infinite_loop() {
        let mut chip = Chip::default();
        chip.load(0x200, &[0x12, 0x00]);
        for _ in 0..100 {
            chip.cycle();
        }
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

pub struct Chip {
    ram: Ram,
    stack: Vec<u16>,
    pc: usize,
}

impl Chip {
    pub fn reset(&mut self) {
        *self = Default::default();
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
        self.pc += 2;

        //println!("decode {:04X}", inst);    // TODO: remove this
        decode! { inst =>
            1NNN(n) { self.pc = n as usize }
        };

        panic!("unknown instruction {:04X}", inst);
    }

    pub fn frame<P>(&mut self, num_cycle: usize, peripheral: &mut P) -> bool
        where P: Timer + Input + Video
    {
        for _ in 0..num_cycle { self.cycle() }
        peripheral.present();
        peripheral.pump()
    }


    /// Only allowed construction by Default trait.
    fn new() -> Chip {
        let mut chip = Chip {
            ram: Default::default(),
            stack: Default::default(),
            pc: Default::default(),
        };
        chip.load(0, &[0x12, 0x00]);
        // TODO: load font data
        chip
    }
}

impl Default for Chip {
    fn default() -> Chip {
        Chip::new()
    }
}

