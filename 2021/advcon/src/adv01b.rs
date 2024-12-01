use std::io::BufRead;

#[allow(dead_code)]
pub fn main() {
  let stdin = std::io::stdin();

  let mut prev2: Option<i64> = None;
  let mut prev1: Option<i64> = None;
  let mut prev = 0;
  let mut count = -1;
  for line in stdin.lock().lines() {
    let measure: i64 = line.unwrap().parse().unwrap();
    if prev2.is_some() {
      let deapth = prev2.unwrap() + prev1.unwrap() + measure;
      count += if prev < deapth { 1 } else { 0 };
      prev = deapth;
    }

    prev2 = prev1;
    prev1 = Some(measure);
  }
  println!("{}", count);
}
