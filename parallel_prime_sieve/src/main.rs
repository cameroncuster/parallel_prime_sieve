use bit_vec::BitVec;
use rayon::prelude::*;
use std::time::{Duration, Instant};

const MX_N: usize = 10i32.pow(8) as usize;

fn prime_sieve() -> (Duration, usize, u64, Vec<usize>) {
    let start = Instant::now();
    let elapsed = start.elapsed();

    (elapsed, 0, 0, vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prime_sieve_single_threaded() -> (usize, u64, Vec<usize>) {
        let mut is_prime = BitVec::from_elem(MX_N, true);
        is_prime.set(0, false);
        is_prime.set(1, false);
        for i in 2..(((MX_N as f64).sqrt() + 1.0) as usize) {
            if is_prime[i] {
                for j in ((i as u64) * (i as u64)..MX_N as u64).step_by(i) {
                    is_prime.set(j as usize, false);
                }
            }
        }
        let mut cnt = 0;
        let mut sum: u64 = 0;
        let mut primes: Vec<usize> = Vec::new();
        for i in (0..MX_N).rev() {
            if is_prime[i] {
                cnt += 1;
                sum += i as u64;
                if primes.len() < 10 {
                    primes.push(i);
                }
            }
        }
        primes.reverse();
        (cnt, sum, primes)
    }
    #[test]
    fn test_prime_sieve() {
        let (cnt_single, sum_single, ten_largest_primes_single) = prime_sieve_single_threaded();
        let (_exe_time, cnt_multi, sum_multi, ten_largest_primes_multi) = prime_sieve();
        assert_eq!(cnt_single, cnt_multi);
        assert_eq!(sum_single, sum_multi);
        assert_eq!(ten_largest_primes_single, ten_largest_primes_multi);
    }
}

fn main() {
    let (exe_time, cnt_primes, sum_primes, ten_largest_primes) = prime_sieve();

    let ten_largest_primes_str = ten_largest_primes
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    let output = format!(
        "{:?} {} {}\n{}\n",
        exe_time, cnt_primes, sum_primes, ten_largest_primes_str
    );

    std::fs::write("primes.txt", output).expect("Unable to write to file");
}
