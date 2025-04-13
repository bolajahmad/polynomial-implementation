// Given a Multilinear polynomial with n variables.
// There exists 2^n possible combinations of each variable.
// This fact can be used to represent a multilinear polynomial
// The important properties of a Multilinear polynomial are

// - Number of the variables
// - Coefficients of each combination of the variables
// The max degree is equal to the number of variables

use ark_bn254::Fq;
use ark_ff::{BigInteger, PrimeField};
use num_bigint::BigUint;

pub mod mocks;
pub mod helper;
pub mod arithmetics;

#[derive(Default, Debug)]
pub struct MultiLinearPolynomial<F: PrimeField> {
    // Number of variables is collected to avoid extra computation.
    variables: usize,
    // variable_combination is a binary representation, 
    // Vec of tuple (variable_combination, coefficient)
    coefficients: Vec<(usize, F)>       
}

impl<F: PrimeField> MultiLinearPolynomial<F> {
    pub fn new(variables: usize, coefficients: Vec<(usize, F)>) -> Self {
        let exponent = BigUint::from(
            variables as u64
        );
        let combinations = BigUint::from(2u128).modpow(&exponent, &BigUint::from_bytes_le(&F::MODULUS.to_bytes_le()));
        
        if F::from(coefficients.len() as u128) > F::from(combinations) {
            panic!("The coefficients.len() must not be gt the variables");
        }

        let mut mlp = MultiLinearPolynomial { 
            variables,
            coefficients: coefficients
        };
        mlp.ensure_sorted();
        mlp.ensure_no_zero_coefficients();

        // Assumes the coefficients are sorted 0..variables
        mlp
    }

    pub fn coefficients(&self) -> &Vec<(usize, F)> {
        &self.coefficients
    }

    pub fn combinations(&self) -> Vec<u64> {
        let exponent = BigUint::from(self.variables);
        let combinations = BigUint::from(2u128)
            .modpow(
                &exponent, 
                &BigUint::from_bytes_le
                (&F::MODULUS.to_bytes_le())
            ) - BigUint::from(1_u32);

        combinations.to_u64_digits()
    }

    pub fn degree(&self) -> F {
        let length = self.coefficients().len();
        // Convert the length of coefficients to binary representation true => 1, false => 0
        let byte_repr = F::from(self.coefficients()[length - 1].0 as u64)
            .into_bigint()
            .to_bits_le();

        // filter out all the falsy values
        let ones_count = byte_repr.iter().filter(|is_true| **is_true)
            .collect::<Vec<&bool>>();

        // The degree is the length of the true values
        return F::from(ones_count.len() as u128);
    }

    pub fn scalar_mul(&self, scalar: F) -> Self {
        let new_coefficients = self
            .coefficients()
            .iter()
            .map(|&(combination, coefficient)| (combination, coefficient * scalar))
            .collect();

        Self::new(self.variables, new_coefficients)
    }

    pub fn ensure_sorted(&mut self) -> bool {
        let mut sorted = self.coefficients.clone();
        sorted.sort_by(|a, b| a.0.cmp(&b.0));

        self.coefficients = sorted.to_vec();

        sorted != self.coefficients
    }

    pub fn ensure_no_zero_coefficients(&mut self) -> bool {
        let mut new_coefficients = self.coefficients.clone();
        new_coefficients.retain(|&(_, coefficient)| coefficient != F::zero());

        if new_coefficients.len() != self.coefficients.len() {
            self.coefficients = new_coefficients;
            return true;
        }
        false
    }

    // fn remove_zero_coefficients(&mut self) {
    //     self.coefficients.retain(|&(_, coefficient)| coefficient != F::zero());
    // }
}

impl<F: PrimeField> MultiLinearPolynomial<F> {
    pub fn evaluate(&mut self, set: Vec<F>) -> F {
        assert_eq!(
            F::from(self.variables as u64), 
            F::from(set.len() as u128), 
            "Invalid number of variables"
        );

        for (i, value) in set.iter().enumerate() {
            // Do partial evaluation of the polynomial
            let _ = self.partial_evaluate(i, *value);
        }

        self.coefficients()[0].1
    }

    pub fn partial_evaluate(&mut self, index: usize, value: F) -> Result<(), ()> {
        let mut new_coefficients = self.coefficients.clone();
        let variable_count = self.variables;

        for (_, (variables, coefficient)) in self.coefficients.iter().enumerate() {
            // Check index of each variable
            // If index is 1 multiply coefficient by value and,
            // Switch the index to 0
            let variable_bits = format!(
                "{:0width$b}", 
                variables, 
                width = variable_count
            );
            let is_1_bit = match variable_bits.chars().nth(index) {
                Some(bit) => bit == '1',
                None => false,
            };
            
            if is_1_bit {
                // clear the i-th bit
                // let variable = clear_ith_bit(*variables as u64, index as u64) as usize;
                let variable = variables & !(1 << (variable_count - 1 - index));
                println!("Variable: {}", variable);
                // multiply coefficient by value
                let coefficient = *coefficient * value;

                // Check if the variable is already in the coefficients
                let has_variable = new_coefficients.iter().any(|(var, _)| *var == variable);
                if !has_variable {
                    // Add the variable to the coefficients
                    new_coefficients.push((variable, coefficient));
                    new_coefficients = new_coefficients.iter().filter(|(var, _)| var != variables).cloned().collect::<Vec<(usize, F)>>();
                } else {
                    // Update the coefficient of the existing variable
                     new_coefficients = new_coefficients.iter().map(|(var, coeff)| if var == &variable {
                        return (*var, coefficient + coeff)
                    } else if var == variables {
                        return (*var, coefficient * F::ZERO)
                    } else {
                        return (*var, *coeff)
                    }).collect::<Vec<(usize, F)>>();
                }

            }
        }

        *self = MultiLinearPolynomial::new(self.variables, new_coefficients);
        Ok(())
    }

