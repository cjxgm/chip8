//! `chip` implements the chip-8 CPU and RAM.
//! Peripherals (IO) are decoupled with the chip and are handled by the user.

extern crate rand;

#[macro_use]
mod decoder;
mod ram;
pub mod peripheral;

pub use peripheral::{Timer, Video, Audio, Input};
use ram::{Ram, Write, Read, Slice};
use std::num::Wrapping;
use rand::random;

pub struct Chip {
    /// Program begins at 0x0200
    ram: Ram,
    stack: Vec<u16>,
    pc: Wrapping<u16>,
    /// R0 ~ RF, sixteen 8-bit registers
    regs: [Wrapping<u8>; 16],
    /// I, one 16-bit register
    reg_i: Wrapping<u16>,
    /// Count down one frame at a time
    reg_delay: u8,
    /// Count down one frame at a time. When not zero, buzz the speaker
    reg_sound: u8,
}

impl Chip {
    pub fn reset(&mut self) {
        *self = Default::default();
    }

    pub fn load(&mut self, addr: u16, data: &[u8]) {
        let mut i = addr;
        for &x in data {
            self.ram.write(i as usize, x);
            i += 1;
        }
    }

    pub fn frame<P>(&mut self, num_cycle: usize, peripheral: &mut P) -> bool
        where P: Timer + Video + Audio + Input
    {
        if self.reg_delay > 0 { self.reg_delay -= 1 }
        if self.reg_sound > 0 {
            self.reg_sound -= 1;
            peripheral.buzz(true);
        } else {
            peripheral.buzz(false);
        }

        for _ in 0..num_cycle {
            if self.cycle(peripheral) {
                return true;
            }
        }

        peripheral.present();
        peripheral.pump()
    }


    /// Only allowed construction by Default trait.
    fn new() -> Chip {
        let mut chip = Chip {
            ram: Default::default(),
            stack: Default::default(),
            pc: Wrapping(0x0200),
            regs: [Wrapping(0); 16],
            reg_i: Wrapping(0),
            reg_delay: 0,
            reg_sound: 0,
        };
        // TODO: load font data
        chip
    }

    fn cycle<Peripheral>(&mut self, p: &mut Peripheral) -> bool
        where Peripheral: Video + Audio + Input
    {
        const INST_SIZE: Wrapping<u16> = Wrapping(2);
        const MSB: Wrapping<u8> = Wrapping(0b1000_0000);
        const LSB: Wrapping<u8> = Wrapping(0b0000_0001);

        let inst: u16 = self.ram.read(self.pc.0 as usize);
        self.pc = self.pc + INST_SIZE;

        let mut stop = false;

        //println!("decode {:04X}", inst);    // TODO: remove this
        (|| {
            decode! { inst =>
                "00E0" => () { p.clear() }
                "00EE" => () { self.pc.0 = self.stack.pop().unwrap() }
                "1NNN" => (n) { self.pc.0 = n }
                "2NNN" => (n) { self.stack.push(self.pc.0); self.pc.0 = n }
                "3XNN" => (x, n) { if self.regs[x].0 == n as u8 { self.pc = self.pc + INST_SIZE } }
                "4XNN" => (x, n) { if self.regs[x].0 != n as u8 { self.pc = self.pc + INST_SIZE } }
                "5XY0" => (x, y) { if self.regs[x] == self.regs[y] { self.pc = self.pc + INST_SIZE } }
                "6XNN" => (x, n) { self.regs[x].0 = n as u8 }
                "7XNN" => (x, n) { self.regs[x] = self.regs[x] + Wrapping(n as u8) }
                "8XY0" => (x, y) { self.regs[x] = self.regs[y] }
                "8XY1" => (x, y) { self.regs[x] = self.regs[x] | self.regs[y] }
                "8XY2" => (x, y) { self.regs[x] = self.regs[x] & self.regs[y] }
                "8XY3" => (x, y) { self.regs[x] = self.regs[x] ^ self.regs[y] }
                "8XY4" => (x, y) {
                    let rx = self.regs[x].0 as u16;
                    let ry = self.regs[y].0 as u16;
                    let r = rx + ry;
                    self.regs[0xF].0 = (r >> 8) as u8;  // >>8 to fetch the carry flag
                    self.regs[x].0 = r as u8;
                }
                "8XY5" => (x, y) {
                    let rx = self.regs[x].0 as i16;
                    let ry = self.regs[y].0 as i16;
                    let r = rx - ry;
                    self.regs[0xF].0 = if r < 0 { 0 } else { 1 };
                    self.regs[x].0 = r as u8;
                }
                "8XY6" => (x, y) {
                    self.regs[0xF] = self.regs[y] & LSB;
                    self.regs[x] = self.regs[y] >> 1;
                }
                "8XY7" => (x, y) {
                    let rx = self.regs[x].0 as i16;
                    let ry = self.regs[y].0 as i16;
                    let r = ry - rx;
                    self.regs[0xF].0 = if r < 0 { 0 } else { 1 };
                    self.regs[x].0 = r as u8;
                }
                "8XYE" => (x, y) {
                    self.regs[0xF] = (self.regs[y] & MSB) >> 7;
                    self.regs[x] = self.regs[y] << 1;
                }
                "9XY0" => (x, y) { if self.regs[x] != self.regs[y] { self.pc = self.pc + INST_SIZE } }
                "ANNN" => (n) { self.reg_i.0 = n }
                "BNNN" => (n) { self.pc.0 = self.regs[0].0 as u16 + n }
                "CXNN" => (x, n) { self.regs[x].0 = random::<u8>() & n as u8 }
                "DXYN" => (x, y, n) {
                    let x = self.regs[x].0 as isize;
                    let y = self.regs[y].0 as isize;
                    let sprite = self.ram.slice(self.reg_i.0 as usize, n as usize);
                    self.regs[0xF].0 = p.draw(x, y, sprite) as u8
                }
                "EX9E" => (x) { if  p.keydown(self.regs[x].0 as usize) { self.pc = self.pc + INST_SIZE } }
                "EXA1" => (x) { if !p.keydown(self.regs[x].0 as usize) { self.pc = self.pc + INST_SIZE } }
                "FX07" => (x) { self.regs[x].0 = self.reg_delay }
                "FX0A" => (x) {
                    if let Some(k) = p.key() {
                        self.regs[x].0 = k;
                    } else {
                        stop = true;
                    }
                }
                "FX15" => (x) { self.reg_delay = self.regs[x].0 }
                "FX18" => (x) { self.reg_sound = self.regs[x].0 }
                "FX1E" => (x) { self.reg_i = self.reg_i + Wrapping(self.regs[x].0 as u16) }
                "FX55" => (x) {
                    let slice: &mut [u8] = self.ram.slice(self.reg_i.0 as usize, x+1);
                    let pairs = slice.iter_mut().zip(self.regs.iter().map(|w| w.0));
                    for (m, x) in pairs { *m = x }
                }
                "FX65" => (x) {
                    let slice: &mut [u8] = self.ram.slice(self.reg_i.0 as usize, x+1);
                    let pairs = slice.iter().zip(self.regs.iter_mut());
                    for (&m, x) in pairs { x.0 = m }
                }
            };

            panic!("unknown instruction {:04X}", inst);
        })();

        stop
    }
}

impl Default for Chip {
    fn default() -> Chip {
        Chip::new()
    }
}

