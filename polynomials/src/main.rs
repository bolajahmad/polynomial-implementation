use polynomials::Polynomials;
use polynomials::types::PolynomialTrait;

fn main() {
    
    // let polynomial = Polynomials::new(
    //     vec![10, -1, 7, 1],
    //     3
    // ).unwrap();

    // let y1 = polynomial.evaluate(1);
    // let y2 = polynomial.evaluate(2);
    // let y3 = polynomial.evaluate(4);
    // let y4 = polynomial.evaluate(5);

    // println!("f(1) = {}", y1);
    // println!("f(2) = {}", y2);
    // println!("f(4) = {}", y3);
    // println!("f(5) = {}", y4);
    let poly = Polynomials::new(
        vec![-40.0, 38.0, -11.0, 1.0],
        3
    ).unwrap();

    let polyb = Polynomials::new(
        vec![-20.0, 29.0, -10.0, 1.0],
        3
    ).unwrap();
    let polyb_result = polyb.scalar_mul(44.0/6.0);
    let result = poly.scalar_mul(-17.0/12.0);
    // let result = &(&polya * &polyb) * &polyc;
    println!("Result: {:?}", result);
    println!("Result: {:?}", polyb_result);

    println!("Result Addition: {:?}", &result + &polyb_result);
}
