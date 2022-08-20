use super::bit_vec::BitVec;
use super::wheel::Wheel;

const _WHEEL30_INC: [u64; 8] = [4, 2, 4, 2, 4, 6, 2, 6];
const WHEEL210_INC: [u64; 48] = [2, 4, 2, 4, 6, 2, 6, 4, 2, 4, 6, 6, 2, 6, 4, 2, 6, 4, 6, 8, 4, 2,
    4, 2, 4, 8, 6, 4, 6, 2, 4, 6, 2, 6, 6, 4, 2, 4, 6, 2, 6, 4, 2, 4, 2, 10, 2, 10];

pub fn sieve(stop: u64) -> Vec<u64> {
    let mut wheel = Wheel::new(&WHEEL210_INC);
    // Create this here rather than when we use it, sync later. Stops expensive allocation from
    // happening everytime we find a new prime.
    let mut multiplier_wheel = wheel.clone();

    let mut primes = vec![2, 3, 5, 7];

    // Store all numbers. For efficiency reasons we wont be setting multiples of 2, 3, 5, or 7
    // to 0 even though we already know they aren't primes. This is because we are only reading
    // values at increment indices provided by our wheel, which means we should never read their
    // value regardless.
    let mut numbers = BitVec::with_capacity(stop as usize, true);

    if stop < u64::MAX {
        let mut num = 11;
        let stop_root = (stop as f64).sqrt() as u64;

        while num <= stop {
            if !numbers.get_bit((num - 1) as usize) {
                num += wheel.next_inc();
                continue;
            }

            // Only get here if no factors were found
            if num <= stop_root {
                multiplier_wheel.sync(&wheel);
                let mut multiplier = num;
                let mut result = multiplier*num;
                while result <= stop {
                    numbers.set_bit((result - 1) as usize, false);

                    multiplier += multiplier_wheel.next_inc();
                    result = multiplier*num;
                }
            }

            primes.push(num);
            num += wheel.next_inc();
        }
    }

    primes
}