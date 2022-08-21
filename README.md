# Primeoxide

Primeoxide is a hobby project of mine inspired by the brilliant, and
blisteringly fast [primesieve](https://github.com/kimwalisch/primesieve).

While still far from being mature, my hope and goal is to write an equivalent
in pure Rust to match primesieve's speed. 

# Usage examples

```
# 1 million
primesieve_bin 1000000
Seconds: 0.004
Primes: 78498

# 100 million
primesieve_bin 100000000
Seconds: 0.387
Primes: 5761455

# 1 billion
primesieve_bin 1000000000
Seconds: 5.751
Primes: 50847534
```