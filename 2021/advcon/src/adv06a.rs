use proconio::input;

#[allow(dead_code)]
pub fn main() {
  const DAYS: i32 = 256;
  const DURATION: usize = 7;

  input! {
    fish: String,
  }

  let mut count = Vec::new();
  count.resize(DURATION + 2, 0);
  for x in fish.split(',') {
    count[x.parse::<usize>().unwrap()] += 1;
  }
  for _ in 0..DAYS {
    let add = count[0];
    for i in 1..DURATION + 2 {
      count[i - 1] = count[i];
    }
    count[DURATION + 1] = add;
    count[DURATION - 1] += add;
  }
  println!("{}", count.iter().sum::<i64>());
}
