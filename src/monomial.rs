use crate::{Field, PolyRing};

use std::fmt;
use std::fmt::Debug;

use std::cmp::{max, min, Ord, Ordering, PartialOrd};
use std::ops::{Div, DivAssign, Mul, MulAssign};
use std::vec::Vec;

#[derive(PartialEq, Eq, Clone)]
pub struct Monomial<'lt, F: Field> {
    poly_ring: &'lt PolyRing<F>,
    degrees: Vec<u64>,
}

impl<'lt, F: Field + Debug + fmt::Display> Monomial<'lt, F> {
    pub fn new(poly_ring: &'lt PolyRing<F>, degrees: Vec<u64>) -> Monomial<'lt, F> {
        assert_eq!(poly_ring.variable_count(), degrees.len());
        Monomial {
            poly_ring: poly_ring,
            degrees: degrees,
        }
    }

    pub fn gcd(&self, other: &Self) -> Self {
        assert_eq!(self.variable_count(), other.variable_count());
        let mut res = vec![0; self.variable_count()];
        for i in 0..self.variable_count() {
            res[i] = min(self.degrees[i], other.degrees[i]);
        }
        self.poly_ring.monomial(res)
    }

    pub fn lcm(&self, other: &Self) -> Self {
        assert_eq!(self.variable_count(), other.variable_count());
        let mut res = vec![0; self.variable_count()];
        for i in 0..self.variable_count() {
            res[i] = max(self.degrees[i], other.degrees[i]);
        }
        self.poly_ring.monomial(res)
    }

    pub fn is_divisible(&self, other: &Self) -> bool {
        assert_eq!(self.poly_ring, other.poly_ring);
        for (i, deg) in other.degrees.iter().enumerate() {
            if self.degrees[i] < *deg {
                return false;
            }
        }
        return true;
    }
}

impl<'lt, F: Field> Monomial<'lt, F> {
    pub fn variable_count(&self) -> usize {
        self.degrees.len()
    }

    pub fn is_one(&self) -> bool {
        let mut res = true;
        for d in &self.degrees {
            if *d > 0 {
                res = false;
            }
        }
        return res;
    }
}

impl<'lt, F: Field> Ord for Monomial<'lt, F> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.degrees.cmp(&other.degrees)
    }
}

impl<'lt, F: Field> PartialOrd for Monomial<'lt, F> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'lt, F: Field> Debug for Monomial<'lt, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, deg) in self.degrees.iter().enumerate() {
            if *deg == 0 {
                continue;
            }
            write!(f, "{}", &self.poly_ring[i])?;
            if *deg == 1 {
                continue;
            }
            let s: String = deg
                .to_string()
                .chars()
                .map(|x| match x {
                    '0' => '⁰',
                    '1' => '¹',
                    '2' => '²',
                    '3' => '³',
                    '4' => '⁴',
                    '5' => '⁵',
                    '6' => '⁶',
                    '7' => '⁷',
                    '8' => '⁸',
                    '9' => '⁹',
                    _ => x,
                })
                .collect();
            write!(f, "{}", s)?;
        }
        std::result::Result::Ok(())
    }
}

impl<'lt, F: Field> fmt::Display for Monomial<'lt, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'lt, F: Field + Debug + fmt::Display> Mul<&Self> for Monomial<'lt, F> {
    type Output = Self;
    fn mul(self, other: &Self) -> Monomial<'lt, F> {
        assert_eq!(self.poly_ring, other.poly_ring);
        let mut degrees = self.degrees;
        for (i, deg) in other.degrees.iter().enumerate() {
            degrees[i] += deg;
        }
        self.poly_ring.monomial(degrees)
    }
}

impl<'lt, F: Field + Debug + fmt::Display> Mul<Self> for &Monomial<'lt, F> {
    type Output = Monomial<'lt, F>;
    fn mul(self, other: Self) -> Monomial<'lt, F> {
        assert_eq!(self.poly_ring, other.poly_ring);
        let mut degrees = self.degrees.clone();
        for (i, deg) in other.degrees.iter().enumerate() {
            degrees[i] += deg;
        }
        self.poly_ring.monomial(degrees)
    }
}

impl<'lt, F: Field + Debug + fmt::Display> MulAssign<&Self> for Monomial<'lt, F> {
    fn mul_assign(&mut self, other: &Self) {
        *self = &*self * other;
    }
}

impl<'lt, F: Field + Debug + fmt::Display> Div<&Self> for Monomial<'lt, F> {
    type Output = Self;
    fn div(self, other: &Self) -> Monomial<'lt, F> {
        assert_eq!(self.poly_ring, other.poly_ring);
        let mut degrees = self.degrees;
        for (i, deg) in other.degrees.iter().enumerate() {
            if degrees[i] < *deg {
                panic!("Negative exponents");
            }
            degrees[i] -= deg;
        }
        self.poly_ring.monomial(degrees)
    }
}

impl<'lt, F: Field + Debug + fmt::Display> Div<Self> for &Monomial<'lt, F> {
    type Output = Monomial<'lt, F>;
    fn div(self, other: Self) -> Monomial<'lt, F> {
        assert_eq!(self.poly_ring, other.poly_ring);
        let mut degrees = self.degrees.clone();
        for (i, deg) in other.degrees.iter().enumerate() {
            if degrees[i] < *deg {
                panic!("Negative exponents");
            }
            degrees[i] -= deg;
        }
        self.poly_ring.monomial(degrees)
    }
}

impl<'lt, F: Field + Debug + fmt::Display> DivAssign<&Self> for Monomial<'lt, F> {
    fn div_assign(&mut self, other: &Self) {
        *self = &*self / other;
    }
}
