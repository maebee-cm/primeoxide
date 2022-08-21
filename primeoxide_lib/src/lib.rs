mod bit_vec;
pub mod erat;
mod wheel;

#[cfg(test)]
mod tests {
    use crate::erat;

    #[test]
    fn print_primes() {
        let (_primes, prime_count) = erat::sieve(1000000);
        assert_eq!(prime_count, 78498);
    }
}
