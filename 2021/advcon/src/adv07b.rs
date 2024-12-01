use proconio::input;
use std::cmp;

#[allow(dead_code)]
pub fn main() {
  input! {
    crows: String
  }
  let crows: Vec<i64> = crows.split(',').map(|x| x.parse().unwrap()).collect();

  let mut min = None;
  for x in 0..1000 {
    let mut sum = 0;
    for c in &crows {
      let d = i64::abs(x - c);
      sum += d * (d + 1) / 2;
    }
    match min {
      None => min = Some(sum),
      Some(s) => min = Some(cmp::min(s, sum)),
    }
  }
  println!("{}", min.unwrap());
}
