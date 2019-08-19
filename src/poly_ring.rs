use crate::Field;

use crate::{Monomial, Polynomial};
use std::fmt;
use std::marker::PhantomData;
use std::vec::Vec;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PolyRing<F: Field> {
    field: PhantomData<F>,
    variables: Vec<&'static str>,
}

impl<F: Field> PolyRing<F> {
    pub fn new(variables: Vec<&'static str>) -> PolyRing<F> {
        PolyRing {
            variables: variables,
            field: PhantomData,
        }
    }

    pub fn variable_count(&self) -> usize {
        self.variables.len()
    }
}

impl<F: Field> std::ops::Index<usize> for PolyRing<F> {
    type Output = str;
    fn index(&self, index: usize) -> &str {
        self.variables[index]
    }
}

impl<F: Field + fmt::Debug + fmt::Display> PolyRing<F> {
    pub fn monomial(&self, degrees: Vec<u64>) -> Monomial<F> {
        Monomial::new(self, degrees)
    }

    pub fn variables(&self) -> Vec<Polynomial<F>> {
        let mut res = Vec::new();
        for i in 0..self.variables.len() {
            let mut deg = vec![0; self.variable_count()];
            deg[i] = 1;
            res.push(Polynomial::new(&self, vec![(F::one(), self.monomial(deg))]));
        }
        res
    }
}
