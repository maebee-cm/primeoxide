use std::env::args;
use std::time::Instant;

use primeoxide_lib::erat;

fn main() {
    let (_program_name, num): (_, u64) = {
        let mut args = args();
        (args.next().unwrap(), args.next().unwrap().parse().unwrap())
    };

    let start = Instant::now();
    let primes = erat::sieve(num);
    let end = start.elapsed();
    let time = end.as_secs() as f64 + end.subsec_millis() as f64 / 1000.0;


    println!("Seconds: {:.3}", time);
    println!("Primes: {}", primes.len());
}
