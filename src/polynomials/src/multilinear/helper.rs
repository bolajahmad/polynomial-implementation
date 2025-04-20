use ark_ff::PrimeField;
use num_bigint::BigUint;

/**
 * Clears the ith bit of a binary number.
 * Ensure that the value passes has a 1 at the ith bit else this will have the revrse effect.
 * @param value to clear the ith bit of
 * @param index of the bit to clear [must have a 1 at this ith bit]
 * @return the new number with the ith bit cleared
 */
pub fn clear_ith_bit(value: u64, index: usize) -> u64 {
    value ^ (1 << index)
}

/**
 * Checks if the ith bit of a binary number is 1.
 * If the ith bit is 1, the value will automatically become 0
 * Else, it will result in a new number
 * @param value to check the ith bit of
 * @param index of the bit to check
 * @return true if the ith bit is 1, false otherwise
 */
pub fn check_is_1_bit(value: u64, idx: usize) -> bool {
    // (value & mask) != 0 
    (value & mask(idx)) != 0
}

/**
 * Helper function to return a 1 followed by idx 0s
 * e.g. 1 << 5 === 100000
 * e.g. 1 << 4 === 10000
 */
pub fn mask(idx: usize) -> u64 {
    1 << idx
}
