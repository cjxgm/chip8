//! Utility functions for decoding instructions by patterns.

/// This module is intended to be private, but is made public only for
/// macro usage across module boundaries. It is still considered private.
pub mod private {
    pub struct OpcodeMetadata {
        pub x: u8,
        pub y: u8,
        pub n: u16,
    }

    pub fn decoder_match(inst: u16, inst_pat: &str) -> Option<OpcodeMetadata> {
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
}

/// # Examples
/// ```
/// fn decode(inst: u16) {
///     decode! { inst =>
///         AXY0 (x, y) { ... }
///         AXY1 (x, y) { ... }
///         BXNN (x, n) { ... }
///         CNNX (x, n) { ... }
///         DNXY (x, y, n) { ... }
///         ENNN (n) { ... }
///         F016 () { ... }
///     }
///     panic!("unknown instruction: {:04X}", inst);
/// };
/// ```
macro_rules! decode {
    ($inst: ident => $($inst_pat: tt ($($m: ident),*) $action: block)*) => {
        $(
        match ::decoder::private::decoder_match($inst, stringify!($inst_pat)) {
            Some(::decoder::private::OpcodeMetadata{ $($m,)* .. }) => {
                return $action;
            },
            None => (),
        };
        )*
    };
}

