const XMIN: i32 = 128;
const XMAX: i32 = 160;
const YMIN: i32 = -142;
const YMAX: i32 = -88;

fn does_pass_target(v: &(i32, i32)) -> bool {
  let mut x = 0;
  let mut y = 0;
  let (mut vx, mut vy) = v;
  while y >= YMIN {
    x += vx;
    y += vy;
    vx = if vx > 0 {
      vx - 1
    } else if vx < 0 {
      vx + 1
    } else {
      vx
    };
    vy -= 1;
    if XMIN <= x && x <= XMAX && YMIN <= y && y <= YMAX {
      return true;
    }
  }
  false
}

#[allow(dead_code)]
pub fn main() {
  let mut velocities = Vec::new();
  for vx in 1..(XMAX + 1) {
    for vy in YMIN..-YMIN + 1 {
      if does_pass_target(&(vx, vy)) {
        velocities.push((vx, vy));
      }
    }
  }
  println!("{}", velocities.len());
}
