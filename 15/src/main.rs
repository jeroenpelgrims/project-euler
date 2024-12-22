use memoize::memoize;

#[memoize]
fn count_paths(side: usize, x: usize, y: usize) -> usize {
    if x == side && y == side {
        1
    } else {
        let mut sum = 0;
        if x < side {
            sum = sum + count_paths(side, x + 1, y);
        }
        if y < side {
            sum = sum + count_paths(side, x, y + 1);
        }
        sum
    }
}

fn main() {
    println!("{}", count_paths(2, 0, 0));
    println!("{}", count_paths(3, 0, 0));
    println!("{}", count_paths(4, 0, 0));
    println!("{}", count_paths(20, 0, 0));
}
