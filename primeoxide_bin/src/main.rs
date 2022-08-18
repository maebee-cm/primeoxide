use std::env::args;
use primeoxide_lib::erat;

fn main() {
    let (_program_name, num): (_, u64) = {
        let mut args = args();
        (args.next().unwrap(), args.next().unwrap().parse().unwrap())
    };

    println!("Finding all primes under {}", num);
    let primes = erat::sieve(num);
    println!("Found {} primes under {}", primes.len(), num)
}
