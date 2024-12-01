use std::io::BufRead;

fn main() {
    let plans: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|x| DigPlan::from(&x.unwrap() as &str))
        .collect();

    let val = dig(&plans);
    println!("{}", val);
    let plans: Vec<_> = plans
        .iter()
        .map(|p| {
            let direction = match (p.color % 8) {
                0 => 'R',
                1 => 'D',
                2 => 'L',
                3 => 'U',
                _ => 'x',
            };
            DigPlan {
                direction,
                length: p.color as i64 >> 4,
                color: 0,
            }
        })
        .collect();
    let val = dig(&plans);
    println!("{}", val);
}

fn dig(plans: &Vec<DigPlan>) -> i64 {
    let (d, l) = (plans[0].direction, plans[0].length);
    let (mut x0, mut y0) = match d {
        'U' => (0, -l),
        'L' => (-l, 0),
        'D' => (0, l),
        'R' => (l, 0),
        _ => todo!(),
    };

    let n = plans.len() as i64;
    let mut area = 0;
    for p in plans[1..].iter() {
        let (d, l) = (p.direction, p.length);
        let (x1, y1) = match d {
            'U' => (x0, y0 - l),
            'L' => (x0 - l, y0),
            'D' => (x0, y0 + l),
            'R' => (x0 + l, y0),
            _ => todo!(),
        };
        area += x0 * y1 - x1 * y0;
        (x0, y0) = (x1, y1);
    }
    let area = area.abs() / 2;
    let area = area + (plans.iter().map(|p| p.length).sum::<i64>() - n) / 2 + (n - 4) / 2 + 3;
    area
}

#[derive(Debug, Clone, Copy)]
struct DigPlan {
    direction: char,
    length: i64,
    color: u64,
}

impl From<&str> for DigPlan {
    fn from(value: &str) -> Self {
        let (direction, length, color) =
            sscanf::sscanf!(value, "{} {} (#{:x})", char, i64, u64).unwrap();
        DigPlan {
            direction,
            length,
            color,
        }
    }
}
