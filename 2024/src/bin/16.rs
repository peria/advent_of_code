use std::{collections::BTreeSet, io::Read};

fn main() {
    let maze = input();
    println!("{}", solve1(&maze));
    println!("{}", solve2(&maze));
}

fn input() -> Maze {
    let mut buf = String::new();
    let _ = std::io::stdin().lock().read_to_string(&mut buf);
    Maze::from(&buf as &str)
}

fn solve1(maze: &Maze) -> i64 {
    maze.get_minimum_cost()
}

fn solve2(house: &Maze) -> i64 {
    0
}

#[derive(Debug)]
struct Maze {
    map: Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Maze {
    fn position_of(map: &Vec<Vec<char>>, target: &char) -> (usize, usize) {
        let (i, line) = map
            .iter()
            .enumerate()
            .find(|(_, l)| l.contains(target))
            .unwrap();
        let j = line.iter().position(|x| x == target).unwrap();
        (i, j)
    }

    fn get_minimum_cost(&self) -> i64 {
        // >(east), v(south), <(west), ^(north)
        const DIFFS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        const EAST: usize = 0;

        let width = self.map[0].len();
        let height = self.map.len();

        let mut min_cost = vec![vec![vec![None; 4]; width]; height];
        min_cost[self.start.0][self.start.1][EAST] = Some(0);
        let mut q = BTreeSet::new();
        q.insert((0, self.start.0, self.start.1, EAST));

        while !q.is_empty() {
            let (cost, r0, c0, d0) = q.pop_first().unwrap();

            let mut maybe_update_cost = |ncost: i64, forward: bool, d: usize| {
                let (mut r, mut c) = (r0, c0);
                if forward {
                    r = (r as isize + DIFFS[d].0) as usize;
                    c = (c as isize + DIFFS[d].1) as usize;
                }
                if self.map[r][c] != '#' {
                    if min_cost[r][c][d].is_none() || min_cost[r][c][d].unwrap() > ncost {
                        min_cost[r][c][d] = Some(ncost);
                        q.insert((ncost, r, c, d));
                    }
                }
            };

            maybe_update_cost(cost + 1, true, d0);
            maybe_update_cost(cost + 1000, false, (d0 + 1) % 4);
            maybe_update_cost(cost + 1000, false, (d0 + 3) % 4);
        }

        min_cost[self.end.0][self.end.1]
            .iter()
            .map(|x| x.unwrap_or(std::i64::MAX))
            .min()
            .unwrap()
    }
}

impl From<&str> for Maze {
    fn from(value: &str) -> Self {
        let map: Vec<_> = value
            .trim()
            .split('\n')
            .map(|x| x.chars().collect())
            .collect();

        let start = Self::position_of(&map, &'S');
        let end = Self::position_of(&map, &'E');

        Maze { map, start, end }
    }
}

#[cfg(test)]
mod test {
    use crate::solve1;
    use crate::solve2;
    use crate::Maze;

    const SAMPLE_MAZE1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    const SAMPLE_MAZE2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

    #[test]
    fn test_solve1() {
        let maze = Maze::from(SAMPLE_MAZE1);
        let actual = solve1(&maze);
        let expect = 7036;
        assert_eq!(actual, expect);

        let maze = Maze::from(SAMPLE_MAZE2);
        let actual = solve1(&maze);
        let expect = 11048;
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_solve2() {
        let maze = Maze::from(SAMPLE_MAZE1);
        let actual = solve2(&maze);
        let expect = 0;
        assert_eq!(actual, expect);
    }
}
