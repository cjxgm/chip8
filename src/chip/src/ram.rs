//! Ram implementation for the chip.
//!
//! Support writing and reading various sized native types.
//! Does endianess conversion since chip-8 is big-endian machine.

use std::mem::transmute;

pub trait Read<T> {
    fn read(&mut self, addr: usize) -> T;
}

pub trait Write<T> {
    fn write(&mut self, addr: usize, value: T);
}

const RAM_SIZE: usize = 0x1000;
pub struct Ram {
    mem: [u8; RAM_SIZE],
}

impl Ram {
    fn new() -> Ram {
        Default::default()
    }
}

impl Default for Ram {
    fn default() -> Ram {
        Ram {
            mem: [0; RAM_SIZE],
        }
    }
}

impl Read<u8> for Ram {
    fn read(&mut self, addr: usize) -> u8 {
        self.mem[addr]
    }
}

impl Read<u16> for Ram {
    fn read(&mut self, addr: usize) -> u16 {
        let be: &u16 = unsafe { transmute(&self.mem[addr]) };
        u16::from_be(*be)
    }
}

impl Write<u8> for Ram {
    fn write(&mut self, addr: usize, value: u8) {
        self.mem[addr] = value;
    }
}

impl Write<u16> for Ram {
    fn write(&mut self, addr: usize, value: u16) {
        let be: &mut u16 = unsafe { transmute(&mut self.mem[addr]) };
        *be = value.to_be();
    }
}

