use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Ratio {
    a: i32,
    b: i32,
}

impl Ratio {
    pub fn new(a: i32, b: i32) -> Self {
        assert!(b != 0, "denominator must not be zero");
        let sign = if b < 0 { -1 } else { 1 };
        let (a, b) = (a * sign, b * sign);
        let g = gcd(a, b);
        Ratio { a: a / g, b: b / g }
    }

    pub fn numerator(&self) -> i32 {
        self.a
    }

    pub fn denominator(&self) -> i32 {
        self.b
    }

    pub fn is_zero(&self) -> bool {
        self.a == 0
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

impl Add for Ratio {
    type Output = Ratio;

    fn add(self, other: Ratio) -> Ratio {
        Ratio::new(self.a * other.b + other.a * self.b, self.b * other.b)
    }
}

impl Sub for Ratio {
    type Output = Ratio;

    fn sub(self, other: Ratio) -> Ratio {
        Ratio::new(self.a * other.b - other.a * self.b, self.b * other.b)
    }
}

impl Mul for Ratio {
    type Output = Ratio;

    fn mul(self, other: Ratio) -> Ratio {
        Ratio::new(self.a * other.a, self.b * other.b)
    }
}

impl Div for Ratio {
    type Output = Ratio;

    fn div(self, other: Ratio) -> Ratio {
        assert!(!other.is_zero(), "division by zero");
        Ratio::new(self.a * other.b, self.b * other.a)
    }
}

impl Neg for Ratio {
    type Output = Ratio;

    fn neg(self) -> Ratio {
        Ratio {
            a: -self.a,
            b: self.b,
        }
    }
}

impl From<i32> for Ratio {
    fn from(a: i32) -> Ratio {
        Ratio::new(a, 1)
    }
}

impl fmt::Display for Ratio {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.b == 1 {
            write!(f, "{}", self.a)
        } else {
            write!(f, "{}/{}", self.a, self.b)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Ratio;

    #[test]
    fn reduces_and_normalizes_sign() {
        assert_eq!(Ratio::new(6, -4), Ratio::new(-3, 2));
        assert_eq!(Ratio::new(0, 5), Ratio::new(0, 1));
        assert_eq!(Ratio::new(3, 1), Ratio::from(3));
    }

    #[test]
    fn arithmetic() {
        assert_eq!(Ratio::new(1, 2) + Ratio::new(1, 3), Ratio::new(5, 6));
        assert_eq!(Ratio::new(1, 2) - Ratio::new(1, 2), Ratio::new(0, 1));
        assert_eq!(Ratio::new(2, 3) * Ratio::new(3, 4), Ratio::new(1, 2));
        assert_eq!(Ratio::new(1, 2) / Ratio::new(3, 4), Ratio::new(2, 3));
    }
}
