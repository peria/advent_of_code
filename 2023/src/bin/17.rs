use std::{collections::BTreeSet, io::Read};

fn main() {
    let pool = LavaPool::from_stdin();

    let val = pool.crucible(1, 3);
    println!("{}", val);
    let val = pool.crucible(4, 10);
    println!("{}", val);
}

struct LavaPool {
    map: Vec<Vec<i64>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North = 0,
    West = 1,
    South = 2,
    East = 3,
}

impl LavaPool {
    fn from_stdin() -> LavaPool {
        let mut buf = String::new();
        let _ = std::io::stdin().read_to_string(&mut buf);
        let map = buf
            .trim()
            .split('\n')
            .map(|x| x.chars().map(|y| (y as u8 - '0' as u8) as i64).collect())
            .collect();
        LavaPool { map }
    }

    fn crucible(&self, min_blocks: usize, max_blocks: usize) -> i64 {
        let h = self.map.len();
        let w = self.map[0].len();

        let init_lost = |mut r: usize, mut c: usize, d: Direction, n: usize| -> Option<i64> {
            let mut cost = 0;
            for _ in 0..n {
                let (nr, nc) = match d {
                    Direction::North => (r as i64 - 1, c as i64),
                    Direction::West => (r as i64, c as i64 - 1),
                    Direction::South => (r as i64 + 1, c as i64),
                    Direction::East => (r as i64, c as i64 + 1),
                };
                if nr < 0 || nc < 0 || nr >= h as i64 || nc >= w as i64 {
                    return None;
                }
                (r, c) = (nr as usize, nc as usize);
                cost += self.map[r][c];
            }
            Some(cost)
        };

        let mut dp = vec![vec![vec![None; 4]; w]; h];
        let mut q = BTreeSet::new();
        for d in [Direction::East, Direction::South] {
            dp[0][0][d as usize] = Some(0);
            q.insert((0, 0, 0, d));
        }
        while !q.is_empty() {
            let (cost, r, c, d) = q.pop_first().unwrap();
            if r == h - 1 && c == w - 1 {
                return cost;
            }

            let next_dirs = match d {
                Direction::East | Direction::West => {
                    vec![Direction::North, Direction::South]
                }
                Direction::North | Direction::South => {
                    vec![Direction::East, Direction::West]
                }
            };
            for nd in next_dirs.into_iter() {
                if let Some(lost) = init_lost(r, c, nd, min_blocks) {
                    let (mut nr, mut nc) = match nd {
                        Direction::North => (r - min_blocks, c),
                        Direction::West => (r, c - min_blocks),
                        Direction::South => (r + min_blocks, c),
                        Direction::East => (r, c + min_blocks),
                    };

                    let mut ncost = cost + lost;
                    for _ in min_blocks..=max_blocks {
                        match dp[nr][nc][nd as usize] {
                            None => {
                                dp[nr][nc][nd as usize] = Some(ncost);
                                q.insert((ncost, nr, nc, nd));
                            }
                            Some(v) => {
                                if ncost < v {
                                    dp[nr][nc][nd as usize] = Some(ncost);
                                    q.insert((ncost, nr, nc, nd));
                                }
                            }
                        }

                        let (nnr, nnc) = match nd {
                            Direction::North => (nr as i64 - 1, nc as i64),
                            Direction::West => (nr as i64, nc as i64 - 1),
                            Direction::South => (nr as i64 + 1, nc as i64),
                            Direction::East => (nr as i64, nc as i64 + 1),
                        };
                        if nnr < 0 || nnc < 0 || nnr >= h as i64 || nnc >= w as i64 {
                            break;
                        }
                        (nr, nc) = (nnr as usize, nnc as usize);
                        ncost += self.map[nr][nc];
                    }
                }
            }
        }
        0
    }
}
