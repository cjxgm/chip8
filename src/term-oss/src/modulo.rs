
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn positive() {
        let a: isize = 31;
        let b: isize = 20;
        assert_eq!(a.modulo(b), 11);
    }

    #[test]
    fn positive_contained() {
        let a: isize =  9;
        let b: isize = 20;
        assert_eq!(a.modulo(b), 9);
    }

    #[test]
    fn negative() {
        let a: isize = -31;
        let b: isize =  20;
        assert_eq!(a.modulo(b), 9);
    }

    #[test]
    fn negative_contained() {
        let a: isize = -9;
        let b: isize = 20;
        assert_eq!(a.modulo(b), 11);
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

