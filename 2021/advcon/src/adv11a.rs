use proconio::input;
use proconio::is_stdin_empty;
use std::collections::HashSet;

#[allow(dead_code)]
pub fn main() {
  let mut octopus: Vec<Vec<i32>> = Vec::new();
  loop {
    if is_stdin_empty() {
      break;
    }
    input! {
      line: String,
    }
    octopus.push(line.chars().map(|x| x as i32 - '0' as i32).collect());
  }
  let h = octopus.len();
  let w = octopus[0].len();

  let mut flash = 0;
  for i in 0..1000 {
    let f = run_step(&mut octopus);
    if f == h * w {
      println!("B: {}", i + 1);
      break;
    }
    flash += f;
    if i == 99 {
      println!("A: {}", flash);
    }
  }
  //println!("{:?}", octopus);
}

fn run_step(octs: &mut Vec<Vec<i32>>) -> usize {
  let diffs = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
  ];
  let h = octs.len() as i32;
  let w = octs[0].len() as i32;

  for i in 0..h as usize {
    for j in 0..w as usize {
      octs[i][j] += 1;
    }
  }

  let mut flash = HashSet::new();
  loop {
    let mut updated = false;
    for i in 0..h {
      for j in 0..w {
        if octs[i as usize][j as usize] > 9 && !flash.contains(&(i, j)) {
          updated = true;
          flash.insert((i, j));
          for (di, dj) in &diffs {
            let (ii, jj) = (i + di, j + dj);
            if ii < 0 || jj < 0 || ii >= h || jj >= w {
              continue;
            }
            octs[ii as usize][jj as usize] += 1;
          }
        }
      }
    }
    if !updated {
      break;
    }
  }
  for (i, j) in &flash {
    octs[*i as usize][*j as usize] = 0;
  }
  flash.len()
}
