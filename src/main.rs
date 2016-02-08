extern crate chip;
extern crate term_oss;
use chip::decode;

fn main() {
    decode(0x0123);
    decode(0x7123);
    decode(0x8121);
    decode(0x00E0);
    decode(0x00E1);
}

