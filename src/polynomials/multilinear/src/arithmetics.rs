use std::ops::Add;

use ark_bn254::Fq;
use ark_ff::{BigInteger, PrimeField};

use crate::MultiLinearPolynomial;

impl<F: PrimeField> Add for &MultiLinearPolynomial<F> {
    type Output = MultiLinearPolynomial<F>;

    fn add(self, other: &MultiLinearPolynomial<F>) -> MultiLinearPolynomial<F> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use ark_bn254::Fq;

    use super::*;

    // #[test]
    fn should_add_multilinear_polynomials() {
        let variables = Fq::from(4);
        // 3x + 5
        let poly_a = MultiLinearPolynomial::new(
            variables,
            vec![
                (0, Fq::from(5)),
                (8, Fq::from(2)),
                (4, Fq::from(3))
            ]
        );
        let poly_b = MultiLinearPolynomial::new(
            variables,
            vec![
                (0, Fq::from(5)),
                (2, Fq::from(2)),
                (1, Fq::from(3))
            ]
        );

        let poly_sum = &poly_a + &poly_b;

        assert_eq!(poly_sum.coefficients().len(), 5);
    }
}