//! # User Interaction
//!
//! Handling terminal input and output.
//! Timer can be faked by wait-timeout.

extern crate time;
extern crate rustbox;
extern crate unicode_width;

use std::iter;
use std::time::Duration;
use rustbox::{RustBox, Style, Color, Event};
use rustbox::keyboard::Key;
use unicode_width::UnicodeWidthStr;
use modulo::Modulo;
use time::precise_time_ns as now_ns;
use self::from_nanos::FromNanos;    // add Duration::from_nanos(u64);
use self::key_map::key_from_char;

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Duration;

    #[test]
    fn term() {
        let mut t: Terminal = Default::default();
        t.flip_sprites(10, 20, &[
                       0b10100101,
                       0b11000011,
                       0b00111100,
                       0b11111111,
                       0b11000011]);
        t.paint();
        if t.pump_events(Duration::from_millis(500)) { return }
        for _ in 0..8 {
            t.flip_sprites(10, 23, &[
                           0b10100101,
                           0b11000011,
                           0b00111100,
                           0b11111111,
                           0b11000011]);
            t.paint();
            if t.pump_events(Duration::from_millis(100)) { return }
            if t.keydown(5) { break }
        }
        if t.pump_events(Duration::from_millis(500)) { return }
        for x in 0..20 {
            let x = x as isize - 10isize;
            t.flip_sprites(x, 23, &[
                           0b10100101,
                           0b11000011,
                           0b00111100,
                           0b11111111,
                           0b11000011]);
            t.paint();
            t.flip_sprites(x, 23, &[
                           0b10100101,
                           0b11000011,
                           0b00111100,
                           0b11111111,
                           0b11000011]);
            if t.pump_events(Duration::from_millis(100)) { return }
            if t.keydown(0) { break }
        }
        t.clear();
        t.paint();
        if t.pump_events(Duration::from_millis(1000)) { return }
    }
}


mod key_map {
    //                             0123456789ABCDEF
    const KEY_MAP: &'static str = "x123qweasdzcvfr4";
    // QWERTY KEYBOARD               HEX KEYBOARD
    //     1 2 3 4                     1 2 3 F
    //     q w e r                     4 5 6 E
    //     a s d f                     7 8 9 D
    //     z x c v                     A 0 B C

    pub fn key_from_char(ch: char) -> Option<usize> {
        KEY_MAP.chars()
            .enumerate()
            .filter_map(|(i, c)| select(c == ch, i))
            .next()
    }

    fn select<T>(pred: bool, x: T) -> Option<T> {
        if pred { Some(x) } else { None }
    }
}


/// StyleComplex(style, fg, bg);
struct StyleComplex(Style, Color, Color);

pub struct Terminal {
    rb: RustBox,
    /// 1 pixel width = 2 chars horizontally
    pixel_w: usize,
    /// 1 pixel height = 1 char vertically
    pixel_h: usize,
    /// Screen of CHIP-8.
    screen: Vec<bool>,
    /// keydown status in a frame.
    keydowns: [bool; 16],

    text_style: StyleComplex,
    /// `[0]` for style of OFF.
    /// `[1]` for style of ON.
    cell_styles: [StyleComplex; 2],
}

impl Terminal {
    pub fn paint(&self) {
        let rb = &self.rb;
        rb.clear();

        let tw = rb.width();
        let th = rb.height();
        let w = self.pixel_w * 2;
        let h = self.pixel_h;

        if tw < w || th < h {   // terminal size too small
            self.print_centered(&format!("{}x{} is too small\n\
                                         At least {}x{} is required",
                                         rb.width(),
                                         rb.height(),
                                         self.pixel_w * 2,
                                         self.pixel_h));
            self.draw_border();
        } else {                // terminal size big enough
            let     x = (tw - w) / 2;   // "no underflow" guaranteed by the outer "if".
            let mut y = (th - h) / 2;
            for row in self.screen.chunks(self.pixel_w) {
                let mut x = x;
                for st in row.iter().map(|&on| &self.cell_styles[on as usize]) {
                    rb.print(x, y, st.0, st.1, st.2, "  ");
                    x += 2;
                }
                y += 1;
            }
        }
        rb.present();
    }

    pub fn clear(&mut self) {
        for x in self.screen.iter_mut() {
            *x = false;
        }
    }

    /// Returns true if anything has been flipped to false.
    /// Returns false if nothing has been flipped to false.
    pub fn flip_sprites(&mut self, x: isize, y: isize, sprites: &[u8]) -> bool {
        let mut flip_to_false = false;
        let mut y = y;
        for &sprite in sprites {
            flip_to_false = self.flip_sprite(x, y, sprite) || flip_to_false;
            y += 1;
        }
        flip_to_false
    }

    /// Returns true for quit-request.
    pub fn pump_events(&mut self, frame_time: Duration) -> bool {
        self.keydowns = [false; 16];

        let mut remaining = frame_time;
        loop {
            let start = now_ns();
            let ev = self.rb.peek_event(remaining, false).unwrap();
            let dura = Duration::from_nanos(now_ns() - start);

            if self.handle_event(ev) { return true }

            if remaining <= dura { break }
            remaining = remaining - dura;
        }

        false
    }

    pub fn keydown(&self, which: usize) -> bool {
        self.keydowns[which]
    }

    pub fn key(&self) -> Option<u8> {
        loop {
            let ev = self.rb.poll_event(false).unwrap();
            match ev {
                Event::ResizeEvent(..) => (),
                Event::MouseEvent(..) => (),
                Event::KeyEvent(Key::Esc) => return None,
                Event::KeyEvent(Key::Char(ch)) => {
                    if let Some(k) = key_from_char(ch) {
                        return Some(k as u8);
                    }
                },
                Event::KeyEvent(_) => (),
                _ => unreachable!(),
            }
            self.paint();
        }
    }


