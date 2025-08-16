use ark_bn254::Fq;
use ark_ff::{AdditiveGroup, Field};
use polynomials::multilinear::{helper::check_is_1_bit, MultiLinearPolynomial};

pub struct Sumcheck;

impl Sumcheck {
    pub fn verify(
        mut polynomial: MultiLinearPolynomial<Fq>,
        _round: usize,
        evaluation: Fq,
    ) -> bool {
        let mut poly = polynomial.clone();
        poly.partial_evaluate(0, Fq::ZERO).unwrap();
        polynomial.partial_evaluate(0, Fq::ONE).unwrap();

        let zero_sum = poly.coefficients()[0].1;
        let one_sum = polynomial.coefficients()[0].1;
        return evaluation == (&zero_sum + &one_sum);
    }
}

pub struct Prover {
    pub eqn: MultiLinearPolynomial<Fq>,
    pub round: usize,
    pub previous_polynomial: Option<MultiLinearPolynomial<Fq>>,
    pub evaluated_polynomial: Option<MultiLinearPolynomial<Fq>>,
}

impl Prover {
    fn new(eqn: MultiLinearPolynomial<Fq>) -> Self {
        Prover {
            eqn,
            round: 0,
            previous_polynomial: None,
            evaluated_polynomial: None,
        }
    }

    fn prove(&mut self, index: usize, _value: Vec<Fq>) {
        // for the very first round
        // value and index are 0
        // get possible combinations of the polynomial
        let mut half_univariate = MultiLinearPolynomial::<Fq>::new(self.eqn.variables, vec![]);
        self.round += 1;

        if self.round == 1 {
            for combination in 0..=(self.eqn.combinations()[0] / 2) {
                let mut poly = self.eqn.clone();
                let variables = poly.variables;
                // don't evaluate the first variable
                for i in 1..variables {
                    let index = (variables - 1) - i;
                    poly.partial_evaluate(
                        i,
                        if check_is_1_bit(combination, index) {
                            Fq::ONE
                        } else {
                            Fq::ZERO
                        },
                    )
                    .unwrap();
                }
                half_univariate = &half_univariate + &poly;
            }

            let new_values = vec![
                half_univariate.coefficients()[0],
                if half_univariate.coefficients().len() == 2 {
                    (1, half_univariate.coefficients()[1].1)
                } else {
                    (1, Fq::ZERO)
                },
            ];
            self.evaluated_polynomial = Some(MultiLinearPolynomial::new(1, new_values));
        } else if self.round < self.eqn.variables {
            for combination in 0..=(self.eqn.combinations()[0]) {
                if check_is_1_bit(combination, index) {
                    let mut poly = self.eqn.clone();
                    let variables = poly.variables;

                    for i in 0..variables {
                        if i != index {
                            let index = (variables - 1) - i;
                            poly.partial_evaluate(
                                i,
                                if check_is_1_bit(combination, index) {
                                    Fq::ONE
                                } else {
                                    Fq::ZERO
                                },
                            )
                            .unwrap();
                        }
                    }
                    half_univariate = &half_univariate + &poly;
                }
            }
            let new_values = vec![
                half_univariate.coefficients()[0],
                if half_univariate.coefficients().len() == 2 {
                    (1, half_univariate.coefficients()[1].1)
                } else {
                    (1, Fq::ZERO)
                },
            ];

            self.previous_polynomial = self.evaluated_polynomial.clone();
            self.evaluated_polynomial = Some(MultiLinearPolynomial::new(1, new_values));
        } else {
            for combination in 0..=(self.eqn.combinations()[0]) {
                if check_is_1_bit(combination, index) {
                    let mut poly = self.eqn.clone();
                    let variables = poly.variables;

                    for i in 0..(variables - 1) {
                        let index = (variables - 1) - i;
                        poly.partial_evaluate(
                            i,
                            if check_is_1_bit(combination, index) {
                                Fq::ONE
                            } else {
                                Fq::ZERO
                            },
                        )
                        .unwrap();
                    }
                    half_univariate = &half_univariate + &poly;
                }
            }
            let new_values = vec![
                half_univariate.coefficients()[0],
                if half_univariate.coefficients().len() == 2 {
                    (1, half_univariate.coefficients()[1].1)
                } else {
                    (1, Fq::ZERO)
                },
            ];
            self.previous_polynomial = self.evaluated_polynomial.clone();
            self.evaluated_polynomial = Some(MultiLinearPolynomial::new(1, new_values));
        }
    }
}

pub struct Verifier {
    eqn: MultiLinearPolynomial<Fq>,
    round: usize, // univariate_poly: MultiLinearPolynomial<Fq>,
    random_values: Vec<Fq>,
}

impl Verifier {
    pub fn new(eqn: MultiLinearPolynomial<Fq>, random_values: Vec<Fq>) -> Self {
        Verifier {
            eqn,
            round: 0,
            random_values,
        }
    }

    pub fn verify(&mut self, prover: &Prover, eval_result: Fq, claimed_value: Fq) -> bool {
        if self.round == 0 {
            let sum_0 = prover
                .evaluated_polynomial
                .clone()
                .unwrap()
                .evaluate([Fq::ONE].to_vec());
            let sum_1 = prover
                .evaluated_polynomial
                .clone()
                .unwrap()
                .evaluate([Fq::ZERO].to_vec());
            let calculated = sum_0 + sum_1;

            let eval_random_value = prover
                .evaluated_polynomial
                .clone()
                .unwrap()
                .evaluate([self.random_values[0]].to_vec());

            self.round += 1;
            eval_result == calculated && eval_random_value == claimed_value
        } else if self.round < self.eqn.variables {
            let sum_0 = prover
                .evaluated_polynomial
                .clone()
                .unwrap()
                .evaluate([Fq::ONE].to_vec());
            let sum_1 = prover
                .evaluated_polynomial
                .clone()
                .unwrap()
                .evaluate([Fq::ZERO].to_vec());
            let eval_result = sum_0 + sum_1;

            let prev_evaluation = if self.round > 1 {
                prover
                    .previous_polynomial
                    .clone()
                    .unwrap()
                    .evaluate([self.random_values[self.round - 2]].to_vec())
            } else {
                claimed_value
            };

            self.round += 1;
            eval_result == prev_evaluation
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use ark_bn254::Fq;
    use polynomials::multilinear::mocks::multilinear_polya;

    use crate::{Prover, Verifier};

    #[test]
    fn test_prover_interaction() {
        let mlp = multilinear_polya::<Fq>();

        // verifier random values
        let random_values = vec![Fq::from(3), Fq::from(2), Fq::from(1)];

        let mut prover = Prover::new(mlp.clone());
        let mut verifier = Verifier::new(mlp, random_values.clone());

        prover.prove(0, random_values[..1].to_vec());
        println!(
            "Claimed polynomial: {:?}",
            &prover.evaluated_polynomial.clone()
        );
        // 44 is gotten by evaluating the claimed polynomial at its boolean hypercube.
        let is_verified = verifier.verify(&prover, Fq::from(44), Fq::from(37));

        assert!(is_verified);
        prover.prove(1, random_values[..2].to_vec());
        println!(
            "Claimed polynomial: {:?}",
            &prover.evaluated_polynomial.clone()
        );
        let is_verified = verifier.verify(&prover, Fq::from(44), Fq::from(44));
        assert!(is_verified);

        // let

        prover.prove(2, random_values[..3].to_vec());
        println!(
            "Claimed polynomial: {:?}",
            &prover.evaluated_polynomial.clone()
        );
    }
}
