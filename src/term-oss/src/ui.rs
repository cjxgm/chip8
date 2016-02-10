
extern crate rustbox;
extern crate unicode_width;

use std::iter;
use rustbox::{RustBox, Style, Color};
use rustbox::keyboard::Key;
use rustbox::Event;
use unicode_width::UnicodeWidthStr;
use modulo::Modulo;

#[cfg(test)]
mod test {
    use super::*;
    use std::thread;

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
        thread::sleep_ms(500);
        for _ in 0..8 {
            t.flip_sprites(10, 23, &[
                           0b10100101,
                           0b11000011,
                           0b00111100,
                           0b11111111,
                           0b11000011]);
            t.paint();
            thread::sleep_ms(100);
        }
        thread::sleep_ms(500);
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
            thread::sleep_ms(100);
        }
        t.clear();
        t.paint();
        thread::sleep_ms(1000);
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

    text_style: StyleComplex,
    /// `[0]` for style of OFF.
    /// `[1]` for style of ON.
    cell_styles: [StyleComplex; 2],
}

impl Terminal {
    pub fn new(pixel_size: (usize, usize)) -> Terminal {
        Terminal {
            rb: RustBox::init(Default::default()).unwrap(),
            pixel_w: pixel_size.0,
            pixel_h: pixel_size.1,
            screen: vec![false; pixel_size.0 * pixel_size.1],
            text_style: StyleComplex(rustbox::RB_BOLD, Color::White, Color::Black),
            cell_styles: [
                StyleComplex(rustbox::RB_NORMAL, Color::Black, Color::White),
                StyleComplex(rustbox::RB_NORMAL, Color::Black, Color::Blue),
            ],
        }
    }

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

    /// Returns the value after flipping.
    fn flip(&mut self, x: isize, y: isize) -> bool {
        let x = x.modulo(self.pixel_w as isize);
        let y = y.modulo(self.pixel_h as isize);
        let i = self.pixel_w * y + x;
        let on = self.screen.iter_mut().nth(i).unwrap();
        *on = !*on;
        *on
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


    /// Print text with x centered, single line only.
    /// Long line will be truncated, with "…" appended.
    fn print_centering_x(&self, y: usize, line: &str) {
        let rb = &self.rb;
        let st = &self.text_style;
        let dw = rb.width() as isize - line.width() as isize;
        let x = dw / 2 + dw % 2;
        if x < 0 {
            if line != "…" {
                let shorter = line[..line.len()/2].to_string() + "…";
                self.print_centering_x(y, &shorter);
            }
            return;
        }
        rb.print(x as usize, y, st.0, st.1, st.2, line);
    }

    /// Print text in the center of screen, support multiple lines,
    /// but each line must fit into one single line, or
    /// it will be truncated, with "…" appended.
    /// Too many lines will cause them truncated vertically, with "…" as the last line.
    fn print_centered(&self, text: &str) {
        let rb = &self.rb;
        let h = text.lines().count();
        let dh = rb.height() as isize - h as isize;
        let y = dh / 2 + dh % 2;
        if y < 0 {
            if h > 1 {
                let less = text.lines()
                    .take(h/2)
                    .map(|line| line.to_string() + "\n")
                    .collect::<String>() + "…";
                self.print_centered(&less);
            }
            return;
        }

        let mut y = y;
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

