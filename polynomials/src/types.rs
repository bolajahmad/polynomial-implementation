use ark_ff::PrimeField;

pub trait PolynomialTrait<F: PrimeField> {
    // Gives the value of f(x) at the given value of x;
    fn evaluate(&self, x: F) -> F;
    fn interpolate(points: Vec<(F, F)>) -> Self;
}