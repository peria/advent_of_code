use proconio;

#[allow(dead_code)]
pub fn main() {
  let mut map: Vec<Vec<i32>> = Vec::new();
  loop {
    if proconio::is_stdin_empty() {
      break;
    }
    proconio::input! {
      line: String,
    }
    map.push(line.chars().map(|x| x as i32 - 48).collect());
  }
  let low_points = get_low_points(&map);

  println!(
    "{}",
    low_points
      .iter()
      .map(|&p| {
        let (i, j) = p;
        map[i][j] + 1
      })
      .sum::<i32>()
  );
}

#[allow(dead_code)]
fn get_low_points(map: &Vec<Vec<i32>>) -> Vec<(usize, usize)> {
  let h = map.len();
  let w = map[0].len();
  let mut low_points = Vec::new();
  for i in 0..h {
    for j in 0..w {
      let x = map[i][j];
      let mut is_low_point = true;
      if i > 0 && x >= map[i - 1][j] {
        is_low_point = false;
      }
      if j > 0 && x >= map[i][j - 1] {
        is_low_point = false;
      }
      if i < h - 1 && x >= map[i + 1][j] {
        is_low_point = false;
      }
      if j < w - 1 && x >= map[i][j + 1] {
        is_low_point = false;
      }
      if is_low_point {
        low_points.push((i, j));
      }
    }
  }
  let low_points = low_points;
  low_points
}
