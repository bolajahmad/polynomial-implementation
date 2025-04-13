use ark_ff::PrimeField;
use ark_bn254::Fq;
use polynomials::univariate::{types::PolynomialTrait, Polynomials};
use rand::Rng;

pub struct ShamirSecret {
    total_shares: u64,
    threshold: u8,
}

impl ShamirSecret {
    pub fn new(total_shares: u64, threshold: u8) -> Self {
        ShamirSecret {
            total_shares,
            threshold,
        }
    }

    pub fn generate_shares<F: PrimeField>(&self, secret_key: F) -> Vec<(F, F)> {
        // let's split the secret to 3:2 and place at coefficient 0 & 1 respectively
        let ratio3 = F::from(secret_key) / F::from(3);
        let mut coefficients = vec![ratio3, F::from(secret_key) - ratio3];

        for _ in 0..(self.threshold-1) {
            // generate a random number and push to the polynomial array
            let coeff = F::from(rand::rng().random::<u64>());
            coefficients.push(coeff);
        }
        println!("Polynomial is {:?}", coefficients);
        let poly = Polynomials::new(coefficients).unwrap();

        let mut output_shares = vec![];
        // generate sets of points for the secret sharing
        for _ in 0..self.total_shares {
            // evaluate the polynomial at random points and push
            let random_x = F::from(rand::rng().random::<u64>());
            let random_y = poly.evaluate(random_x);
            output_shares.push((random_x, random_y));
        }

        output_shares
    } 

    pub fn verify_secret<F: PrimeField>(&self, shares: Vec<(F, F)>, secret: F) -> bool {
        // length of shares must be at least self.threshold in length
        if shares.len() < self.threshold as usize {
            println!("Not enough shares to verify");
            return false;
        }

        // interpolate the shares to find the result
        let polynomial = Polynomials::interpolate(shares);

        // Index 0 + 1 must equal secret
        let derived_secret = polynomial.coefficients()[0] + polynomial.coefficients()[1];
        derived_secret == secret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shamir_secret() {
        let shamir = ShamirSecret::new(5, 3);
        let secret_key = Fq::from(42);
        let shares = shamir.generate_shares(secret_key);

        assert_eq!(shares.len(), 5);

        let is_valid = shamir.verify_secret(shares.clone(), secret_key);
        assert!(is_valid, "The secret should be valid");
    }
}