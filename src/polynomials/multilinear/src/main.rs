
use ark_bn254::Fq;
use multilinear::MultiLinearPolynomial;
use num_bigint::{BigInt, Sign};

fn main() {
    // [0,3,2,5]
    // 3(1-a)(b) + 2a(1-b) + 5(1-a)(1-b)
    // 3b - 3ab + 2a - 2ab + 5-5a-5b+5ab
    // 
    let coefficients = vec![
        (0, Fq::from(0)),
        (1, Fq::from(2)),
        (2, Fq::from(0)),
        (3, Fq::from(5)),
    ];
    let mut poly = MultiLinearPolynomial::new(2, coefficients);
    let result = poly.partial_evaluate(1, Fq::from(5));
    println!("Result: {:?}", poly);
}