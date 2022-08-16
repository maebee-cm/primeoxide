pub mod erat;
mod bit_vec;
mod wheel;

#[cfg(test)]
mod tests {
    use crate::erat;

    #[test]
    fn print_primes() {
        let primes = erat::sieve(1000000);
        println!("primes found: {}", primes.len());

        println!("done!");
    }
}
