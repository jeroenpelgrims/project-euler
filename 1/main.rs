fn main() {
  let max: u64 = 1000;
  let result: u64 = (0..max).fold(0u64, |result, i| {
    if i % 3 == 0 || i % 5 == 0 {
      result + i
    } else {
      result
    }
  });
  
  println!("{}", result);
}