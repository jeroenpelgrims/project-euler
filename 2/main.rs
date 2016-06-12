struct Fibonacci {
  curr: u64,
  next: u64
}

impl Iterator for Fibonacci {
  type Item = u64;

  fn next(&mut self) -> Option<u64> {
    let new_next = self.curr + self.next;

    self.curr = self.next;
    self.next = new_next;

    Some(self.curr)
  }
}

fn fibonacci() -> Fibonacci {
  Fibonacci { curr: 1, next: 1 }
}

fn main() {
  let fib_max = 4_000_000;
  let result: u64 = fibonacci()
    .take_while(|x| x <= &fib_max)
    .filter(|x| x % 2 == 0)
    .fold(0u64, |result, next| result + next);

  println!("{}", result);
}