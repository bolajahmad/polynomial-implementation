use ark_bn254::Fq;
use multilinear::{helper::{check_ith_bit, clear_ith_bit}, mocks::{multilinear_polya, multilinear_polyb}};

fn main() {
    // let number = 29_u64;
    // let i = 0_u64; // Example: Clear the 3rd bit (index 2)

    // let adjusted_index = 63 - i;
    // let new_number = number & !(1 << (5 - 1 - i));

    // println!("Original number: {:b}", number);
    // println!("Number with {} cleared: {:b}", new_number, new_number);

    let mut poly = multilinear_polyb::<Fq>();
       let _ =  poly.partial_evaluate(0, Fq::from(3));
       println!("Partial evaluation: {:?}", poly);
//     let _ = poly.partial_evaluate(5, Fq::from(2));
//    let _ =  poly.partial_evaluate(0, Fq::from(3));
//     let _ = poly.partial_evaluate(1, Fq::from(5));
//     let _ = poly.partial_evaluate(2, Fq::from(1));
//     let _ = poly.partial_evaluate(3, Fq::from(2));

//     println!("Partial evaluation: {:?}", poly);
let result = poly.evaluate([
    Fq::from(3),
    Fq::from(5),
    Fq::from(1),
    Fq::from(2),
    Fq::from(3),
    Fq::from(2),
].to_vec());

println!("Result: {:?}", result);
}