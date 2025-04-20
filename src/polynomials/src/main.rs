fn check_is_1_bit(value: u64, idx: usize) -> bool {
    // If the ith index is not 1, then the result will be another integer
    (value & mask(idx)) != 0
}

fn mask(idx: usize) -> u64 {
    // start with 1, shift left idx times
    // e.g. 1 << 5 === 100000
    // e.g. 1 << 4 === 10000
    1 << idx
}

fn loop_through() {
    let id = 3;
    let n = 42;

    for i in 0..8 {
        let bit = (n >> i) & 1;
        println!("Bit at position {}: {}", i, bit);

        if bit == 1 {
            println!("1 bit found");
        } else {
            println!("0 Bit found");
        }
    }
}

pub fn clear_ith_bit(number: u64, index: usize) -> u64 {
    number ^ (1 << index)
}

fn main() {
    let max_variables = 5;
    let value = 29;
    let id = 3;
    let idx = (max_variables - 1) - id;

    // // println!("Value: {}", value ^ (1 << idx));

    // // loop_through();
    // // println!("1 shofted idx {} times: {}", 2, 1 << 1 );
    // // println!("Character at ith index: {}", 1 << 4);
    // println!("Number after clearing ith bit at index {} is {}", idx, clear_ith_bit(value, idx));

    // if check_is_1_bit(value, idx) {
    //     println!("Bit at index {} is 1", idx);
    //     println!("Number after clearing ith bit at index {} is {}", idx, clear_ith_bit(value, idx));
    // }


    // f(a,b) = 2ab + 3a + b
    // f(2, 5) = 31
    // f(3,4) = 40

    // 00, 01, 10, 11
    // [2, 5, 6, 10]

    // degree = x, x+1
}