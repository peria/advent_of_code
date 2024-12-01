use std::io::BufRead;

#[allow(dead_code)]
pub fn main() {
  let stdin = std::io::stdin();

  let mut reports = Vec::new();
  for line in stdin.lock().lines() {
    reports.push(line.unwrap());
  }
  let reports = reports;

  let n = reports[0].len();
  let mut r = reports.clone();
  for i in 0..n {
    let mut on = Vec::new();
    let mut off = Vec::new();
    for x in r {
      if x.chars().nth(i).unwrap() == '1' {
        on.push(x);
      } else {
        off.push(x);
      }
    }
    if off.len() > on.len() {
      r = off.iter().map(|x| x.to_string()).collect();
    } else {
      r = on.iter().map(|x| x.to_string()).collect();
    }
  }
  let o2 = i64::from_str_radix(&*r[0], 2).unwrap();

  let mut r = reports.clone();
  for i in 0..n {
    if r.len() == 1 {
      break;
    }
    let mut on = Vec::new();
    let mut off = Vec::new();
    for x in r {
      if x.chars().nth(i).unwrap() == '1' {
        on.push(x);
      } else {
        off.push(x);
      }
    }
    if off.len() <= on.len() || on.len() == 0 {
      r = off.iter().map(|x| x.to_string()).collect();
    } else {
      r = on.iter().map(|x| x.to_string()).collect();
    }
  }
  let co2 = i64::from_str_radix(&*r[0], 2).unwrap();

  println!("{} * {} = {}", o2, co2, o2 * co2);
}
