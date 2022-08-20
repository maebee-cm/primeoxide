pub mod erat;
mod bit_vec;
mod wheel;

#[cfg(test)]
mod tests {
    use crate::erat;

    #[test]
    fn print_primes() {
        let (_primes, prime_count) = erat::sieve(1000000);
        assert_eq!(prime_count, 78498);
        println!("primes found: {}", prime_count);

        println!("done!");
    }
}
