use proconio::input;
use proconio::is_stdin_empty;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn main() {
  let mut is_big = Vec::<bool>::new();
  let mut edges: Vec<Vec<usize>> = Vec::new();
  is_big.resize(2, false);
  edges.resize(2, Vec::new());

  {
    let mut caves = HashMap::new();
    caves.insert(String::from("start"), 0usize);
    caves.insert(String::from("end"), 1usize);
    loop {
      if is_stdin_empty() {
        break;
      }
      input! { edge: String }
      let nodes: Vec<String> = edge.split('-').map(|x| x.to_string()).collect();
      for n in &nodes {
        if !caves.contains_key(n) {
          caves.insert(n.clone(), caves.len());
          edges.resize(caves.len(), Vec::new());
          is_big.resize(caves.len(), n.chars().nth(0).unwrap().is_uppercase());
        }
      }
      let from = caves[&nodes[0]];
      let to = caves[&nodes[1]];
      edges[from].push(to);
      edges[to].push(from);
    }
  }

  let mut visited = Vec::new();
  visited.resize(edges.len(), 0);
  println!("{}", search(0, &edges, &is_big, &mut visited, false));
}

fn search(
  from: usize,
  edges: &Vec<Vec<usize>>,
  is_big: &Vec<bool>,
  visited: &mut Vec<usize>,
  mut twice: bool,
) -> i32 {
  if from == 1 {
    return 1;
  }

  visited[from] += 1;
  let second = !is_big[from] && visited[from] > 1;
  if second {
    twice = true;
  }

  let mut count = 0;
  for &to in &edges[from] {
    if to == 0 || (!is_big[to] && visited[to] > 0 && twice) {
      continue;
    }
    count += search(to, edges, is_big, visited, twice);
  }

  visited[from] -= 1;
  count
}
