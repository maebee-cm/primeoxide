use super::bit_vec::BitVec;
use super::wheel::Wheel;

const WHEEL210_INC: [u64; 48] = [
    2, 4, 2, 4, 6, 2, 6, 4, 2, 4, 6, 6, 2, 6, 4, 2, 6, 4, 6, 8, 4, 2, 4, 2, 4, 8, 6, 4, 6, 2, 4, 6,
    2, 6, 6, 4, 2, 4, 6, 2, 6, 4, 2, 4, 2, 10, 2, 10,
];

const WHEEL210: [u64; 48] = [
    1, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97, 101,
    103, 107, 109, 113, 121, 127, 131, 137, 139, 143, 149, 151, 157, 163, 167, 169, 173, 179, 181,
    187, 191, 193, 197, 199, 209
];

/// Performs a sieve of Eratosthenes in the number range [2..`stop`]
pub fn sieve(stop: u64) -> BitVec {
    let wheel_size = 210;
    let wheel_len = WHEEL210_INC.len() as u64;
    let mut wheel = Wheel::new(&WHEEL210_INC);
    // Create this here rather than when we use it, sync later. Stops expensive allocation from
    // happening everytime we find a new prime.
    let mut multiplier_wheel = wheel.clone();

    // Store only numbers which exist in our wheel of choice. For now this is hardcoded to a 210
    // wheel, however this may change in the future.
    // This by itself slightly under counts
    let mut bits = (stop / wheel_size * wheel_len) as usize;
    // Gives us the max number we can store while accounting for the fractions lost from the earlier
    // division.
    let mut max_num = bits as u64 / wheel_len * wheel_size;
    // Figure out how many more bits we need without going over `stop`. The reason want an exact
    // count of how many bits we need is because this later lets us use the very efficient popcnt
    // instruction, or an equivalent to find all primes after the square root of `stop`
    while max_num < stop {
        bits += 1;
        max_num += wheel.next_inc();
    }
    wheel.reset();

    let mut numbers = BitVec::with_capacity(bits, true);

    let mut num = 11;
    // iterations starts at 4 because we treat the pre-sieved 2, 3, 5, and 7 as if they were
    // made in here
    let mut iterations = 4;
    let stop_root = (stop as f64).sqrt() as u64;

    while num <= stop_root {
        if !unsafe {numbers.get_bit(iterations)} {
            num += wheel.next_inc();
            iterations += 1;
            continue;
        }

        // Only get here if no factors were found
        multiplier_wheel.sync(&wheel);
        let mut multiplier = num;
        let mut result = multiplier * num;
        while result <= stop {
            let index = unsafe { get_num_idx(result) };
            unsafe { numbers.set_bit(index, false); }

            multiplier += multiplier_wheel.next_inc();
            result = multiplier * num;
        }

        num += wheel.next_inc();
        iterations += 1;
    }

    numbers
}

/// Get the index of the number passed
unsafe fn get_num_idx(num: u64) -> usize {
    let div = num/210;
    let rem = num%210;

    let mut wheel_pos = WHEEL210.iter().position(|&x| x == rem).unwrap();

    // Our number line starts at 11, yet our list of remainders starts at 1. The calculations doesn't
    // really work if the number line doesn't start at 1, and it's more of a headache to have the
    // the vector start with 1. So instead this formula produced the index for a 1-indexed array
    // and then we increment by (4-1) since there are 4 pres-sieved primes.
    (div * 48) as usize + wheel_pos + 3
}
