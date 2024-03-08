use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Rational {
    num: i32,
    den: u32,
}

impl Rational {
    pub fn new(num: i32, den: i32) -> Self {
        let mut opposite_sign = false;
        if num * den < 0 {
            opposite_sign = true;
        }

        let mut num = num.abs();
        let den: u32 = den.abs() as u32;

        let gcd = gcd(num as u32, den);
        if opposite_sign {
            num *= -1;
        }

        Rational {
            num: num / gcd as i32,
            den: den / gcd,
        }
    }
    pub fn new_from_int(num: i32) -> Self {
        Rational { num, den: 1 }
    }
    pub fn zero() -> Self {
        Rational { num: 0, den: 1 }
    }
    pub fn is_integer(&self) -> bool {
        self.den == 1
    }
    pub fn eval(&self) -> f64 {
        self.num as f64 / (self.den as f64)
    }
    pub fn num(&self) -> i32 {
        self.num
    }
    pub fn den(&self) -> i32 {
        self.den as i32
    }
}

impl PartialEq<i32> for Rational {
    fn eq(&self, other: &i32) -> bool {
        self.num == *other && self.den == 1
    }
}
impl PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.den == other.den
    }
}

impl Add for Rational {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let lcm = lcm(self.den, rhs.den) as i32;
        Rational::new(
            lcm / self.den as i32 * self.num + lcm / rhs.den as i32 * rhs.num,
            lcm,
        )
    }
}
impl Add<i32> for Rational {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        let lcm = self.den as i32;
        Rational::new(lcm / self.den as i32 * self.num + lcm * rhs, lcm)
    }
}
impl Sub for Rational {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let lcm = lcm(self.den, rhs.den);
        Rational::new(
            lcm as i32 / self.den as i32 * self.num - lcm as i32 / rhs.den as i32 * rhs.num,
            lcm as i32,
        )
    }
}

impl Sub<i32> for Rational {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        let lcm = self.den;
        Rational::new(
            lcm as i32 / self.den as i32 * self.num - lcm as i32 * rhs,
            lcm as i32,
        )
    }
}
impl Mul for Rational {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Rational::new(self.num * rhs.num, (self.den * rhs.den) as i32)
    }
}
impl Div for Rational {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Rational::new(self.num * rhs.den as i32, self.den as i32 * rhs.num)
    }
}

pub fn gcd(mut n: u32, mut m: u32) -> u32 {
    assert!(n != 0 || m != 0);
    if n == 0 {
        std::mem::swap(&mut n, &mut m);
    }
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}
fn lcm(first: u32, second: u32) -> u32 {
    first * second / gcd(first, second)
}

#[test]
fn test_rat_operators() {
    let a = Rational::new(-1, 2);
    let b = Rational::new(2, 2);
    assert_eq!(a + b, Rational::new(1, 2));

    let a = Rational::new(-1, 2);
    let b = Rational::new(2, 2);
    assert_eq!(a - b, Rational::new(-3, 2));

    let a = Rational::new(-1, 2);
    let b = Rational::new(3, 2);
    assert_eq!(a * b, Rational::new(-3, 4));

    let a = Rational::new(-1, 2);
    let b = Rational::new(2, 3);
    assert_eq!(a / b, Rational::new(-3, 4));
}
