use std::collections::VecDeque;
use std::io::BufRead;

fn main() {
    let heights = get_input();
    let e = get_goal(&heights);
    let dist = make_dist_map(&heights, e);
    let mut map = Vec::new();
    for (x, y) in heights.iter().zip(dist.iter()) {
        for (&a, &b) in x.iter().zip(y.iter()) {
            map.push((a, b));
        }
    }

    println!("{}", get_shortest(&map, &'S'));
    println!("{}", get_shortest(&map, &'a'));
}

fn get_input() -> Vec<Vec<char>> {
    let mut hs = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        hs.push(line.chars().collect());
    }
    hs
}

fn get_goal(map: &Vec<Vec<char>>) -> (usize, usize) {
    for (i, row) in map.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'E' {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn get_shortest(map: &Vec<(char, usize)>, c: &char) -> usize {
    let (_ch, dist) = map
        .iter()
        .filter(|(x, _d)| x == c)
        .min_by_key(|(_x, d)| d)
        .unwrap();
    *dist
}

fn make_dist_map(map: &Vec<Vec<char>>, (si, sj): (usize, usize)) -> Vec<Vec<usize>> {
    let h = map.len();
    let w = map[0].len();
    let mut dp = vec![vec![h * w * 2; w]; h];
    let mut q = VecDeque::new();
    dp[si][sj] = 0;
    q.push_back((si, sj));

    while let Some((i, j)) = q.pop_front() {
        let steps = dp[i][j] + 1;
        let t = match map[i][j] {
            'S' => 'a',
            'E' => 'z',
            _ => map[i][j],
        };
        for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let (ii, jj) = (i as i32 + di, j as i32 + dj);
            if ii < 0 || jj < 0 || ii >= h as i32 || jj >= w as i32 {
                continue;
            }
            let (ii, jj) = (ii as usize, jj as usize);
            let tt = match map[ii][jj] {
                'S' => 'a',
                'E' => 'z',
                _ => map[ii][jj],
            };
            if t as u32 > tt as u32 + 1 {
                continue;
            }
            if dp[ii][jj] > steps {
                dp[ii][jj] = steps;
                q.push_back((ii, jj));
            }
        }
    }
    dp
}
