use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let map = get_input();

    println!("{}", count_visible(&map));
    println!("{}", highest_scenic_score(&map));
}

fn get_input() -> Vec<Vec<i32>> {
    let mut map = Vec::new();
    for l in std::io::stdin().lock().lines() {
        let x: Vec<i32> = l
            .unwrap()
            .chars()
            .map(|x| x.to_string().parse::<i32>().unwrap())
            .collect();
        map.push(x);
    }
    map
}

fn count_visible(map: &Vec<Vec<i32>>) -> usize {
    const INIT: i32 = -1;

    let mut visible = HashSet::new();
    for (i, line) in map.iter().enumerate() {
        let mut m = INIT;
        for (j, &h) in line.iter().enumerate() {
            if h > m {
                visible.insert((i, j));
                m = h;
            }
        }
        let mut m = INIT;
        for (j, &h) in line.iter().enumerate().rev() {
            if h > m {
                visible.insert((i, j));
                m = h;
            }
        }
    }

    let mut ms = vec![INIT; map[0].len()];
    for (i, line) in map.iter().enumerate() {
        for (j, &h) in line.iter().enumerate() {
            if h > ms[j] {
                visible.insert((i, j));
                ms[j] = h;
            }
        }
    }
    let mut ms = vec![INIT; map[0].len()];
    for (i, line) in map.iter().enumerate().rev() {
        for (j, &h) in line.iter().enumerate() {
            if h > ms[j] {
                visible.insert((i, j));
                ms[j] = h;
            }
        }
    }

    visible.len()
}

fn highest_scenic_score(map: &Vec<Vec<i32>>) -> i64 {
    map.iter()
        .enumerate()
        .map(|(i, ts)| {
            ts.iter()
                .enumerate()
                .map(|(j, _t)| scenic_score(i, j, &map))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn scenic_score(r: usize, c: usize, map: &Vec<Vec<i32>>) -> i64 {
    let w = map[0].len();
    let h = map.len();
    let t = &map[r][c];

    let left = match map[r][0..c].iter().rev().position(|x| x >= t) {
        Some(x) => x + 1,
        None => c,
    } as i64;
    let right = match map[r][(c + 1)..w].iter().position(|x| x >= t) {
        Some(x) => x + 1,
        None => w - c - 1,
    } as i64;
    let up = match map[0..r].iter().rev().position(|x| x[c] >= *t) {
        Some(x) => x + 1,
        None => r,
    } as i64;
    let down = match map[(r + 1)..h].iter().position(|x| x[c] >= *t) {
        Some(x) => x + 1,
        None => h - r - 1,
    } as i64;

    // eprintln!("{} - {}*{}*{}*{}", t, left, right, up, down);
    left * right * up * down
}
