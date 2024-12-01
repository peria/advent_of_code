use itertools::sorted;
use proconio::input;
use proconio::is_stdin_empty;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn main() {
  let mut sum = 0;
  loop {
    if is_stdin_empty() {
      break;
    }
    input! {
      digits: [String; 10],
      _: String,
      display: [String; 4],
    }
    sum += analyze(&digits, &display);
  }
  println!("{}", sum);
}

#[allow(dead_code)]
fn analyze(digits: &Vec<String>, display: &Vec<String>) -> i64 {
  let digits: Vec<String> = digits
    .iter()
    .map(|x| sorted(x.chars()).collect::<String>())
    .collect();
  let mut dig2seg = Vec::new();
  dig2seg.resize(10, None);
  for dig in digits {
    match dig.len() {
      2 => dig2seg[1] = Some(dig),
      3 => dig2seg[7] = Some(dig),
      4 => dig2seg[4] = Some(dig),
      7 => dig2seg[8] = Some(dig),
      _ => (),
    };
  }

  let seg2dig = HashMap::new();

  let mut value = 0;
  for seg in display {
    let seg: String = sorted(seg.chars()).collect();
    match seg2dig.get(&*seg) {
      Some(v) => value = value * 10 + v,
      None => {}
    }
  }
  value
}
