use crate::{Field, Monomial, PolyRing};
use std::ops::{Add, Div, Mul, Neg, Sub};

use std::fmt;
use std::fmt::{Debug, Display};

#[derive(PartialEq, Eq, Clone)]
pub struct Polynomial<'lt, F: Field> {
    poly_ring: &'lt PolyRing<F>,
    terms: Vec<(F, Monomial<'lt, F>)>,
}

impl<'lt, F: Field + Debug + Display> Polynomial<'lt, F> {
    pub fn new(
        poly_ring: &'lt PolyRing<F>,
        terms: Vec<(F, Monomial<'lt, F>)>,
    ) -> Polynomial<'lt, F> {
        let mut res = Polynomial {
            poly_ring: poly_ring,
            terms: terms,
        };
        res.normalize();
        return res;
    }

    fn normalize(&mut self) {
        if self.terms.is_empty() {
            return;
        }
        let mut orig = std::mem::replace(&mut self.terms, Vec::<(F, Monomial<'lt, F>)>::new());
        orig.sort_by(|a, b| b.1.cmp(&a.1));
        let mut it = orig.into_iter();
        let mut last = it.next().unwrap();
        for t in it {
            if t.1 == last.1 {
                last.0 += t.0;
            } else {
                if !last.0.is_zero() {
                    self.terms.push(last.clone());
                }
                last = t;
            }
        }
        if !last.0.is_zero() {
            self.terms.push(last.clone());
        }
    }

    pub fn is_zero(&self) -> bool {
        self.terms.is_empty()
    }

    pub fn is_one(&self) -> bool {
        self.terms.len() == 1 && self.terms[0].0 == F::one() && self.terms[0].1.is_one()
    }

    pub fn lm(&self) -> Monomial<'lt, F> {
        assert!(!self.is_zero());
        self.terms[0].1.clone()
    }

    pub fn lt(&self) -> Self {
        assert!(!self.is_zero());
        Polynomial::new(self.poly_ring, vec![self.terms[0].clone()])
    }

    pub fn lc(&self) -> F {
        assert!(!self.is_zero());
        self.terms[0].0
    }

    pub fn s_polynomial(&self, other: &Self) -> Self {
        if self.is_zero() {
            Polynomial::new(self.poly_ring, Vec::new())
        } else {
            let lcm = self.lm().lcm(&other.lm());
            let f = Polynomial::new(
                self.poly_ring,
                vec![(F::one() / self.lc(), lcm.clone() / &self.lm())],
            );
            let g = Polynomial::new(
                self.poly_ring,
                vec![(F::one() / other.lc(), lcm / &other.lm())],
            );
            //println!("({})*({}) - ({})*({})", f, self, g, other);
            f * self.clone() - g * other.clone()
        }
    }
}

impl<'lt, F: Field + Debug> fmt::Debug for Polynomial<'lt, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for term in &self.terms {
            if !term.0.is_one() || term.1.is_one() {
                term.0.fmt(f)?;
            } else {
                write!(f, "+")?;
            }
            Debug::fmt(&term.1, f)?;
        }
        std::result::Result::Ok(())
    }
}

impl<'lt, F: Field + Display> fmt::Display for Polynomial<'lt, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for term in &self.terms {
            if (-term.0).is_one() {
                write!(f, "-")?;
            } else if !term.0.is_one() || term.1.is_one() {
                if first {
                    write!(f, "{}", term.0)?;
                } else {
                    write!(f, "{:+}", term.0)?;
                }
            } else if !first {
                write!(f, "+")?;
            }
            first = false;
            fmt::Display::fmt(&term.1, f)?;
        }
        if first {
            write!(f, "0")?;
        }
        std::result::Result::Ok(())
    }
}

impl<'lt, F: Field + Debug + Display> Add<Self> for Polynomial<'lt, F> {
    type Output = Self;
    fn add(mut self, other: Self) -> Self {
        assert_eq!(self.poly_ring, other.poly_ring);
        for v in other.terms {
            self.terms.push(v);
        }
        self.normalize();
        return self;
    }
}

impl<'lt, F: Field + Debug + Display> Add<F> for Polynomial<'lt, F> {
    type Output = Self;
    fn add(mut self, other: F) -> Self {
        self.terms.push((
            other,
            Monomial::new(self.poly_ring, vec![0; self.poly_ring.variable_count()]),
        ));
        self.normalize();
        return self;
    }
}

impl<'lt, F: Field + Debug + Display> Neg for Polynomial<'lt, F> {
    type Output = Self;
    fn neg(mut self) -> Self {
        for v in &mut self.terms {
            v.0 = -v.0;
        }
        return self;
    }
}

impl<'lt, F: Field + Debug + Display> Sub<Self> for Polynomial<'lt, F> {
    type Output = Self;
    fn sub(mut self, other: Self) -> Self {
        assert_eq!(self.poly_ring, other.poly_ring);
        for mut v in other.terms {
            v.0 = -v.0;
            self.terms.push(v);
        }
        self.normalize();
        return self;
    }
}

impl<'lt, F: Field + Debug + Display> Sub<F> for Polynomial<'lt, F> {
    type Output = Self;
    fn sub(mut self, other: F) -> Self {
        self.terms.push((
            -other,
            Monomial::new(self.poly_ring, vec![0; self.poly_ring.variable_count()]),
        ));
        self.normalize();
        return self;
    }
}

impl<'lt, F: Field + Debug + Display> Mul<Self> for Polynomial<'lt, F> {
    type Output = Self;
    fn mul(mut self, other: Self) -> Self {
        assert_eq!(self.poly_ring, other.poly_ring);
        let orig = std::mem::replace(&mut self.terms, Vec::<(F, Monomial<'lt, F>)>::new());
        for t1 in &orig {
            for t2 in &other.terms {
                self.terms.push((t1.0 * t2.0, t1.1.clone() * &t2.1));
            }
        }
        self.normalize();
        return self;
    }
}

impl<'lt, F: Field + Debug + Display> Mul<F> for Polynomial<'lt, F> {
    type Output = Self;
    fn mul(mut self, other: F) -> Self {
        for t in &mut self.terms {
            t.0 *= other;
        }
        return self;
    }
}

impl<'lt, F: Field + Debug + Display> Div<&Vec<Self>> for Polynomial<'lt, F> {
    type Output = Self;
    fn div(self, divs: &Vec<Self>) -> Self {
        let mut all_zero = true;
        for d in divs {
            if !d.is_zero() {
                all_zero = false;
            }
        }
        assert!(!all_zero);

        /*println!("Div {} with", self);
        for d in divs {
            println!("  {}", d);
        }*/

        let mut p = self.clone();
        let mut r = Vec::<(F, Monomial<'lt, F>)>::new();
        while !p.is_zero() {
            let mut divised = false;
            for div in divs {
                while !p.is_zero() && p.lm().is_divisible(&div.lm()) {
                    let delta = Polynomial::new(
                        self.poly_ring,
                        vec![(p.lc() / div.lc(), p.lm() / &div.lm())],
                    );
                    p = p - delta * div.clone();
                    divised = true;
                }
            }
            if !divised {
                r.push(p.terms.remove(0));
            }
        }
        return Polynomial::new(self.poly_ring, r);
    }
}

impl<'lt, F: Field + Debug + Display> Div<F> for Polynomial<'lt, F> {
    type Output = Self;
    fn div(mut self, other: F) -> Self {
        for t in &mut self.terms {
            t.0 /= other;
        }
        return self;
    }
}
