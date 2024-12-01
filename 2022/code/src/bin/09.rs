use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let heads = input();
    let heads = simulate_heads(&heads);

    println!("{}", count_tail_visits(&heads));
    println!("{}", count_9_visits(&heads));
}

fn input() -> Vec<(char, i32)> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|x| {
            let y = x.unwrap();
            let xs: Vec<&str> = y.split(' ').collect();
            (
                xs[0].chars().nth(0).unwrap(),
                xs[1].to_string().parse::<i32>().unwrap(),
            )
        })
        .collect()
}

fn simulate_heads(operations: &Vec<(char, i32)>) -> Vec<(i32, i32)> {
    let head = (0, 0);
    let mut heads = Vec::new();
    heads.push(head);
    for (d, n) in operations {
        for _ in 0..*n {
            let (hx, hy) = heads.last().unwrap();
            let head = match d {
                'U' => (*hx, *hy + 1),
                'D' => (*hx, *hy - 1),
                'L' => (*hx - 1, *hy),
                'R' => (*hx + 1, *hy),
                _ => (*hx, *hy),
            };
            heads.push(head);
        }
    }
    heads
}

fn count_tail_visits(heads: &Vec<(i32, i32)>) -> usize {
    let mut tail = (0, 0);
    let mut tails = HashSet::new();
    tails.insert(tail);
    for head in heads {
        tail = update_tail(&head, &tail);
        tails.insert(tail);
    }
    tails.len()
}

fn count_9_visits(heads: &Vec<(i32, i32)>) -> usize {
    let mut knots = vec![(0, 0); 10];
    let mut tails = HashSet::new();
    tails.insert(knots.last().unwrap().clone());
    for head in heads {
        knots[0] = *head;
        for i in 0..(knots.len() - 1) {
            knots[i + 1] = update_tail(&knots[i], &knots[i + 1]);
        }
        tails.insert(*knots.last().unwrap());
    }
    tails.len()
}

fn update_tail(h: &(i32, i32), t: &(i32, i32)) -> (i32, i32) {
    let (hx, hy) = h;
    let (tx, ty) = t;
    if i32::abs(hx - tx) <= 1 && i32::abs(hy - ty) <= 1 {
        return *t;
    }
    let mut dx = 0;
    let mut dy = 0;
    if hx == tx {
        dy = match hy - ty {
            2 => 1,
            -2 => -1,
            _ => 0,
        };
    } else if hy == ty {
        dx = match hx - tx {
            2 => 1,
            -2 => -1,
            _ => 0,
        }
    } else {
        dx = if hx > tx { 1 } else { -1 };
        dy = if hy > ty { 1 } else { -1 };
    }
    (tx + dx, ty + dy)
}
