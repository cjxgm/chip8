extern crate chip8;

struct OpcodeMetadata {
    x: u8,
    y: u8,
    n: u16,
}

fn decoder_match(inst: u16, inst_pat: &str) -> Option<OpcodeMetadata> {
    // sanitize
    if inst_pat.len() != 4 { panic!("LOGIC FAILURE: blame the programmer!") }
    let ips: Vec<(_, _)> = format!("{:04X}", inst).chars().zip(inst_pat.chars()).collect();

    // match
    let matched = ips.iter().all(|&(i, p)| if p.is_digit(16) { i == p } else { true });
    if !matched { return None }

    // extract metadata
    let extract = |mask| ips.iter()
        .filter_map(|&(i, p)| if p == mask { Some(i) } else { None })
        .collect::<String>();
    let extract_u16 = |mask| {
        let e = extract(mask);
        if e.len() == 0 { 0 }
        else { u16::from_str_radix(&e, 16).unwrap() }
    };
    Some(OpcodeMetadata {
        x: extract_u16('X') as u8,
        y: extract_u16('Y') as u8,
        n: extract_u16('N'),
    })
}

/// # Examples
/// ```
/// let decode = decoder! {
///     AXY0 (x, y) { ... }
///     AXY1 (x, y) { ... }
///     BXNN (x, n) { ... }
///     CNNN (n) { ... }
///     D012 () { ... }
/// };
/// decode(inst);
/// ```
macro_rules! decoder {
    ($($inst_pat: tt ($($m: ident),*) $action: block)*) => {
        {
            fn decode(inst: u16) {
                $(
                match decoder_match(inst, stringify!($inst_pat)) {
                    Some(OpcodeMetadata{ $($m,)* .. }) => {
                        $action;
                        return;
                    },
                    None => (),
                };
                )*
                panic!("unknown instruction {:04X}", inst);
            }
            decode
        }
    };
}

fn decode(inst: u16) {
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

fn main() {
    decode(0x0123);
    decode(0x7123);
    decode(0x8121);
    decode(0x00E0);
    decode(0x00E1);
}

