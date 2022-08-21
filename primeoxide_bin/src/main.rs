use std::env::args;
use std::time::Instant;

use primeoxide_lib::erat;

fn main() {
    let (program_name, num) = {
        let mut args = args();
        (args.next().unwrap(), args.next())
    };

    let num = if let Some(string) = num {
        string
    } else {
        println!(
            "No argument supplied proper usage: {} <number>",
            program_name
        );
        return;
    };

    let num = match num.parse() {
        Ok(val) => val,
        Err(e) => {
            println!(
                "Failed to parse supplied number \"{}\" with error: {}\nproper usage: {} <number>",
                num, e, program_name
            );
            return;
        }
    };

    let start = Instant::now();
    let (_primes, prime_count) = erat::sieve(num);
    let end = start.elapsed();
    let time = end.as_secs() as f64 + end.subsec_millis() as f64 / 1000.0;

    println!("Seconds: {:.3}", time);
    println!("Primes: {}", prime_count);
}
