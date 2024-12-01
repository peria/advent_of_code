use std::io::BufRead;

#[allow(dead_code)]
pub fn main() {
  let stdin = std::io::stdin();

  let mut reports = Vec::new();
  for line in stdin.lock().lines() {
    reports.push(line.unwrap());
  }

  let n = reports.len();
  let m = reports[0].len();
  let mut gamma: i64 = 0;
  let mut epsilon: i64 = 0;
  for i in 0..m {
    let on = reports
      .iter()
      .filter(|&x| x.chars().nth(i).unwrap() == '1')
      .count();
    gamma <<= 1;
    epsilon <<= 1;
    if on > n / 2 {
      gamma += 1;
    } else {
      epsilon += 1;
    }
  }
  println!("{}", gamma * epsilon);
}
