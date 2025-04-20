use super::Frequency;
use std::ops::{Div, Mul, Rem};

impl Mul for Frequency {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Frequency(self.0 * rhs.0)
    }
}

impl Div for Frequency {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Frequency(self.0 / rhs.0)
    }
}

impl Rem for Frequency {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Frequency(self.0 % rhs.0)
    }
}

impl num_traits::Zero for Frequency {
    fn zero() -> Self {
        Frequency(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl num_traits::One for Frequency {
    fn one() -> Self {
        Frequency(1)
    }
}

impl num_traits::Num for Frequency {
    type FromStrRadixErr = std::num::ParseIntError;

    fn from_str_radix(s: &str, _radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        s.parse::<u64>().map(Frequency)
    }
}