    /// Assume self.keydowns == [false; 16] on the start of a frame.
    /// Returns true for quit-request.
    fn handle_event(&mut self, ev: Event) -> bool {
        match ev {
            Event::ResizeEvent(..) => (),
            Event::MouseEvent(..) => (),
            Event::KeyEvent(Key::Esc) => return true,
            Event::KeyEvent(Key::Char(ch)) => {
                if let Some(k) = key_from_char(ch) {
                    self.keydowns[k] = true;
                }
            },
            Event::KeyEvent(_) => (),
            Event::NoEvent => (),
            _ => unreachable!(),
        }

        false
    }

    /// Only allow construction from Default trait
    fn new(pixel_size: (usize, usize)) -> Terminal {
        Terminal {
            rb: RustBox::init(Default::default()).unwrap(),
            pixel_w: pixel_size.0,
            pixel_h: pixel_size.1,
            screen: vec![false; pixel_size.0 * pixel_size.1],
            keydowns: [false; 16],
            text_style: StyleComplex(rustbox::RB_BOLD, Color::White, Color::Black),
            cell_styles: [
                StyleComplex(rustbox::RB_NORMAL, Color::Black, Color::White),
                StyleComplex(rustbox::RB_NORMAL, Color::Black, Color::Blue),
            ],
        }
    }

    /// Returns true if anything has been flipped to false.
    /// Returns false if nothing has been flipped to false.
    fn flip_sprite(&mut self, x: isize, y: isize, sprite: u8) -> bool {
        let mut flip_to_false = false;
        let mut x = x;
        for i in (0..8).rev() {
            if sprite & (1 << i) != 0 {
                if self.flip(x, y) == false {
                    flip_to_false = true;
                }
            }
            x += 1;
        }
        flip_to_false
    }

    /// Returns the value after flipping.
    fn flip(&mut self, x: isize, y: isize) -> bool {
        let x = x.modulo(self.pixel_w as isize);
        let y = y.modulo(self.pixel_h as isize);
        let i = self.pixel_w * y + x;
        let on = self.screen.iter_mut().nth(i).unwrap();
        *on = !*on;
        *on
    }

    /// Print text with x centered, single line only.
    /// Long line will be truncated, with "…" appended.
    fn print_centering_x(&self, y: usize, line: &str) {
        let rb = &self.rb;
        let st = &self.text_style;
        let dw = rb.width() as isize - 2 - line.width() as isize;   // -2 for the border
        let x = dw / 2 + dw % 2;
        if x < 0 {
            if line != "…" {
                let len = line.chars().count() - 1;                 // -1 for removing the last "…" (always assuming it's there)
                let shorter = line.chars().take(len / 2).collect::<String>() + "…";
                self.print_centering_x(y, &shorter);
            }
            return;
        }
        rb.print(x as usize + 1, y, st.0, st.1, st.2, line);        // +1 for the border
    }

    /// Print text in the center of screen, support multiple lines,
    /// but each line must fit into one single line, or
    /// it will be truncated, with "…" appended.
    /// Too many lines will cause them truncated vertically, with "…" as the last line.
    fn print_centered(&self, text: &str) {
        let rb = &self.rb;
        let h = text.lines().count();
        let dh = rb.height() as isize - 2 - h as isize;             // -2 for the border
        let y = dh / 2 + dh % 2;
        if y < 0 {
            if h > 2 {
                let less = text.lines()
                    .take((h-1)/2)
                    .map(|line| line.to_string() + "\n")
                    .collect::<String>() + "…";
                self.print_centered(&less);
            }
            else if h > 1 {
                self.print_centered("…");
            }
            return;
        }

        let mut y = y + 1;                                          // +1 for the border
        for line in text.lines() {
            self.print_centering_x(y as usize, line);
            y += 1;
        }
    }

    /// Must not exceed the screen.
    fn draw_box(&self, x: usize, y: usize, w: usize, h: usize) {
        if w < 2 || h < 2 { return }

        let rb = &self.rb;
        let st = &self.text_style;

        // x lines
        {
            let line: String = iter::repeat("─").take(w-2).collect();
            rb.print(x+1, y,     st.0, st.1, st.2, &line);
            rb.print(x+1, y+h-1, st.0, st.1, st.2, &line);
        }

        // y lines
        for i in 1..h-1 {
            rb.print(x,     y+i, st.0, st.1, st.2, "│");
            rb.print(x+w-1, y+i, st.0, st.1, st.2, "│");
        }

        // corners
        rb.print(x,     y,     st.0, st.1, st.2, "┌");
        rb.print(x,     y+h-1, st.0, st.1, st.2, "└");
        rb.print(x+w-1, y,     st.0, st.1, st.2, "┐");
        rb.print(x+w-1, y+h-1, st.0, st.1, st.2, "┘");
    }

    fn draw_border(&self) {
        let rb = &self.rb;
        self.draw_box(0, 0, rb.width(), rb.height());
    }
}

impl Default for Terminal {
    fn default() -> Terminal {
        Terminal::new((64, 32))
    }
}

/// Provide from_nanos to std::time::Duration
mod from_nanos {
    use std::time::Duration;

    const NANOS_PER_SEC: u64 = 1_000_000_000;

    pub trait FromNanos {
        fn from_nanos(nanos: u64) -> Duration;
    }

    impl FromNanos for Duration {
        fn from_nanos(nanos: u64) -> Duration {
            let secs = nanos / NANOS_PER_SEC;
            let nanos = (nanos % NANOS_PER_SEC) as u32;
            Duration::new(secs, nanos)
        }
    }
}

