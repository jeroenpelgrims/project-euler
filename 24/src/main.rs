use itertools::Itertools;

fn main() {
    let digits = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let permutations = digits.iter().permutations(digits.len());
    let millionth = permutations.skip(999_999).next().unwrap();
    let number = millionth.iter().fold(0 as i64, |acc, x| acc * 10 + *x);
    println!("{}", number);
}
