mod bit_vec;
pub mod erat;
mod wheel;

#[cfg(test)]
mod tests {
    use crate::erat;

    #[test]
    fn print_primes() {
        let primes = erat::sieve(1000000);
        assert_eq!(primes.get_population_count(None), 78498);
    }
}
