use num::BigRational;
use buchberger::{groebner, PolyRing};

fn f(x: f64) -> BigRational {
    BigRational::from_float(x).expect("Expected a valid float")
}

fn main() {
    let poly_ring = PolyRing::<BigRational>::new(vec!["a", "b", "c", "λ"]);
    let x = poly_ring.variables();

    let g1 = x[0].clone() + x[1].clone() + x[2].clone() - f(2.0);
    let g2 = x[1].clone() * x[2].clone() - x[1].clone() - x[2].clone() + x[3].clone() + f(1.0);
    let g3 = x[0].clone() * x[1].clone() - x[0].clone() - x[1].clone() + x[3].clone() + f(1.0);
    let g4 = x[2].clone() * x[0].clone() - x[2].clone() - x[0].clone() + x[3].clone() + f(1.0);

    println!("g1={}, g2={}, g3={}, g4={}", g1, g2, g3, g4);
    println!("S={}", g1.s_polynomial(&g2));

    let mut ideal = vec![g1, g2, g3, g4];
    groebner(&mut ideal);

    for (i, poly) in ideal.iter().enumerate() {
        println!("g{}={},", i + 1, poly);
    }
}