use bit_vec::BitVec;
use rayon::prelude::*;
use std::sync::atomic::{AtomicI32, AtomicI64};
use std::time::{Duration, Instant};

const MX_N: usize = 10i32.pow(8) as usize;
const NUM_THREADS: usize = 8;

fn calc_is_prime(mx_n: usize) -> BitVec {
    let mut is_prime = BitVec::from_elem(mx_n, true);
    is_prime.set(0, false);
    is_prime.set(1, false);
    for i in 2..(((mx_n as f64).sqrt() + 1.0) as usize) {
        if is_prime[i] {
            for j in (i * i..mx_n).step_by(i) {
                is_prime.set(j, false);
            }
        }
    }
    is_prime
}

fn prime_sieve() -> (Duration, usize, u64, Vec<usize>) {
    let start = Instant::now();

    let is_prime_small = calc_is_prime(((MX_N as f64).sqrt() as usize) + 1);
    let mut list_of_primes = Vec::<usize>::new();
    for i in 0..is_prime_small.len() {
        if is_prime_small[i] {
            list_of_primes.push(i);
        }
    }

    let cnt = AtomicI32::new(MX_N as i32 - 2);
    let sum = AtomicI64::new((MX_N * (MX_N - 1) / 2) as i64 - 1);

    let block_size = MX_N / NUM_THREADS;
    let mut is_prime = vec![true; MX_N];
    is_prime[0] = false;
    is_prime[1] = false;
    is_prime
        .par_chunks_mut(block_size)
        .enumerate()
        .for_each(|(chunk_index, chunk)| {
            for &prime in &list_of_primes {
                let base = chunk_index * block_size;
                let start = if base <= prime {
                    prime * prime
                } else {
                    prime * ((base + prime - 1) / prime)
                };
                for j in (start..base + chunk.len()).step_by(prime) {
                    if chunk[j - base] {
                        cnt.fetch_add(-1, std::sync::atomic::Ordering::Relaxed);
                        sum.fetch_add(-(j as i64), std::sync::atomic::Ordering::Relaxed);
                    }
                    chunk[j - base] = false;
                }
            }
        });

    let mut ten_largest_primes = Vec::<usize>::new();
    for i in (0..MX_N).rev() {
        if ten_largest_primes.len() >= 10 {
            break;
        }
        if is_prime[i] {
            ten_largest_primes.push(i);
        }
    }
    ten_largest_primes.reverse();

    let elapsed = start.elapsed();

    (
        elapsed,
        cnt.load(std::sync::atomic::Ordering::Relaxed) as usize,
        sum.load(std::sync::atomic::Ordering::Relaxed) as u64,
        ten_largest_primes,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prime_sieve_single_threaded() -> (usize, u64, Vec<usize>) {
        let is_prime = calc_is_prime(MX_N);
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
