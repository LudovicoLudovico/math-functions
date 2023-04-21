use std::cmp::PartialEq;
use std::ops::{Add, AddAssign, Mul, Sub};
use crate::algebra::polynomials::Pol;

impl Add for Pol {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result: Vec<f64> = vec![
            0.;
            if self.0.len() >= other.0.len() {
                self.0.len()
            } else {
                other.0.len()
            }
        ];

        for (i, el) in result.iter_mut().enumerate() {
            let first = if i < self.0.len() { self.0[i] } else { 0. };
            let second = if i < other.0.len() { other.0[i] } else { 0. };

            *el = first + second;
        }

        Pol(result)
    }
}

impl AddAssign for Pol {
    fn add_assign(&mut self, rhs: Self) {
        if self.0.len() >= rhs.0.len() {
            for (i, el) in rhs.0.iter().enumerate() {
                self.0[i] += el;
            }
        } else {
            for (i, el) in self.0.iter_mut().enumerate() {
                *el += rhs.0[i];
            }

            for i in self.0.len()..rhs.0.len() {
                self.0.push(rhs.0[i]);
            }
        }
    }
}

impl Sub for Pol {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result: Vec<f64> = vec![
            0.;
            if self.0.len() >= other.0.len() {
                self.0.len()
            } else {
                other.0.len()
            }
        ];

        for (i, el) in result.iter_mut().enumerate() {
            let first = if i < self.0.len() { self.0[i] } else { 0. };
            let second = if i < other.0.len() { other.0[i] } else { 0. };

            *el = first - second;
        }

        Pol(result)
    }
}

impl Mul for Pol {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut result = Pol(vec![0.; self.0.len() + rhs.0.len() - 1]);

        for (i, s_el) in self.0.iter().enumerate() {
            let mut inter = vec![0.; i];
            for r_el in rhs.0.iter() {
                inter.push(s_el * r_el);
            }

            result += Pol(inter);
        }

        result
    }
}

impl Mul<&Pol> for f64 {
    type Output = Pol;

    fn mul(self, rhs: &Pol) -> Self::Output {
        let mut new = Vec::with_capacity(rhs.0.len());
        for el in &rhs.0 {
            new.push(el * self);
        }

        Pol(new)
    }
}

impl<'a> Mul<&'a Pol> for &'a Pol {
    type Output = Pol;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = Pol(vec![0.; self.0.len() + rhs.0.len() - 1]);

        for (i, s_el) in self.0.iter().enumerate() {
            let mut inter = vec![0.; i];
            for r_el in rhs.0.iter() {
                inter.push(s_el * r_el);
            }

            result += Pol(inter);
        }

        result.clone()
    }
}

impl<'a> Mul<&'a Pol> for Pol {
    type Output = Pol;

    fn mul(self, rhs: &'a Pol) -> Self::Output {
        let mut result = Pol(vec![0.; self.0.len() + rhs.0.len() - 1]);

        for (i, s_el) in self.0.iter().enumerate() {
            let mut inter = vec![0.; i];
            for r_el in rhs.0.iter() {
                inter.push(s_el * r_el);
            }

            result += Pol(inter);
        }

        result.clone()
    }
}

impl PartialEq for Pol {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        } else {
            for i in 0..self.0.len() {
                if self.0[i] != other.0[i] {
                    return false;
                }
            }
        }

        true
    }
}
