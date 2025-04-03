use std::ops::{Add, Mul};

use ark_ff::{PrimeField, Zero};

use crate::Polynomials;

impl<F: PrimeField> Add for &Polynomials<F> {
    type Output = Polynomials<F>;

    fn add(self, other: &Polynomials<F>) -> Polynomials<F> {
        // Get the higher one of the 2.
        let (higher_degree, lower_degree) = if self.coefficients().len() > other.coefficients().len() {
            (self.coefficients(), other.coefficients())
        } else {
            (other.coefficients(), self.coefficients())
        };
        let mut coefficients = Vec::with_capacity(higher_degree.len());

        // Use the index also.
        for (index, value) in lower_degree.iter().enumerate() {
            // add the ld[i] to the hd[i]
            coefficients.push(*value + higher_degree[index])
        }
        // copy the rest of the coefficients in higher_degree polynomial
        coefficients.extend_from_slice(&higher_degree[lower_degree.len()..]);

        // Create a new polynomial with the coefficients and the degree of the higher degree polynomial
        Polynomials::new(coefficients).unwrap()
    }
}

impl<F: PrimeField> Mul for &Polynomials<F> {
    type Output = Polynomials<F>;

    fn mul(self, other: &Polynomials<F>) -> Polynomials<F> {
        let output_degree = self.coefficients().len() + other.coefficients().len() - 2;

        let mut coefficients = vec![Zero::zero(); output_degree + 1];
        for (i, coeff1) in self.coefficients().iter().enumerate() {
            for (j, coeff2) in other.coefficients().iter().enumerate() {
                coefficients[i + j] = coefficients[i + j] + &(*coeff1 * coeff2);
            }
        }

        Polynomials::new(coefficients).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use ark_ff::AdditiveGroup;
    use ark_bn254::Fq;

    use super::*;
    use crate::Polynomials;

    #[test]
    fn should_add_polynomials() {
        // 3x + 5
        let poly1 = Polynomials::new(
            vec![Fq::from(5), Fq::from(3)]
        ).unwrap();
        // 2x + 4
        let poly2 = Polynomials::new(
            vec![Fq::from(4), Fq::from(2)]
        ).unwrap();
        // 5x + 9
        let result = &poly1 + &poly2;
        assert_eq!(result.coefficients()[0], Fq::from(9));
        assert_eq!(result.coefficients()[1], Fq::from(5));


        // 3x^5 + 5x^3 - 7x - 5
        let poly1 = Polynomials::new(
            vec![Fq::from(-5), Fq::from(-7), Zero::zero(), Fq::from(5), Zero::zero(), Fq::from(3)],
        ).unwrap();
        // x^3 + 9x^2 - 9
        let poly2 = Polynomials::new(
            vec![Fq::from(-9), Zero::zero(), Fq::from(9), Fq::from(1)]
        ).unwrap();

        // 3x^5 + 6x^3 + 9x^2 -7x -14
        let result = &poly1 + &poly2;
        println!("Result: {:?}", result);
        assert_eq!(result.coefficients(), &vec![Fq::from(-14), Fq::from(-7), Fq::from(9), Fq::from(6), Zero::zero(), Fq::from(3)]);

    }

    #[test]
    fn should_multiply_polynomials() {
        // 3x + 2
        let poly1 = Polynomials::new(
            vec![Fq::from(2), Fq::from(3)]
        ).unwrap();

        // 2x + 4
        let poly2 = Polynomials::new(
            vec![Fq::from(4), Fq::from(2)]
        ).unwrap();

        let result = &poly1 * &poly2;
        // 8x^2 + 14x + 8
        assert_eq!(result.coefficients()[0], Fq::from(8));
        assert_eq!(result.coefficients()[1], Fq::from(16));   
        assert_eq!(result.coefficients()[2], Fq::from(6));

        // x^3 + 9x^2 - 9
        let poly1 = Polynomials::new(
            vec![Fq::from(-9), Zero::zero(), Fq::from(9), Fq::from(1)]
        ).unwrap();
        // 3x^5 + 5x^3 - 7x - 5
        let poly2 = Polynomials::new(
            vec![Fq::from(-5), Fq::from(-7), Fq::ZERO, Fq::from(5), Fq::ZERO, Fq::from(3)]
        ).unwrap();

        let result = &poly1 * &poly2;
        // 3x^8 + 27x^7 + 5x^6 + 18x^5 - 7x^4 - 133x^3 - 45x^2 + 83x + 45
        assert_eq!(result.coefficients()[0], Fq::from(45));
        assert_eq!(result.coefficients()[1], Fq::from(63));
        assert_eq!(result.coefficients()[2], Fq::from(-45));
        assert_eq!(result.coefficients()[3], Fq::from(-113));
        assert_eq!(result.degree(), 8);
    }

    #[test]
    fn should_perform_scalar_mul() {
        // 3x + 2
        let poly1 = Polynomials::new(
            vec![Fq::from(2), Fq::from(3)]
        ).unwrap();

        assert_eq!(poly1.scalar_mul(Fq::from(4)).coefficients(), &vec![Fq::from(8), Fq::from(12)]);
        assert_eq!(poly1.scalar_mul(Zero::zero()).coefficients(), vec![Fq::zero(), Fq::ZERO].as_slice());
        assert_eq!(poly1.scalar_mul(Fq::from(-1)).coefficients(), vec![Fq::from(-2), Fq::from(-3)].as_slice());

        // 8x^2 + 14x + 8
        let poly2 = Polynomials::new(
            vec![Fq::from(8), Fq::from(14), Fq::from(8)]
        ).unwrap();
        assert_eq!(poly2.scalar_mul(Fq::from(10)).coefficients(), &vec![Fq::from(80), Fq::from(140), Fq::from(80)]);
        assert_eq!(poly2.scalar_mul(Fq::from(4)).coefficients(), &vec![Fq::from(32), Fq::from(56), Fq::from(32)]);
    }
}