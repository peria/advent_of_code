use proconio::input;
use proconio::is_stdin_empty;

enum Snail {
  Pair {
    parent: Box<Option<Snail>>,
    left: Box<Snail>,
    right: Box<Snail>,
  },
  Number {
    val: i32,
  },
}

impl Snail {
  fn magnitude(&self) -> i32 {
    match self {
      Snail::Pair {
        parent,
        left,
        right,
      } => left.magnitude() * 3 + right.magnitude() * 2,
      Snail::Number { val } => *val,
    }
  }

  fn is_valid(&self, level: i32) -> bool {
    match self {
      Snail::Pair {
        parent,
        left,
        right,
      } => level < 4 && left.is_valid(level + 1) && right.is_valid(level + 1),
      Snail::Number { val } => *val < 10,
    }
  }
}

fn reduce(_snail: &mut Snail) -> bool {
  false
}

fn make_snail(iter: &mut impl Iterator<Item = char>) -> Snail {
  let c = iter.next().unwrap();
  match c {
    '[' => {
      let left = make_snail(iter);
      iter.next();
      let right = make_snail(iter);
      iter.next(); // skip ']'
      Snail::Pair {
        parent: Box::new(None),
        left: Box::new(left),
        right: Box::new(right),
      }
    }
    _ => {
      let val = c as i32 - '0' as i32;
      Snail::Number { val: val }
    }
  }
}

#[allow(dead_code)]
pub fn main() {
  let mut root = None;
  loop {
    if is_stdin_empty() {
      break;
    }
    input! { line: String }
    let mut snail = make_snail(&mut line.chars());
    // root = root + snail
    let root = match root {
      None => Some(snail),
      _ => Some(Snail::Pair {
        parent: Box::new(None),
        left: Box::new(root.unwrap()),
        right: Box::new(snail),
      }),
    };
  }
  let mut root = root.unwrap();
  while !reduce(&mut root) {}
  println!("{}", root.magnitude());
}
