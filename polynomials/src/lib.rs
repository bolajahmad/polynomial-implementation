use std::vec;

use ark_ff::{One, PrimeField, Zero};
use ark_bn254::Fq;
use types::PolynomialTrait;

pub mod types;
mod arithmetics;

#[derive(Debug)]
pub enum PolynomialError {
    DegreeError,
}

//How would we implement a Polynomial type in Rust?
// define an struct tuple (List of coefficients, degree)
// Smallest coeffifient first 
// (ax^n + bx^(n-1) + ... cx + d) (d, c, ... , b, a)
#[derive(Default, Debug, Clone)]
pub struct Polynomials<F: PrimeField>(Vec<F>);

impl<F: PrimeField> Polynomials<F> {
    pub fn new(coefficients: Vec<F>) -> Result<Self, PolynomialError> {
        // The length of the coefficients must be greater than 1
        if coefficients.len() <= 0 {
            return Err(PolynomialError::DegreeError);
        }

        Ok(Self(coefficients))
    }

    // Returns the list of coefficients only
    pub fn coefficients(&self) -> &Vec<F> {
        &self.0
    }

    // Returns a more optimal value of the degree by elimination
    fn degree(&self) -> u8 {
        // Returns the first degree where the coefficient is not 0
        let mut degree = (self.coefficients().len() - 1) as u8;

        // For each coefficient, reduce degree till encounter non-zero
        for i in 0..degree {
            if self.coefficients()[i as usize] != Zero::zero() {
                break;
            } else {
                degree -= 1;
            }
        }

        return degree
    }

    pub fn scalar_mul(&self, scalar: F) -> Self {
        Polynomials::new(self.coefficients().iter().map(|&x| (x * scalar)).collect()).unwrap()
    }

    fn from_points(points: Vec<F>, x: F) -> (Self, F) {
        let numerator = points
            .iter()
            .filter(|&xi| x != *xi)
            .map(|&xi| Polynomials::new(vec![-xi, One::one()])
            .unwrap()).collect::<Vec<Polynomials<F>>>();
        let denominator = points
            .into_iter()
            .filter(|&xi| x != xi)
            .map(|xi| x - xi)
            .reduce(|acc, curr| acc * curr)
            .unwrap();

        let mut polynomial: Polynomials<F> = Polynomials::new(vec![One::one()]).unwrap();
        for poly in numerator.iter() {
            polynomial = &polynomial * poly;
        }
        // Multiply all the numerators
        (polynomial, denominator)
    }
}

impl<F: PrimeField> PolynomialTrait<F> for Polynomials<F> {
    fn evaluate(&self, x: F) -> F {
        let mut result = Zero::zero();
        let mut power = F::one();

        // Do a to_vec to pass ownership
        let mut coefficients = self.coefficients().to_vec();
        // reverse for more optimized manipulation
        // coefficients.reverse();
        // let mut i = self.degree() + 1;
        
        for (i, coeff) in coefficients.iter().enumerate() {
            // let pow = if i > 0 {
            //     x.pow(&[i as u64])
            // } else {
            //     One::one()
            // };

            // println!("{}^{} = {}", x, i, pow);

            println!("Power of x, {:?}; coefficient {}", power, coeff);
            result += *coeff * power;
            power = power * x;
        }

        result
    }

    // Should take an array of points of variable length
    // Point = (x: u32, y: u32)
    // The resulting polynomial cpefficient are rounded to the nearest integer
    fn interpolate(points: Vec<(F, F)>) -> Self {
        let mut interpolated_poly = Polynomials::new(vec![Zero::zero()]).unwrap();

        for (x, y) in points.iter() {
            let (numerator, denominator) = Polynomials::from_points( 
                points.iter().map(|(x, _)| *x).collect(), *x);
                
            interpolated_poly = &interpolated_poly + &numerator.scalar_mul(*y / denominator);
        }

        // Placeholder for interpolation logic
        interpolated_poly
    }
}


#[cfg(test)]
mod tests {
    use ark_ff::{AdditiveGroup, Field};

    use super::*;

    #[test]
    fn should_initialize_polynomial() {
        // 3x + 5
        let poly_result = Polynomials::new(
            vec![Fq::from(5), Fq::from(3)]
        );
        assert!(poly_result.is_ok());

        let poly = poly_result.unwrap();
        assert_eq!(poly.coefficients()[1], Fq::from(3));
        assert_eq!(poly.coefficients()[0], Fq::from(5));

        // 3x^5 + 5x^3 - 7x - 5
        let poly_result = Polynomials::new(
            vec![Fq::from(-5), Fq::from(-7), Zero::zero(), Fq::from(5), Zero::zero(), Fq::from(3)]
        );
        assert!(poly_result.is_ok());
        let poly = poly_result.unwrap();
        assert_eq!(poly.coefficients()[5], Fq::from(3));
        assert_eq!(poly.coefficients()[4], Fq::zero());
        assert_eq!(poly.coefficients()[1], Fq::from(-7));
    }

    #[test]
    fn should_have_correct_degree() {
        // 3x + 5
        let poly = Polynomials::new(
            vec![Fq::from(5), Fq::from(3)]
        ).unwrap();
        assert_eq!(poly.degree(), 1);

        // 3x^5 + 5x^3 - 7x - 5
        let poly_result = Polynomials::new(
            vec![Fq::from(-5), Fq::from(-7), Fq::ZERO, Fq::from(5), Fq::zero(), Fq::from(3)]
        );
        let poly = poly_result.unwrap();
        assert_eq!(poly.degree(), 5);
    }

    #[test]
    fn should_evaluate_polynomial() {
        // 3x + 5
        let poly = Polynomials::new(
            vec![Fq::from(5), Fq::from(3)]
        ).unwrap();
        // assert_eq!(poly.true_degree(), 1);
        assert_eq!(poly.evaluate(Fq::from(2)), Fq::from(11));
        assert_eq!(poly.evaluate(Fq::ZERO), Fq::from(5));
        assert_eq!(poly.evaluate(Fq::from(-1)), Fq::from(2));

        // // 3x^5 + 5x^3 - 7x - 5
        let poly = Polynomials::new(
            vec![Fq::from(-5), Fq::from(-7), Fq::ZERO, Fq::from(5), Fq::zero(), Fq::from(3)]
        ).unwrap();
        assert_eq!(poly.evaluate(Fq::from(2)), Fq::from(117));
        assert_eq!(poly.evaluate(Fq::from(10)), Fq::from(304925));
    }

    #[test]
    fn should_interpolate_points() {
        let points = vec![
            (Fq::ONE, Fq::from(17)), 
            (Fq::from(2), Fq::from(44)), 
            (Fq::from(4), Fq::from(182)), 
            (Fq::from(5), Fq::from(305))];
        let poly = Polynomials::interpolate(points);
        assert_eq!(poly.coefficients(), &vec![Fq::from(10), Fq::from(-1), Fq::from(7), Fq::one()]);
    }
}