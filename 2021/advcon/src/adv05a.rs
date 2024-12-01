use proconio::input;
use proconio::is_stdin_empty;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn main() {
  let mut lines = Vec::new();
  loop {
    if is_stdin_empty() {
      break;
    }
    input! {
      p0: String,
      _: String,
      p1: String,
    }
    let p0: Vec<i32> = p0.split(',').map(|x| x.parse().unwrap()).collect();
    let p1: Vec<i32> = p1.split(',').map(|x| x.parse().unwrap()).collect();
    lines.push((p0[0], p0[1], p1[0], p1[1]));
  }

  let mut count = HashMap::<(i32, i32), i32>::new();
  for (x0, y0, x1, y1) in lines {
    let dx = (x1 - x0).signum();
    let dy = (y1 - y0).signum();
    // if dx != 0 && dy != 0 {
    //   continue;
    // }
    let (mut x, mut y) = (x0, y0);
    count.insert((x, y), count.get(&(x, y)).unwrap_or(&0) + 1);
    while (x, y) != (x1, y1) {
      x += dx;
      y += dy;
      count.insert((x, y), count.get(&(x, y)).unwrap_or(&0) + 1);
    }
  }
  println!("{}", count.values().filter(|&x| *x >= 2).count());
}
