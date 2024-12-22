use memoize::memoize;

#[memoize]
fn count_paths(side: usize, x: usize, y: usize) -> usize {
    match (x, y) {
        (x, y) if x > side || y > side => 0,
        (x, y) if x == side && y == side => 1,
        (x, y) => count_paths(side, x + 1, y) + count_paths(side, x, y + 1),
    }
}

fn main() {
    println!("{}", count_paths(2, 0, 0));
    println!("{}", count_paths(3, 0, 0));
    println!("{}", count_paths(4, 0, 0));
    println!("{}", count_paths(20, 0, 0));
}
