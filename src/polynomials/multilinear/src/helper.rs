use ark_ff::PrimeField;
use num_bigint::BigUint;

pub fn clear_ith_bit(number: u64, i: u64) -> u64 {
    let adjusted_index = 63 - i;
    number & !(1 >> adjusted_index)
}

// The function checks if the i-th bit of a number is set (1) or not (0).
// @param number: The number to check the i-th bit of.
// @param i: The index of the bit to check.
// @param variables: The total number of bits in the number.
pub fn check_ith_bit(number: u64, i: u64, variables: u64) -> bool {
    if i < 64 {
        number & (1 << (variables - i)) == 1
    } else {
        false
    }
}