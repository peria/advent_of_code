use std::collections::HashSet;
use std::io::stdin;

#[allow(dead_code)]
pub fn main() {
  let mut dots = HashSet::new();
  loop {
    let mut line = String::new();
    let _ = stdin().read_line(&mut line);
    let line = line.trim().to_string();
    if line.is_empty() {
      break;
    }
    let mut iter = line.split(',');
    let x = iter.next().unwrap().parse::<usize>().unwrap();
    let y = iter.next().unwrap().parse::<usize>().unwrap();
    dots.insert((x, y));
  }

  loop {
    let mut line = String::new();
    let _ = stdin().read_line(&mut line);
    let line = line.trim().to_string();
    let mut iter = line.split_whitespace();
    if iter.next() == None {
      break;
    }
    iter.next();
    let mut iter = iter.next().unwrap().split('=');
    let pivot = iter.next().unwrap();
    let fold = iter.next().unwrap().parse::<usize>().unwrap();
    let mut after = HashSet::new();
    for dot in &dots {
      let (x, y) = dot;
      if pivot == "x" {
        if x < &fold {
          after.insert((*x, *y));
        } else {
          after.insert((2 * fold - x, *y));
        }
      } else {
        if y < &fold {
          after.insert((*x, *y));
        } else {
          after.insert((*x, 2 * fold - y));
        }
      }
    }
    dots.clone_from(&after);

    println!("{}", dots.len());
  }

  let xmin = *dots.iter().map(|(x, _)| x).min().unwrap() as usize;
  let xmax = *dots.iter().map(|(x, _)| x).max().unwrap() as usize + 1;
  let ymin = *dots.iter().map(|(_, y)| y).min().unwrap() as usize;
  let ymax = *dots.iter().map(|(_, y)| y).max().unwrap() as usize + 1;
  for y in ymin..ymax {
    for x in xmin..xmax {
      if dots.contains(&(x, y)) {
        print!("#");
      } else {
        print!(".");
      }
    }
    println!("");
  }
}
