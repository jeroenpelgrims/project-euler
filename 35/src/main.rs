use core::num;

use memoize::memoize;
use primal::is_prime;

fn rotations(number: u64) -> Vec<u64> {
    let string = number.to_string();
    let chars: Vec<_> = string.chars().collect();
    let char_rotations = (0..string.len()).map(|n| {
        let mut copy = chars.clone();
        copy.rotate_right(n);
        copy
    });
    char_rotations
        .map(|xs| xs.iter().collect::<String>().parse::<u64>().unwrap())
        .collect()
}

#[memoize]
fn is_circular_prime(number: u64) -> bool {
    rotations(number).iter().all(|x| is_prime(*x))
}

fn main() {
    let circular_primes = (1..1_000_000).filter(|x| is_circular_prime(*x)).count();
    println!("{:?}", circular_primes);
}
