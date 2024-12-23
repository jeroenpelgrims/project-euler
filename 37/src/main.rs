use memoize::memoize;
use primal::is_prime;

fn right_truncations(x: u64) -> Vec<u64> {
    if x < 10 {
        vec![x]
    } else {
        let right_trimmed = x / 10;
        vec![vec![right_trimmed], right_truncations(right_trimmed)].concat()
    }
}

fn left_truncations(x: u64) -> Vec<u64> {
    if x < 10 {
        vec![x]
    } else {
        let zeroes = (x as f64).log10() as u64;
        let left_unit = (10 as u64).pow(zeroes as u32);
        let left_trimmed = x - (x / left_unit * left_unit);
        vec![vec![left_trimmed], left_truncations(left_trimmed)].concat()
    }
}

#[memoize]
fn is_truncatable(x: u64) -> bool {
    let lefts = left_truncations(x);
    let rights = right_truncations(x);
    let trunctations = vec![lefts, rights].concat();

    is_prime(x) && trunctations.iter().all(|&y| is_prime(y))
}

fn main() {
    let sum: u64 = (1..)
        .filter(|&x| is_prime(x))
        .skip(4)
        .filter(|&x| is_truncatable(x))
        .take(11)
        .sum();
    println!("{:?}", sum);
}
