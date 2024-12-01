use proconio::input;
use proconio::is_stdin_empty;

#[allow(dead_code)]
pub fn main() {
  let mut number = 0;
  loop {
    if is_stdin_empty() {
      break;
    }
    input! {
      _digits: [String; 10],
      _: String,
      display: [String; 4],
    }
    for d in display {
      match d.len() {
        2 => number += 1, // 1
        3 => number += 1, // 7
        4 => number += 1, // 4
        7 => number += 1, // 8
        _ => {}
      }
    }
  }
  println!("{}", number);
}
