use proconio::input;
use proconio::is_stdin_empty;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
  cost: usize,
  i: usize,
  j: usize,
}

impl Ord for State {
  fn cmp(&self, other: &Self) -> Ordering {
    other.cost.cmp(&self.cost)
  }
}

impl PartialOrd for State {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

#[allow(dead_code)]
pub fn minimum_risk(map: &Vec<Vec<usize>>) -> usize {
  static MOVE: &'static [(isize, isize)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];
  let n = map.len();
  let m = map[0].len();

  let mut risk: Vec<Vec<_>> = vec![vec![usize::MAX; m]; n];
  let mut pq = BinaryHeap::new();
  risk[0][0] = 0;
  pq.push(State {
    cost: 0,
    i: 0,
    j: 0,
  });
  println!("{}", pq.len());
  while let Some(State { cost, i, j }) = pq.pop() {
    if i == n - 1 && j == m - 1 {
      return cost;
    }
    if cost > risk[i][j] {
      continue;
    }
    for (di, dj) in MOVE {
      let (ii, jj) = (i as isize + di, j as isize + dj);
      if ii < 0 || jj < 0 || ii >= n as isize || jj >= m as isize {
        continue;
      }
      let ii = ii as usize;
      let jj = jj as usize;
      let ncost = cost + map[ii][jj];
      if ncost < risk[ii][jj] {
        risk[ii][jj] = ncost;
        pq.push(State {
          cost: ncost,
          i: ii,
          j: jj,
        });
      }
    }
  }
  0usize
}

#[allow(dead_code)]
pub fn main() {
  let mut map: Vec<Vec<usize>> = Vec::new();
  loop {
    if is_stdin_empty() {
      break;
    }
    input! { line: String }
    map.push(line.chars().map(|x| x as usize - '0' as usize).collect());
  }
  println!("{}", minimum_risk(&map));
}
