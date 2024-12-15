use std::{
    collections::{HashSet, VecDeque},
    io::Read,
};

fn main() {
    let farm = input();
    println!("{}", solve1(&farm));
    println!("{}", solve2(&farm));
}

fn input() -> Farm {
    let mut buf = String::new();
    let _ = std::io::stdin().lock().read_to_string(&mut buf);
    Farm::from(&buf as &str)
}

fn solve1(farm: &Farm) -> i64 {
    let regions = farm.parse();
    regions.iter().map(|x| x.price()).sum()
}

#[allow(unused_variables)]
fn solve2(farm: &Farm) -> i64 {
    0
}

#[derive(Debug, Clone)]
struct Farm {
    map: Vec<Vec<char>>,
}

impl Farm {
    fn parse(&self) -> Vec<Region> {
        let width = self.map[0].len();
        let height = self.map.len();

        let mut regions = Vec::new();
        let mut map = self.map.clone();
        for i in 0..height {
            for j in 0..width {
                if let Some(region) = Self::get_region_from(&mut map, i, j) {
                    regions.push(region);
                }
            }
        }
        regions
    }

    fn get_region_from(map: &mut Vec<Vec<char>>, r0: usize, c0: usize) -> Option<Region> {
        const DIFFS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let width = map[0].len() as isize;
        let height = map.len() as isize;

        let label = map[r0][c0];
        if label == '.' {
            return None;
        }

        let mut pos = HashSet::new();
        pos.insert((r0 as isize, c0 as isize));
        let mut q = VecDeque::new();
        q.push_back((r0, c0));
        while !q.is_empty() {
            let (r0, c0) = q.pop_front().unwrap();
            for &(dr, dc) in DIFFS.iter() {
                let r = r0 as isize + dr;
                let c = c0 as isize + dc;
                if r < 0 || c < 0 || r >= height || c >= width {
                    continue;
                }
                let (r, c) = (r as usize, c as usize);
                if map[r][c] != label {
                    continue;
                }
                map[r][c] = '.';
                pos.insert((r as isize, c as isize));
                q.push_back((r, c));
            }
        }

        let area = pos.len();
        let mut sides = area * 4;
        for &(r0, c0) in pos.iter() {
            for &(dr, dc) in DIFFS.iter() {
                let (r, c) = (r0 as isize + dr, c0 as isize + dc);
                if pos.contains(&(r, c)) {
                    sides -= 1;
                }
            }
        }

        let region = Region {
            _label: label,
            area,
            sides,
        };
        Some(region)
    }
}

impl From<&str> for Farm {
    fn from(value: &str) -> Self {
        let map = value
            .trim()
            .split('\n')
            .map(|x| x.chars().collect())
            .collect();
        Farm { map }
    }
}

#[derive(Debug, Copy, Clone)]
struct Region {
    _label: char,
    area: usize,
    sides: usize,
}

impl Region {
    fn price(&self) -> i64 {
        self.area as i64 * self.sides as i64
    }
}

#[cfg(test)]
mod test {
    use crate::solve1;
    use crate::solve2;
    use crate::Farm;

    const SAMPLE_INPUT1: &str = "AAAA
BBCD
BBCC
EEEC";

    const SAMPLE_INPUT2: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_solve1_1() {
        let farm = Farm::from(SAMPLE_INPUT1);
        let actual = solve1(&farm);
        let expect = 140;
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_solve1_2() {
        let farm = Farm::from(SAMPLE_INPUT2);
        let actual = solve1(&farm);
        let expect = 1930;
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_solve2_1() {
        let farm = Farm::from(SAMPLE_INPUT1);
        let actual = solve2(&farm);
        let expect = 236;
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_solve2_2() {
        let farm = Farm::from(SAMPLE_INPUT2);
        let actual = solve2(&farm);
        let expect = 1206;
        assert_eq!(actual, expect);
    }
}
