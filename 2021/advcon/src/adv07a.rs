use proconio::input;

#[allow(dead_code)]
pub fn main() {
  input! {
    crows: String
  }
  let mut crows: Vec<i64> = crows.split(',').map(|x| x.parse().unwrap()).collect();
  crows.sort();
  let p = crows[crows.len() / 2];
  println!("{}", crows.iter().fold(0, |s, &x| s + i64::abs(p - x)));
}
