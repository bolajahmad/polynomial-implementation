use std::ops::{Add, Mul};

use ark_bn254::Fq;
use ark_ff::{BigInteger, PrimeField};

use crate::MultiLinearPolynomial;

impl<F: PrimeField> Add for &MultiLinearPolynomial<F> {
    type Output = MultiLinearPolynomial<F>;

    fn add(self, other: &MultiLinearPolynomial<F>) -> MultiLinearPolynomial<F> {
        // Check if the two polynomials have the same number of variables
        if self.variables != other.variables {
            panic!("The two polynomials must have the same number of variables");
        }

        let (mut larger, mut smaller) = if self.coefficients().len() > other.coefficients().len() {
            (self.coefficients().clone(), other.coefficients().clone())
        } else {
            (other.coefficients().clone(), self.coefficients().clone())
        };

        // This will store the results of coefficient addition
        let mut summed_coefficients: Vec<(usize, F)> = vec![];

        for (variables_a, coefficient_a) in larger.iter_mut() {
            // Find coefficient with the same index.
            let existing_coefficient = smaller
                .iter()
                .position(|(pos, _)| pos == variables_a);

            if existing_coefficient.is_some() {
                summed_coefficients.push((
                    *variables_a, 
                    *coefficient_a + smaller[existing_coefficient.unwrap()].1
                ));

                // convert the coefficients to 0 here
                *coefficient_a = F::zero();
                smaller[existing_coefficient.unwrap()].1 = F::zero();
                continue;
            }  

            // If the coefficient does not exist, just add it
            summed_coefficients.push((
                *variables_a, 
                *coefficient_a
            ));
            *coefficient_a = F::zero();
        }

        // append the non-zero coefficients in the smaller polynomial to summed_coefficients
        for (variables_b, coefficient_b) in smaller.iter() {
            if *coefficient_b != F::zero() {
                summed_coefficients.push((
                    *variables_b, 
                    *coefficient_b
                ));
            }
        }
        
        MultiLinearPolynomial::new(
            self.variables,
            summed_coefficients
        )
    }
}

impl<F: PrimeField> Mul for &MultiLinearPolynomial<F> {
 type Output = MultiLinearPolynomial<F>;

 fn mul(self, other: &MultiLinearPolynomial<F>) -> MultiLinearPolynomial<F> {
    assert_eq!(
        self.variables, 
        other.variables, 
        "The two polynomials must have the same number of variables"
    );
    let (larger, smaller) = if self.coefficients().len() > other.coefficients().len() {
        (self.coefficients().clone(), other.coefficients().clone())
    } else {
        (other.coefficients().clone(), self.coefficients().clone())
    };

    let mut product_result = vec![];

    // For each index in the larger polynomial, multiply it with the smaller polynomial
    // Add the variable of each to get the new position
    for (variables_a, coefficient_a) in larger.iter() {
        for (variables_b, coefficient_b) in smaller.iter() {
            product_result.push((
                variables_a + variables_b, 
                *coefficient_a * *coefficient_b
            ))
        }                                                                                                                                                                                                                                                                                                                                                 
    }
    let total_variables = if self.variables > other.variables {
        self.variables
    } else {
        other.variables
    };

    MultiLinearPolynomial::new(
        total_variables,
        product_result
    )
 }
}

#[cfg(test)]
mod tests {
    use ark_bn254::Fq;
    use num_bigint::BigInt;

    use super::*;

    #[test]
    fn should_add_multilinear_polynomials() {
        let variables = 4usize;
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

    #[test]
    fn should_do_multiply() {
        // polya = (1 - a) ([(0, 1), (2, -1)])
        // polyb = b    ([(0, 0), (1, 1)])

        let polya = MultiLinearPolynomial::new(
            2,
            vec![
                (0, Fq::from(1)),
                (2, -Fq::from(1))
            ]
        );
        let polyb = MultiLinearPolynomial::new(
            2,
            vec![
                (0, Fq::from(0)),
                (1, Fq::from(1))
            ]
        );
        let poly_product = &polya * &polyb;
        assert_eq!(poly_product.coefficients().len(), 2);
        assert_eq!(poly_product.coefficients()[0].0, 1);
        assert_eq!(poly_product.coefficients()[1].1, Fq::from(-1));

        let polyc = MultiLinearPolynomial::new(
            2,
            vec![
                (0, Fq::from(1)),
                (1, -Fq::from(1))
            ]
        );
        let poly_product = &polyc * &polya;
        println!("Poly product: {:?}", poly_product);
        assert_eq!(poly_product.coefficients().len(), 4);
        assert_eq!(poly_product.coefficients()[0].0, 0);
        assert_eq!(poly_product.coefficients()[2].1, Fq::from(-1));
    }
}