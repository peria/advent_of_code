use proconio::input;
use proconio::is_stdin_empty;

#[allow(dead_code)]
pub fn main() {
  let mut score = 0;
  let mut imcomps = Vec::new();
  loop {
    if is_stdin_empty() {
      break;
    }
    input! {
      chunk: String,
    }
    let (s, i) = get_score(&chunk);
    score += s;
    match i {
      Some(x) => imcomps.push(x),
      None => (),
    }
  }
  imcomps.sort();

  println!("{}", score);
  println!("{}", imcomps[imcomps.len() / 2]);
}

#[allow(dead_code)]
fn get_score(chunk: &String) -> (i64, Option<i64>) {
  let mut stack = Vec::new();
  for c in chunk.chars() {
    match c {
      '(' => stack.push(')'),
      '[' => stack.push(']'),
      '{' => stack.push('}'),
      '<' => stack.push('>'),
      _ => match stack.last() {
        Some(x) => {
          if *x == c {
            stack.pop();
          } else {
            match c {
              ')' => return (3, None),
              ']' => return (57, None),
              '}' => return (1197, None),
              '>' => return (25137, None),
              _ => (),
            };
          }
        }
        None => {
          println!("empty stack");
          return (0, None);
        }
      },
    }
  }
  if stack.is_empty() {
    return (0, None);
  }
  let mut score = 0;
  for x in stack.iter().rev() {
    score = score * 5
      + match x {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
      };
  }
  (0, Some(score))
}
