use crate::{Field, Polynomial};
use std::fmt::{Debug, Display};

pub fn groebner<F: Field + Debug + Display>(polys: &mut Vec<Polynomial<F>>) {
    let mut changed = true;
    while changed {
        changed = false;
        'outer: for i in 0..polys.len() {
            for j in 0..i {
                let s = polys[i].s_polynomial(&polys[j]) / &*polys;
                if !s.is_zero() {
                    polys.push(s);
                    changed = true;
                    break 'outer;
                }
            }
        }
    }
}
