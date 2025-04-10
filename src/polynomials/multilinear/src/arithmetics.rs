use std::ops::Add;

use ark_ff::PrimeField;

use crate::MultiLinearPolynomial;

impl<F: PrimeField> Add for &MultiLinearPolynomial<F> {
    type Output = MultiLinearPolynomial<F>;

    fn add(self, other: &MultiLinearPolynomial<F>) -> MultiLinearPolynomial<F> {
        // Can only add if they have same variables?
        // Totally different variables
        assert!(self.variables == other.variables, "The variables must be the same");
        // assume f(a,b) = 2a + 3b + 5 -> [(0,5), (1, 3), (2, 2)]
        // f(c, d) = 2c + 3d + 5 -> [(0, 5), ()]

        let dense_self = self.fill_with_zero();
        let dense_other = other.fill_with_zero();
        println!("Dense self: {:?}", dense_self);
        println!("Dense other: {:?}", dense_other);

        let mut sum_results = Vec::new();

        for (idx, (variables, coefficient)) in dense_other.coefficients().iter().enumerate() {
            sum_results.push((*variables, (*coefficient + dense_self.coefficients()[idx].1)));
        };

        let mut sum_poly = MultiLinearPolynomial::new(self.variables, sum_results);
        println!("Sum poly: {:?}", sum_poly);
        sum_poly.remove_zero_coefficients();
        sum_poly
    }
}

#[cfg(test)]
mod tests {
    use ark_bn254::Fq;

    use super::*;

    #[test]
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