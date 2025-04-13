// Given a Multilinear polynomial with n variables.
// There exists 2^n possible combinations of each variable.
// This fact can be used to represent a multilinear polynomial
// The important properties of a Multilinear polynomial are

// - Number of the variables
// - Coefficients of each combination of the variables
// The max degree is equal to the number of variables

use ark_ff::{BigInteger, PrimeField};
use num_bigint::{BigUint, ToBigUint};

pub mod mocks;
pub mod helper;
pub mod arithmetics;

#[derive(Default, Debug)]
pub struct MultiLinearPolynomial<F: PrimeField> {
    variables: F,
    // variable_combination is a binary representation, 
    coefficients: Vec<(usize, F)>       // Vec of tuple (variable_combination, coefficient)
}

// 2ab + 3bc
// ()

impl<F: PrimeField> MultiLinearPolynomial<F> {
    pub fn new(variables: F, coefficients: Vec<(usize, F)>) -> Self {
        let exponent = BigUint::from_bytes_le(&variables.into_bigint().to_bytes_le());
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

    pub fn combinations(&self) -> F {
        let exponent = BigUint::from_bytes_le(&self.variables.into_bigint().to_bytes_le());
        let combinations = BigUint::from(2u128).modpow(&exponent, &BigUint::from_bytes_le(&F::MODULUS.to_bytes_le()));

        F::from(combinations - 1.to_biguint().unwrap())
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

    fn fill_with_zero(&self) -> Self {
        let mut new_coefficients = vec![];

        for i in 0..self.combinations().into_bigint().to_bytes_le()[0] as usize {
            if !self.coefficients.iter().any(|&(combination, _)| combination == i) {
                new_coefficients.push((i, F::zero()));
            }
        }

        Self::new(
            self.variables,
            new_coefficients
        )
    }

    fn remove_zero_coefficients(&mut self) {
        self.coefficients.retain(|&(_, coefficient)| coefficient != F::zero());
    }
}

impl<F: PrimeField> MultiLinearPolynomial<F> {
    pub fn evaluate(&mut self, set: Vec<F>) -> F {
        assert_eq!(self.variables, F::from(set.len() as u128), "Invalid number of variables");

        for (i, value) in set.iter().enumerate() {
            // Do partial evaluation of the polynomial
            let _ = self.partial_evaluate(i, *value);
        }

        self.coefficients()[0].1
    }

    pub fn partial_evaluate(&mut self, index: usize, value: F) -> Result<(), ()> {
        let mut new_coefficients = self.coefficients.clone();
        let variable_count = self.variables.into_bigint().to_bytes_le();

        for (_, (variables, coefficient)) in self.coefficients.iter().enumerate() {
            // Check index of each variable
            // If index is 1 multiply coefficient by value and,
            // Switch the index to 0
            let variable_bits = format!("{:0width$b}", variables, width = variable_count[0] as usize);
            let is_1_bit = match variable_bits.chars().nth(index) {
                Some(bit) => bit == '1',
                None => false,
            };
            
            if is_1_bit {
                // clear the i-th bit
                // let variable = clear_ith_bit(*variables as u64, index as u64) as usize;
                let variable = variables & !(1 << (variable_count[0] as usize - 1 - index));
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
     * @params points: Vec<(usize, F)>, F here is the evaluation of the points
     * @params variables: usize, the total number of variables
     * @return: MultiLinearPolynomial<F>
     * @example: points = [(0, 1), (1, 2), (2, 3)]
     */
    pub fn interpolate(points: Vec<(usize, F)>, variables: F) -> MultiLinearPolynomial<F> {
        // let mut coefficients = vec![];
        
        // // Iterate through the points and create a new polynomial
        // for (variable, yvalue) in points.iter() {
        //     // y . if variable[i] == 1 (check_1) else (check_0)
        //     // Get the variable combination
        //     let variable_bits = format!("{:0width$b}", variable, width = variables);
        //     let mut variable_product = MultiLinearPolynomial::default();
            
        //     for data in variable_bits.chars() {
        //         if data == '1' {
        //             // check_1
        //             variable_product *= 1;
        //         } else {
        //             // check_0
        //             variable_product *= 0;
        //         }
        //     }

        //     // Add the coefficient to the coefficients vector
        //     coefficients.push((variable_combination, coefficient));
        // }

        // MultiLinearPolynomial::new(F::from(points.len() as u128), coefficients)
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use crate::mocks::{multilinear_polya, multilinear_polyb};
    use super::*;

    use ark_bn254::Fq;

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
        // 2abc + 2ab + 3bc + 4
        let mut poly_a = multilinear_polyb::<Fq>();

        // define the evaluating set on each variable
        let set = vec![
            Fq::from(2),
            Fq::from(3),
            Fq::from(4),
            Fq::from(2),
            Fq::from(3),
            Fq::from(4),
        ];
        let result = poly_a.partial_evaluate(2, Fq::from(2));

        // assert_eq!(result, Fq::from(0));
    }
}