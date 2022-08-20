use super::bit_vec::BitVec;
use super::wheel::Wheel;

const _WHEEL30_INC: [u64; 8] = [4, 2, 4, 2, 4, 6, 2, 6];
const WHEEL210_INC: [u64; 48] = [2, 4, 2, 4, 6, 2, 6, 4, 2, 4, 6, 6, 2, 6, 4, 2, 6, 4, 6, 8, 4, 2,
    4, 2, 4, 8, 6, 4, 6, 2, 4, 6, 2, 6, 6, 4, 2, 4, 6, 2, 6, 4, 2, 4, 2, 10, 2, 10];

const WHEEL210: [u64; 48] = [1, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73,
    79, 83, 89, 97, 101, 103, 107, 109, 113, 121, 127, 131, 137, 139, 143, 149, 151, 157, 163, 167,
    169, 173, 179, 181, 187, 191, 193, 197, 199, 209];

pub fn sieve(stop: u64) -> Vec<u64> {
    let wheel_size = 210;
    let wheel_len = WHEEL210_INC.len();
    let mut wheel = Wheel::new(&WHEEL210_INC);
    // Create this here rather than when we use it, sync later. Stops expensive allocation from
    // happening everytime we find a new prime.
    let mut multiplier_wheel = wheel.clone();

    let mut primes = vec![2, 3, 5, 7];

    // Store only numbers which exist in our wheel of choice. For now this is hardcoded to a 210
    // wheel, however this may change in the future.
    // This calculation slightly over-allocates, but those values aren't ever touched, and at most
    // will take up a few extra bytes, so it doesn't really matter.
    let bits = (stop/wheel_size*wheel_len+stop%wheel_size) as usize;
    let mut numbers = BitVec::with_capacity(bits, true);

    if stop < u64::MAX {
        let mut num = 11;
        let mut iterations = 0;
        let stop_root = (stop as f64).sqrt() as u64;

        while num <= stop {
            if !numbers.get_bit(iterations) {
                num += wheel.next_inc();
                iterations += 1;
                continue;
            }

            // Only get here if no factors were found
            if num <= stop_root {
                multiplier_wheel.sync(&wheel);
                let mut multiplier = num;
                let mut result = multiplier*num;
                while result <= stop {
                    let index = get_num_idx(result);
                    numbers.set_bit(index, false);

                    multiplier += multiplier_wheel.next_inc();
                    result = multiplier*num;
                }
            }

            primes.push(num);
            num += wheel.next_inc();
            iterations += 1;
        }
    }

    primes
}

fn get_num_idx(num: u64) -> usize {
    // Our number line starts at 11, yet our list of remainders starts 1. The calculations doesn't
    // really work if the number line doesn't start at 1, and it's more of a headache to have the
    // the vector start with 1. So instead this formula produced the index for a 1-indexed array
    // and then we decrement by 1.
    ((num/210)*48) as usize + WHEEL210.iter().position(|&x| x == num%210).unwrap() - 1
}