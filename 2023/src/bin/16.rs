use std::io::Read;

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().read_to_string(&mut buf);
    let mut floor = Floor::from(&buf as &str);

    let val = floor.simulate_light(0, 0, Direction::Right);
    println!("{}", val);
    let val = floor.simulate_light_from_all();
    println!("{}", val);
}

struct Floor {
    mirrors: Vec<Vec<char>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right = 0,
    Up = 1,
    Down = 2,
    Left = 3,
}

impl Floor {
    fn simulate_light(&mut self, r: i64, c: i64, d: Direction) -> usize {
        let h = self.mirrors.len();
        let w = self.mirrors[0].len();
        let mut visited = vec![vec![vec![false; 4]; w]; h];
        let mut q = vec![(r, c, d)];
        while !q.is_empty() {
            let (r, c, d) = q.pop().unwrap();
            if r < 0 || c < 0 || r >= h as i64 || c >= w as i64 {
                continue;
            }
            if visited[r as usize][c as usize][d as usize] {
                continue;
            }
            visited[r as usize][c as usize][d as usize] = true;

            let next_dirs = match self.mirrors[r as usize][c as usize] {
                '/' => match d {
                    Direction::Right => vec![Direction::Up],
                    Direction::Up => vec![Direction::Right],
                    Direction::Down => vec![Direction::Left],
                    Direction::Left => vec![Direction::Down],
                },
                '\\' => match d {
                    Direction::Right => vec![Direction::Down],
                    Direction::Up => vec![Direction::Left],
                    Direction::Down => vec![Direction::Right],
                    Direction::Left => vec![Direction::Up],
                },
                '-' => match d {
                    Direction::Right | Direction::Left => vec![d],
                    Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
                },
                '|' => match d {
                    Direction::Right | Direction::Left => vec![Direction::Up, Direction::Down],
                    Direction::Up | Direction::Down => vec![d],
                },
                '.' => vec![d],
                _ => todo!("unknown mirror"),
            };

            for nd in next_dirs.iter() {
                let (r, c) = match nd {
                    Direction::Right => (r, c + 1),
                    Direction::Up => (r - 1, c),
                    Direction::Down => (r + 1, c),
                    Direction::Left => (r, c - 1),
                };
                q.push((r, c, *nd));
            }
        }

        visited
            .iter()
            .map(|r| r.iter().filter(|c| c.iter().any(|f| *f)).count())
            .sum()
    }

    fn simulate_light_from_all(&mut self) -> usize {
        let h = self.mirrors.len();
        let w = self.mirrors[0].len();
        let v = (0..h)
            .map(|r| {
                self.simulate_light(r as i64, 0, Direction::Right)
                    .max(self.simulate_light(r as i64, w as i64 - 1, Direction::Left))
            })
            .max()
            .unwrap();
        let v = (0..w)
            .map(|c| {
                self.simulate_light(0, c as i64, Direction::Down)
                    .max(self.simulate_light(h as i64 - 1, c as i64, Direction::Up))
            })
            .max()
            .unwrap()
            .max(v);
        v
    }
}

impl From<&str> for Floor {
    fn from(s: &str) -> Self {
        let floor = s.lines().map(|l| l.chars().collect()).collect();
        Self { mirrors: floor }
    }
}
