use std::vec;

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
pub struct Polynomials(Vec<f64>, u8);

impl Polynomials {
    pub fn new(coefficients: Vec<f64>, order: u8) -> Result<Self, PolynomialError> {
        // The degree must not be less than the length of the coefficients
        if coefficients.len() as u8 > order + 1 {
            return Err(PolynomialError::DegreeError);
        }

        if coefficients.len() == 0 {
            return Err(PolynomialError::DegreeError);
        }

        Ok(Self(coefficients, order))
    }

    // Returns the list of coefficients only
    fn coefficients(&self) -> &Vec<f64> {
        &self.0
    }

    // Returns the degree of the polynomial
    fn degree(&self) -> u8 {
        self.1
    }

    // Returns a more optimal value of the degree by elimination
    fn true_degree(&self) -> u8 {
        // Returns the first degree where the coefficient is not 0
        let mut degree = if self.coefficients().len() <  self.degree() as usize {
            self.coefficients().len() as u8
        } else {
            self.degree() + 1
        };

        // For each coefficient, reduce degree till encounter non-zero
        for i in 0..degree {
            if self.coefficients()[i as usize] != 0.0 {
                break;
            } else {
                degree -= 1;
            }
        }

        return degree - 1
    }

    pub fn scalar_mul(&self, scalar: f64) -> Self {
        Polynomials::new(self.coefficients().iter().map(|&x| (x * scalar).round()).collect(), self.degree()).unwrap()
    }

    fn from_points(points: Vec<f64>, x: f64) -> (Self, f64) {
        let numerator = points
            .iter()
            .filter(|&xi| x != *xi)
            .map(|&xi| Polynomials::new(vec![-xi, 1.0], 1)
            .unwrap()).collect::<Vec<Polynomials>>();
        let denominator = points
            .into_iter()
            .filter(|&xi| x != xi)
            .map(|xi| x - xi)
            .reduce(|acc, curr| acc * curr)
            .unwrap();

        let mut polynomial = Polynomials::new(vec![1.0], 1).unwrap();
        for poly in numerator.iter() {
            polynomial = &polynomial * poly;
        }
        // Multiply all the numerators
        (polynomial, denominator)
    }
}

impl PolynomialTrait for Polynomials {
    fn evaluate(&self, x: f64) -> f64 {
        let mut result = 0.0;

        // Do a to_vec to pass ownership
        let mut coefficients = self.coefficients().to_vec();
        // reverse for more optimized manipulation
        coefficients.reverse();
        // Use the true_degree to avoid unnecessary computation
        let mut i = self.true_degree() as i16;
        
        for coeff in coefficients.iter() {
            if i < 0 {
                break;
            }

            let pow = if i > 0 {
                x.powf(i as f64)
            } else {
                1.0
            };

            result += coeff * pow;
            i -= 1;
        }

        result
    }

    // Should take an array of points of variable length
    // Point = (x: u32, y: u32)
    // The resulting polynomial cpefficient are rounded to the nearest integer
    fn interpolate(points: Vec<(f64, f64)>) -> Self {
        let mut interpolated_poly = Polynomials::new(vec![0.0], 0).unwrap();

        for (x, y) in points.iter() {
            let (numerator, denominator) = Polynomials::from_points( 
                points.iter().map(|(x, _)| *x).collect(), *x);
                
                println!("Numerator: {:?}", numerator);
                println!("Denominator: {:?}", denominator);
                println!("Y/denominator: {:?}", y / denominator);
            
            interpolated_poly = &interpolated_poly + &numerator.scalar_mul(y / denominator);
            println!("Interpolated polynomial: {:?}", interpolated_poly);
        }

        // Placeholder for interpolation logic
        interpolated_poly
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_initialize_polynomial() {
        // 3x + 5
        let poly_result = Polynomials::new(
            vec![5.0, 3.0],
            1
        );
        assert!(poly_result.is_ok());

        let poly = poly_result.unwrap();
        assert_eq!(poly.coefficients()[1], 3.0);
        assert_eq!(poly.coefficients()[0], 5.0);

        // 3x^5 + 5x^3 - 7x - 5
        let poly_result = Polynomials::new(
            vec![-5.0, -7.0, 0.0, 5.0, 0.0, 3.0],
            5
        );
        assert!(poly_result.is_ok());
        let poly = poly_result.unwrap();
        assert_eq!(poly.coefficients()[5], 3.0);
        assert_eq!(poly.coefficients()[4], 0.0);
        assert_eq!(poly.coefficients()[1], -7.0);
    }

    #[test]
    fn should_have_correct_degree() {
        // 3x + 5
        let poly = Polynomials::new(
            vec![5.0, 3.0],
            1
        ).unwrap();
        assert_eq!(poly.degree(), 1);

        // 3x^5 + 5x^3 - 7x - 5
        let poly_result = Polynomials::new(
            vec![0.0, 5.0, 3.0],
            5
        );
        let poly = poly_result.unwrap();
        assert_eq!(poly.degree(), 5);
    }

    #[test]
    fn should_have_true_degree() {
        // 3x + 5
        let poly = Polynomials::new(
            vec![5.0, 3.0],
            1
        ).unwrap();
        assert_eq!(poly.true_degree(), 1);

        // 3x^5 + 5x^3 - 7x - 5
        let poly = Polynomials::new(
            vec![0.0, 5.0, 3.0],
            5
        ).unwrap();
        assert_eq!(poly.true_degree(), 1);

        // 3x^7 + 5x^3 - 7x - 5
        let poly = Polynomials::new(
            vec![-5.0, -7.0, 0.0, 5.0, 0.0, 0.0, 0.0, 3.0],
            7
        ).unwrap();
        assert_eq!(poly.true_degree(), 7);
    }

    #[test]
    fn should_evaluate_polynomial() {
        // 3x + 5
        let poly = Polynomials::new(
            vec![5.0, 3.0],
            1
        ).unwrap();
        // assert_eq!(poly.true_degree(), 1);
        assert_eq!(poly.evaluate(2.0), 11.0);
        assert_eq!(poly.evaluate(0.0), 5.0);
        assert_eq!(poly.evaluate(-1.0), 2.0);

        // // 3x^5 + 5x^3 - 7x - 5
        let poly = Polynomials::new(
            vec![-5.0, -7.0, 0.0, 5.0, 0.0, 3.0],
            5
        ).unwrap();
        assert_eq!(poly.evaluate(2.0), 117.0);
        assert_eq!(poly.evaluate(10.0), 304925.0);
    }

    #[test]
    fn should_interpolate_points() {
        let points = vec![
            (1.0, 17.0), 
            (2.0, 44.0), 
            (4., 182.0), 
            (5., 305.0)];
        let poly = Polynomials::interpolate(points);
        // assert_eq!(poly.coefficients(), &vec![10.0, -1.0, 7.0, 1.0]);
        assert_eq!(poly.coefficients()[2], 7.0);
    }
}