    /**
     * Takes a Vec of points (x, y) where x is the variable combination
     * Should return a multilinear polynomial
     * We need the evaluation of each combination of the polynomial
     * @params points: Vec<(usize, F)>, F here is the evaluation of the points
     * @params variables: usize, the total number of variables
     * @return: MultiLinearPolynomial<F>
     * @example: points = [(0, 1), (1, 2), (2, 3)]
     */
    pub fn interpolate(points: Vec<usize>, variables: usize) -> MultiLinearPolynomial<F> {
        assert_eq!((points.len() as f64).log2(), variables as f64);
        let mut coefficients = MultiLinearPolynomial::new(variables, vec![]);
        
        // // Iterate through the points and create a new polynomial
        for (index, coefficient) in points.iter().enumerate() {
            // y . if variable[i] == (1 - a) (check_1) else (a)
            // Get the variable combination
            let variable_bits = format!("{:0width$b}", index, width = variables);
            let mut variable_product = MultiLinearPolynomial::new(variables, vec![(0, F::ONE)]);
            for (bit, data) in variable_bits.chars().enumerate() {
                if data == '1' {
                    // check_1
                    variable_product = &variable_product * &MultiLinearPolynomial::new(variables, [(0, F::ZERO), (2usize.pow(bit as u32), F::from(1))].to_vec());
                } else {
                    // check_0
                    variable_product = &variable_product * &MultiLinearPolynomial::new(variables, [(0, F::ONE), (2usize.pow(bit as u32), F::from(-1))].to_vec());
                }
            }

            coefficients = &coefficients + &variable_product.scalar_mul(F::from(*coefficient as u64));
        }
        
        coefficients
    }
}

#[cfg(test)]
mod tests {
    use crate::{mocks::{multilinear_polya, multilinear_polyb}, MultiLinearPolynomial};
    // use super::*;

    use ark_bn254::Fq;
    use ark_ff::{Field, PrimeField};

    #[test]
    fn should_initialize_multilinear_polynomial() {
        // 3x + 5
        let poly_a = multilinear_polya::<Fq>();
        assert_eq!(poly_a.degree(), Fq::from(3));
        assert_eq!(poly_a.coefficients().len(), 4);

        let poly_b = multilinear_polyb::<Fq>();
        assert_eq!(poly_b.degree(), Fq::from(4));
        assert_eq!(poly_b.coefficients().len(), 5);
    }

    #[test]
    fn should_perform_scalar_mul() {
        // 2abc + 2ab + 3bc + 4
        let poly_a = multilinear_polya::<Fq>();
        let scalar = Fq::from(2);
        let poly_a_scalar_mul = poly_a.scalar_mul(scalar);

        for i in 0..poly_a.coefficients().len() {
            assert_eq!(poly_a_scalar_mul.coefficients()[i].1, poly_a.coefficients()[i].1 * scalar);
        }

        assert_eq!(poly_a_scalar_mul.coefficients()[2].0, 6);
    }

    #[test]
    fn should_evaluate_polynomial() {
    // f(a,b,c,d,e,f) = 2bcdf + 2abcf + 3bcd + 4abc + 9
    // 4bdf + 4abf + 6bd + 8ab + 9
    let coefficients = vec![
        (0, Fq::from(9u128)),
        (28, Fq::from(3u128)),
        (29, Fq::from(2u128)),
        (56, Fq::from(4u128)),
        (57, Fq::from(2u128)),
    ];
    let mut poly = MultiLinearPolynomial::new(6, coefficients);
    let _ = poly.partial_evaluate(2, Fq::from(2));

    assert_eq!(poly.coefficients()[4].1, Fq::from(4));
    assert_eq!(poly.coefficients()[2].0, 21);
    }
    
    #[test]
    fn should_interpolate_polynomial() {
         // f(a,b) = 2a + 3b - 5ab + 6
         let polynomial: MultiLinearPolynomial<Fq> = MultiLinearPolynomial::interpolate(
            vec![6, 9, 8, 6], 
            2
        );
        assert_eq!(polynomial.coefficients().len(), 4);
        assert_eq!(polynomial.coefficients()[3].1, Fq::from(-5));
        
        // let poly = f(a,b,c) = 3ab + 12abc - 4bc - c + 15
        let polynomial: MultiLinearPolynomial<Fq> = MultiLinearPolynomial::interpolate(
            vec![15, 14, 15, 10, 15, 14, 18, 25], 3
        );
        assert_eq!(polynomial.coefficients().len(), 5);
        assert_eq!(polynomial.coefficients()[4].1, Fq::from(12));
        assert_eq!(polynomial.coefficients()[3].1, Fq::from(-4));
    }
}