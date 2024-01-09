use bit_vec::BitVec;
use std::time::Instant;

const MX_N: usize = 10i32.pow(8) as usize;

fn prime_sieve() -> (f64, usize, u64, Vec<usize>) {
    (0.0, 0, 0, vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prime_sieve_single_threaded() -> (usize, u64, Vec<usize>) {
        let mut is_prime = BitVec::from_elem(MX_N, true);
        is_prime.set(0, false);
        is_prime.set(1, false);
        for i in 2..((MX_N as f64).sqrt() + 1 as usize) {
            if is_prime[i] {
                for j in (i * i..MX_N as usize).step_by(i) {
                    is_prime.set(j, false);
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
        (cnt, sum, primes)
    }
    #[test]
    fn test_prime_sieve() {
        let (cnt, sum, primes) = prime_sieve_single_threaded();
        // compare with prime_sieve
        assert_eq!(cnt, 5761455);
        assert_eq!(sum, 279209790387276);
        assert_eq!(
            primes,
            vec![
                99999989, 99999971, 99999959, 99999941, 99999931, 99999847, 99999839, 99999827,
                99999821, 99999787
            ]
        );
    }
}

fn main() {
    let (exe_time, cnt_primes, sum_primes, ten_largest_primes) = prime_sieve();
}
