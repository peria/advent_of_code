use proconio::input;
use proconio::is_stdin_empty;
use std::collections::HashMap;
use substring::Substring;

#[allow(dead_code)]
pub fn main() {
  input! { start: String }
  let mut map = HashMap::new();
  loop {
    if is_stdin_empty() {
      break;
    }
    input! {
      pair: String,
      _: String,
      add: char,
    }
    map.insert(pair, add);
  }
  // End of input

  let mut pairs = HashMap::new();
  for i in 1..start.len() {
    let key = start.substring(i - 1, i + 1);
    let entry = pairs.entry(key.to_string()).or_insert(0);
    *entry += 1i64;
  }

  for _ in 0..40 {
    let prev = pairs.clone();
    pairs.clear();
    for (pair, add) in &map {
      let from = match prev.get(&pair as &str) {
        Some(x) => *x,
        None => 0i64,
      };
      let key: String = vec![pair.chars().nth(0).unwrap(), *add].iter().collect();
      let entry = pairs.entry(key).or_insert(0);
      *entry += from;
      let key: String = vec![*add, pair.chars().nth(1).unwrap()].iter().collect();
      let entry = pairs.entry(key).or_insert(0);
      *entry += from;
    }
  }

  let mut chars = HashMap::new();
  for (pair, num) in &pairs {
    let key = pair.chars().nth(0).unwrap();
    let entry = chars.entry(key).or_insert(0);
    *entry += num;
    let key = pair.chars().nth(1).unwrap();
    let entry = chars.entry(key).or_insert(0);
    *entry += num;
  }

  let min = (chars.values().min().unwrap() + 1i64) / 2i64;
  let max = (chars.values().max().unwrap() + 1i64) / 2i64;
  println!("{}", max - min);
}
