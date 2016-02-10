
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn modulo() {
        {
            let a: isize = -10;
            let b: isize =  20;
            assert_eq!(a.modulo(b), 10);
        }
    }
}

pub trait Modulo<Divisor = Self> {
    type Output;
    fn modulo(self, div: Divisor) -> Self::Output;
}

impl Modulo for isize {
    type Output = usize;
    fn modulo(self, div: isize) -> Self::Output {
        ((self % div + div) % div) as Self::Output
    }
}

