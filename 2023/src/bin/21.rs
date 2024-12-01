use std::{collections::VecDeque, io::BufRead};

use itertools::Itertools;

fn main() {
    let map: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect();

    let val = get_plots(&map, 6);
    println!("{}", val);
    let val = get_plots(&map, 64);
    eprintln!("{}", val);
}

fn get_plots(map: &Vec<Vec<char>>, n: usize) -> i64 {
    let h = map.len();
    let w = map[0].len();

    let (sr, sc) = find_start(map);
    eprintln!("{} {}", sr, sc);

    let mut q = VecDeque::new();
    q.push_back((sr, sc));
    let mut dp = vec![vec![None; w]; h];
    dp[sr][sc] = Some(0);
    while !q.is_empty() {
        let (r, c) = q.pop_front().unwrap();
        let m = dp[r][c].unwrap() + 1;
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nr = r as i64 + dr;
            let nc = c as i64 + dc;
            if nr < 0 || nc < 0 || nr >= h as i64 || nc >= w as i64 {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if map[nr][nc] == '#' || dp[nr][nc].is_some() {
                continue;
            }
            dp[nr][nc] = Some(m);
            q.push_back((nr, nc));
        }
    }
    dp.iter()
        .map(|r| {
            r.iter()
                .filter(|&x| {
                    let k = if let Some(k) = x { *k } else { 999999999 };
                    k <= n && k % 2 == n % 2
                })
                .count()
        })
        .sum::<usize>() as i64
}

fn find_start(map: &Vec<Vec<char>>) -> (usize, usize) {
    for (r, row) in map.iter().enumerate() {
        if let Some(p) = row.iter().find_position(|&c| c == &'S') {
            return (r, p.0);
        }
    }
    (0, 0)
}
