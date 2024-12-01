use proconio::input;
use proconio::is_stdin_empty;

#[allow(dead_code)]
pub fn main() {
  let mut prev: Option<i64> = None;
  let mut count = 0;

  loop {
    if is_stdin_empty() {
      break;
    }
    input! {
        depth: i64
    }

    count += match prev {
      Some(x) => {
        if x < depth {
          1
        } else {
          0
        }
      }
      None => 0,
    };
    prev = Some(depth);
  }
  println!("{}", count);
}
