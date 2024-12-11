use std::{collections::VecDeque, io::BufRead};

fn main() {
    let map = input();
    println!("{}", solve1(&map));
    println!("{}", solve2(&map));
}

fn input() -> TopoMap {
    let mut lines = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.ok().unwrap();
        lines.push(line);
    }

    TopoMap::from(&lines)
}

fn solve1(map: &TopoMap) -> i64 {
    let trails = map.find_starts();
    let mut score = 0;
    for pos in trails.iter() {
        let heads = map.find_heads_from(pos);
        score += heads.len();
    }
    score as i64
}

fn solve2(map: &TopoMap) -> i64 {
    map.count_paths() as i64
}

struct TopoMap {
    heights: Vec<Vec<i32>>,
}

impl TopoMap {
    fn find_starts(&self) -> Vec<(usize, usize)> {
        let mut trails = Vec::new();

        for (i, row) in self.heights.iter().enumerate() {
            for (j, h) in row.iter().enumerate() {
                if h == &0 {
                    trails.push((i, j));
                }
            }
        }

        trails
    }

    const DIFFS: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    fn find_heads_from(&self, &(r0, c0): &(usize, usize)) -> Vec<(usize, usize)> {
        let width = self.heights[0].len();
        let height = self.heights.len();

        let mut heads = Vec::new();
        let mut reachable = vec![vec![false; width]; height];
        reachable[r0][c0] = true;
        let mut q = VecDeque::new();
        q.push_back((r0, c0));
        while !q.is_empty() {
            let (r0, c0) = q.pop_front().unwrap();
            let h = self.heights[r0][c0] + 1;
            if h == 10 {
                heads.push((r0, c0));
                continue;
            }
            for &(dr, dc) in Self::DIFFS.iter() {
                let r = r0 as isize + dr;
                let c = c0 as isize + dc;
                if r < 0 || c < 0 {
                    continue;
                }
                let r = r as usize;
                let c = c as usize;
                if r >= height || c >= width {
                    continue;
                }
                if reachable[r][c] {
                    continue;
                }
                if self.heights[r][c] == h {
                    reachable[r][c] = true;
                    q.push_back((r, c));
                }
            }
        }

        heads
    }

    fn count_paths(&self) -> usize {
        let width = self.heights[0].len();
        let height = self.heights.len();

        let mut paths = vec![vec![0; width]; height];
        let places = self.get_level(0);
        places.iter().for_each(|&(r, c)| paths[r][c] = 1);
        for h in 0..9 {
            let places = self.get_level(h);
            for &(r0, c0) in places.iter() {
                for &(dr, dc) in Self::DIFFS.iter() {
                    let r = r0 as isize + dr;
                    let c = c0 as isize + dc;
                    if r < 0 || c < 0 {
                        continue;
                    }
                    let r = r as usize;
                    let c = c as usize;
                    if r >= height || c >= width {
                        continue;
                    }
                    if self.heights[r][c] == h + 1 {
                        paths[r][c] += paths[r0][c0];
                    }
                }
            }
        }

        let tops = self.get_level(9);
        tops.iter().map(|&(r, c)| paths[r][c]).sum()
    }

    fn get_level(&self, level: i32) -> Vec<(usize, usize)> {
        let mut places = Vec::new();
        for (i, row) in self.heights.iter().enumerate() {
            for (j, h) in row.iter().enumerate() {
                if h == &level {
                    places.push((i, j));
                }
            }
        }
        places
    }
}

impl From<&Vec<String>> for TopoMap {
    fn from(value: &Vec<String>) -> Self {
        let width = value[0].len();
        let height = value.len();

        let mut map = vec![vec![0; width]; height];
        for (i, row) in value.iter().enumerate() {
            for (j, c) in row.char_indices() {
                let h = c as u8 - '0' as u8;
                map[i][j] = h as i32;
            }
        }

        TopoMap { heights: map }
    }
}

impl From<&str> for TopoMap {
    fn from(value: &str) -> Self {
        let mut map = Vec::new();
        for line in value.lines() {
            let line = line.to_string();
            map.push(line);
        }

        TopoMap::from(&map)
    }
}

#[cfg(test)]
mod test {
    use crate::solve1;
    use crate::solve2;
    use crate::TopoMap;

    const TEST_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    #[test]
    fn test_solve1() {
        let map = TopoMap::from(TEST_INPUT);
        let actual = solve1(&map);
        let expect = 36;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_solve2() {
        let map = TopoMap::from(TEST_INPUT);
        let actual = solve2(&map);
        let expect = 81;
        assert_eq!(expect, actual);
    }
}
