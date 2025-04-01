pub trait PolynomialTrait {
    // Gives the value of f(x) at the given value of x;
    fn evaluate(&self, x: f64) -> f64;
    fn interpolate(points: Vec<(f64, f64)>) -> Self;
}