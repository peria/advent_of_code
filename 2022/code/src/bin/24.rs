use std::collections::VecDeque;
use std::io::BufRead;

fn main() {
    let map = get_input();

    println!("{}", shortest_goal(&map));
    println!("{}", once_back_to_start(&map));
}

fn shortest_goal(map: &Vec<Vec<char>>) -> i64 {
    let mut dp = prepare_dp(map);
    let n = dp.len();
    let m = dp[0].len();
    solve_dp(&mut dp, (0, 0), (n - 1, m - 1), 0) as i64 + 1
}

fn once_back_to_start(map: &Vec<Vec<char>>) -> i64 {
    let mut dp = prepare_dp(map);
    let n = dp.len();
    let m = dp[0].len();

    let s2g = solve_dp(&mut dp, (0, 0), (n - 1, m - 1), 0) + 1;
    reset_dp(&mut dp);
    // debug_dp(&dp);
    let g2s = solve_dp(&mut dp, (n - 1, m - 1), (0, 0), s2g + 1) + 1;
    reset_dp(&mut dp);
    eprintln!("Run: {} {}", s2g, g2s);
    solve_dp(&mut dp, (0, 0), (n - 1, m - 1), g2s + 1) as i64 + 1
}

fn prepare_dp(map: &Vec<Vec<char>>) -> Vec<Vec<Vec<Option<i64>>>> {
    let n = map.len() - 2;
    let m = map[0].len() - 2;
    let l = lcm(n, m);
    let mut dp = vec![vec![vec![Some(-1); l]; m]; n];
    for i in 0..n {
        for j in 0..m {
            let dir = &map[i + 1][j + 1];
            if dir == &'.' {
                continue;
            }
            let (dr, dc) = match dir {
                '^' => (-1, 0),
                '>' => (0, 1),
                'v' => (1, 0),
                '<' => (0, -1),
                _ => (0, 0),
            };
            let (mut r, mut c) = (i as i64, j as i64);
            for k in 0..l {
                dp[r as usize][c as usize][k] = None;
                r = (r + dr + n as i64) % n as i64;
                c = (c + dc + m as i64) % m as i64;
            }
        }
    }
    dp
}

fn solve_dp(
    dp: &mut Vec<Vec<Vec<Option<i64>>>>,
    (sr, sc): (usize, usize),
    (gr, gc): (usize, usize),
    st: usize,
) -> usize {
    let n = dp.len();
    let m = dp[0].len();
    let l = dp[0][0].len();

    // Init
    let mut q = VecDeque::new();
    for i in 0..l {
        if let Some(_t) = dp[sr][sc][i] {
            let mut t = i;
            while t < st {
                t += l;
            }
            dp[sr][sc][i] = Some(t as i64);
            q.push_back((sr as i64, sc as i64, t));
        }
    }
    while let Some((r, c, t)) = q.pop_front() {
        let t = t + 1;
        let nt = t % l;
        for (dr, dc) in [(0, 0), (0, -1), (0, 1), (-1, 0), (1, 0)] {
            let (nr, nc) = ((r + dr), (c + dc));
            if nr < 0 || nr as usize >= n || nc < 0 || nc as usize >= m {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if let Some(v) = dp[nr][nc][nt] {
                if v < st as i64 || v > t as i64 {
                    dp[nr][nc][nt] = Some(t as i64);
                    q.push_back((nr as i64, nc as i64, t));
                }
            }
        }
    }
    let mut r = 100000000;
    for k in 0..l {
        if let Some(x) = dp[gr][gc][k] {
            if x > 0 {
                r = r.min(x);
            }
        }
    }
    r as usize
}

fn reset_dp(dp: &mut Vec<Vec<Vec<Option<i64>>>>) {
    let n = dp.len();
    let m = dp[0].len();
    let l = dp[0][0].len();
    for i in 0..n {
        for j in 0..m {
            for k in 0..l {
                dp[i][j][k] = match dp[i][j][k] {
                    Some(_v) => Some(-1),
                    None => None,
                };
            }
        }
    }
}

#[allow(dead_code)]
fn debug_dp(dp: &Vec<Vec<Vec<Option<i64>>>>) {
    let n = dp.len();
    let m = dp[0].len();
    let l = dp[0][0].len();
    for k in 0..usize::min(l, 260) {
        eprintln!("Age: {}", k);
        for i in 0..n {
            for j in 0..m {
                match dp[i][j][k] {
                    Some(x) => {
                        if x < 0 {
                            eprint!(" ...");
                        } else {
                            eprint!(" {:3}", x);
                        }
                    }
                    None => {
                        eprint!(" xxx");
                    }
                }
            }
            eprintln!("");
        }
    }
}

#[allow(unused_assignments)]
fn lcm(x: usize, y: usize) -> usize {
    let (mut a, mut b, mut g) = (x, y, x % y);
    while g > 0 {
        a = b;
        b = g;
        g = a % b;
    }
    let g = b;

    x / g * y
}

fn get_input() -> Vec<Vec<char>> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap().chars().collect())
        .collect()
}
