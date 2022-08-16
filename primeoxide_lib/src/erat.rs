use super::bit_vec::BitVec;
use super::wheel::Wheel;

const WHEEL30_INC: [u64; 8] = [4, 2, 4, 2, 4, 6, 2, 6];
const WHEEL210_INC: [u64; 48] = [2, 4, 2, 4, 6, 2, 6, 4, 2, 4, 6, 6, 2, 6, 4, 2, 6, 4, 6, 8, 4, 2,
    4, 2, 4, 8, 6, 4, 6, 2, 4, 6, 2, 6, 6, 4, 2, 4, 6, 2, 6, 4, 2, 4, 2, 10, 2, 10];

pub fn sieve(stop: u64) -> Vec<u64> {
    let mut primes = vec![2, 3, 5, 7];
    let wheel_size = 210;
    let mut wheel = Wheel::new(WHEEL210_INC.to_vec());

    // only store odd numbers
    let mut numbers = BitVec::with_capacity((stop/wheel_size + stop%wheel_size) as usize);

    if stop < u64::MAX {
        let mut num = 11;

        'prime_search: while num < stop {
            let root = (num as f64).sqrt() as u64;

            // we KNOW that no multiples of 2, 3, 5, or 7 are included
            for prime in &primes[4..] {
                if *prime > root {
                    break;
                }

                if num%prime == 0 {
                    num += wheel.next_inc();
                    continue 'prime_search
                }
            }

            // Only get here if no factors were found
            primes.push(num);
            num += wheel.next_inc();
        }
    }

    primes
}
