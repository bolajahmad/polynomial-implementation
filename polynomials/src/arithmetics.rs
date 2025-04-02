use std::{iter::Sum, ops::{Add, Mul}};

use crate::Polynomials;

impl Add for &Polynomials {
    type Output = Polynomials;

    fn add(self, other: &Polynomials) -> Polynomials {
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
            coefficients.push(value + higher_degree[index])
        }
        // copy the rest of the coefficients in higher_degree polynomial
        coefficients.extend_from_slice(&higher_degree[lower_degree.len()..]);

        // Create a new polynomial with the coefficients and the degree of the higher degree polynomial
        Polynomials::new(coefficients, higher_degree.len() as u8 - 1).unwrap()
    }
}

impl Mul for &Polynomials {
    type Output = Polynomials;

    fn mul(self, other: &Polynomials) -> Polynomials {
        let output_degree = self.coefficients().len() + other.coefficients().len() - 2;

        let mut coefficients = vec![0.0; output_degree + 1];
        for (i, coeff1) in self.coefficients().iter().enumerate() {
            for (j, coeff2) in other.coefficients().iter().enumerate() {
                coefficients[i + j] += coeff1 * coeff2;
            }
        }

        Polynomials::new(coefficients, output_degree as u8).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::Polynomials;

    #[test]
    fn should_add_polynomials() {
        // 3x + 5
        let poly1 = Polynomials::new(
            vec![5.0, 3.0],
            1
        ).unwrap();
        // 2x + 4
        let poly2 = Polynomials::new(
            vec![4.0, 2.0],
            1
        ).unwrap();
        // 5x + 9
        let result = &poly1 + &poly2;
        assert_eq!(result.coefficients()[0], 9.0);
        assert_eq!(result.coefficients()[1], 5.0);


        // 3x^5 + 5x^3 - 7x - 5
        let poly1 = Polynomials::new(
            vec![-5.0, -7.0, 0.0, 5.0, 0.0, 3.0],
            5
        ).unwrap();
        // x^3 + 9x^2 - 9
        let poly2 = Polynomials::new(
            vec![-9.0, 0.0, 9.0, 1.0],
            6
        ).unwrap();

        // 3x^5 + 6x^3 + 9x^2 -7x -14
        let result = &poly1 + &poly2;
        println!("Result: {:?}", result);
        assert_eq!(result.coefficients(), &vec![-14.0, -7.0, 9.0, 6.0, 0.0, 3.0]);

    }

    #[test]
    fn should_multiply_polynomials() {
        // 3x + 2
        let poly1 = Polynomials::new(
            vec![2.0, 3.0],
            1
        ).unwrap();

        // 2x + 4
        let poly2 = Polynomials::new(
            vec![4.0, 2.0],
            1
        ).unwrap();

        let result = &poly1 * &poly2;
        // 8x^2 + 14x + 8
        assert_eq!(result.coefficients()[0], 8.0);
        assert_eq!(result.coefficients()[1], 16.0);   
        assert_eq!(result.coefficients()[2], 6.0);

        // x^3 + 9x^2 - 9
        let poly1 = Polynomials::new(
            vec![-9.0, 0.0, 9.0, 1.0],
            3
        ).unwrap();
        // 3x^5 + 5x^3 - 7x - 5
        let poly2 = Polynomials::new(
            vec![-5.0, -7.0, 0.0, 5.0, 0.0, 3.0],
            5
        ).unwrap();

        let result = &poly1 * &poly2;
        // 3x^8 + 27x^7 + 5x^6 + 18x^5 - 7x^4 - 133x^3 - 45x^2 + 83x + 45
        assert_eq!(result.coefficients()[0], 45.0);
        assert_eq!(result.coefficients()[1], 63.0);
        assert_eq!(result.coefficients()[2], -45.0);
        assert_eq!(result.coefficients()[3], -113.0);
        assert_eq!(result.true_degree(), 8);
    }

    #[test]
    fn should_perform_scalar_mul() {
        // 3x + 2
        let poly1 = Polynomials::new(
            vec![2.0, 3.0],
            1
        ).unwrap();

        assert_eq!(poly1.scalar_mul(4.0).coefficients(), &vec![8.0, 12.0]);
        assert_eq!(poly1.scalar_mul(0.0).coefficients(), vec![0.0, 0.0].as_slice());
        assert_eq!(poly1.scalar_mul(-1.0).coefficients(), vec![-2.0, -3.0].as_slice());

        // 8x^2 + 14x + 8
        let poly2 = Polynomials::new(
            vec![8.0, 14.0, 8.0],
            2
        ).unwrap();
        assert_eq!(poly2.scalar_mul(10.0).coefficients(), &vec![80.0, 140.0, 80.0]);
        assert_eq!(poly2.scalar_mul(4.0).coefficients(), &vec![32.0, 56.0, 32.0]);
    }
}