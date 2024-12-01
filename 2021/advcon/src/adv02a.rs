use proconio::input;
use proconio::is_stdin_empty;

#[allow(dead_code)]
pub fn main() {
  let mut x = 0;
  let mut y = 0;

  loop {
    if is_stdin_empty() {
      break;
    }

    input! {
      dir: String,
      dist: i64,
    }
    match &*dir {
      "forward" => x += dist,
      "down" => y += dist,
      "up" => y -= dist,
      _ => {}
    }
  }

  println!("{}", x * y);
}
