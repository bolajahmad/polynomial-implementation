use ark_bn254::Fq;
use ark_ff::{PrimeField};

use crate::MultiLinearPolynomial;

pub fn multilinear_polya<F: PrimeField>() -> MultiLinearPolynomial<F> {
    let variables = F::from(3);
    // f(a,b,c) = 2abc + 2ab + 3bc + 4
    let coefficients = vec![
        (0, F::from(4u128)),
        (3, F::from(3u128)),
        (6, F::from(2u128)),
        (7, F::from(2u128)),
    ];

    MultiLinearPolynomial::new(variables, coefficients)
}

pub fn multilinear_polyb<F: PrimeField>() -> MultiLinearPolynomial<F> {
    let variables = F::from(6);
    // f(a,b,c,d,e,f) = 2bcdf + 2abcf + 3bcd + 4abc + 9
    let coefficients = vec![
        (0, F::from(9u128)),
        (28, F::from(3u128)),
        (29, F::from(2u128)),
        (56, F::from(4u128)),
        (57, F::from(2u128)),
    ];

    MultiLinearPolynomial::new(variables, coefficients)
}
