use std::{collections::VecDeque, io::Read};

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().lock().read_to_string(&mut buf);
    let maze = parse_maze(&buf as &str);
    println!("{}", solve1(&maze, 100));
    println!("{}", solve2(&maze, 100));
}

fn parse_maze(buf: &str) -> Vec<Vec<char>> {
    buf.trim()
        .split('\n')
        .map(|x| x.chars().collect())
        .collect()
}

fn solve1(maze: &Vec<Vec<char>>, save: usize) -> i64 {
    solve(maze, save, 2)
}

fn solve2(maze: &Vec<Vec<char>>, save: usize) -> i64 {
    solve(maze, save, 20)
}

fn solve(maze: &Vec<Vec<char>>, save: usize, pause: usize) -> i64 {
    const DIFFS: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let width = maze[0].len();
    let height = maze.len();

    let mut steps = vec![vec![width * height; width]; height];
    let (sr, sc) = find_position(maze, 'S');
    steps[sr][sc] = 0;
    let mut q = VecDeque::new();
    let mut path = Vec::new();
    q.push_back((sr, sc));
    path.push((sr, sc));
    while !q.is_empty() {
        let (r0, c0) = q.pop_front().unwrap();
        let s = steps[r0][c0] + 1;
        for d in DIFFS.iter() {
            let r = r0 as i64 + d.0;
            let c = c0 as i64 + d.1;
            if r < 0 || c < 0 {
                continue;
            }
            let r = r as usize;
            let c = c as usize;
            if r >= height || c >= width || maze[r][c] == '#' || steps[r][c] <= s {
                continue;
            }
            steps[r][c] = s;
            q.push_back((r, c));
            path.push((r, c));
        }
    }

    let mut count = 0;
    for (i, &(r0, c0)) in path.iter().enumerate() {
        for &(r1, c1) in path[(i + 1)..].iter() {
            let diff = r0.max(r1) - r0.min(r1) + c0.max(c1) - c0.min(c1);
            if diff > pause {
                continue;
            }

            let s0 = steps[r0][c0];
            let s1 = steps[r1][c1];
            if s0 >= s1 {
                continue;
            }
            if s1 - s0 >= save + diff {
                count += 1;
            }
        }
    }
    count
}

fn find_position(maze: &Vec<Vec<char>>, target: char) -> (usize, usize) {
    for (r, row) in maze.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            if col == &target {
                return (r, c);
            }
        }
    }

    (maze.len(), maze[0].len())
}

#[cfg(test)]
mod test {
    use crate::parse_maze;
    use crate::solve1;
    use crate::solve2;

    const SAMPLE_INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_solve1() {
        let maze = parse_maze(SAMPLE_INPUT);
        let actual = solve1(&maze, 20);
        let expect = 5;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_solve2() {
        let maze = parse_maze(SAMPLE_INPUT);
        let actual = solve2(&maze, 70);
        let expect = 41;
        assert_eq!(expect, actual);
    }
}
