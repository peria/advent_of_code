use proconio::input;
use proconio::is_stdin_empty;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn main() {
  input! {
    numbers: String
  };
  let numbers: Vec<i64> = numbers.split(",").map(|x| x.parse().unwrap()).collect();

  loop {
    if is_stdin_empty() {
      break;
    }
    input! {
      board: [[i64; 5]; 5]
    }
    let (count, score) = bingo(board, &numbers);
    println!("{} {}", count, score);
  }
}

#[allow(dead_code)]
fn bingo(mut board: Vec<Vec<i64>>, numbers: &Vec<i64>) -> (usize, i64) {
  let mut locations: HashMap<i64, (usize, usize)> = HashMap::new();
  for i in 0..5 {
    for j in 0..5 {
      locations.insert(board[i][j], (i, j));
    }
  }
  let locations = locations;

  for (count, number) in numbers.iter().enumerate() {
    let location = locations.get(number);
    if location == None {
      continue;
    }
    let (r, c) = location.unwrap();
    board[*r][*c] = -1;
    if is_bingo(&board, r, c) {
      let score = board
        .concat()
        .iter()
        .filter(|&x| x >= &0)
        .fold(0, |s, x| s + x);
      return (count, number * score);
    }
  }
  (numbers.len(), 0)
}

#[allow(dead_code)]
fn is_bingo(board: &Vec<Vec<i64>>, r: &usize, c: &usize) -> bool {
  let ids = [0, 1, 2, 3, 4];
  return board.iter().all(|x| x[*c] < 0)
    || board[*r].iter().all(|x| x < &0)
    || ids.iter().all(|i| board[*i as usize][*i as usize] < 0)
    || ids
      .iter()
      .all(|i| board[*i as usize][(4 - *i) as usize] < 0);
}
