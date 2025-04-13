
use num_bigint::{BigInt, BigUint, Sign};

fn main() {
assert_eq!(BigInt::from(-1125).to_u64_digits(), (Sign::Minus, vec![1125]));
assert_eq!(BigInt::from(4294967295u32).to_u64_digits(), (Sign::Plus, vec![4294967295]));
assert_eq!(BigInt::from(4294967296u64).to_u64_digits(), (Sign::Plus, vec![4294967296]));
assert_eq!(BigInt::from(-112500000000i64).to_u64_digits(), (Sign::Minus, vec![112500000000]));
assert_eq!(BigInt::from(112500000000i64).to_u64_digits(), (Sign::Plus, vec![112500000000]));
assert_eq!(BigInt::from(1u128 << 64).to_u64_digits(), (Sign::Plus, vec![0, 1]));


let representaion = BigUint::from_bytes_be("21edfbe93d0038e8c59e0fd7550162142664d0b3e3f1446572495bab062959cc".as_bytes());
println!("Integer returned, {:?}", representaion.to_u64_digits())
}