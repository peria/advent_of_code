use std::{collections::HashSet, io::BufRead};

fn main() {
    let maze = input();
    println!("{}", solve1(&maze));
    println!("{}", solve2(&maze));
}

fn input() -> Maze {
    let mut map = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.ok().unwrap();
        map.push(line);
    }

    Maze::parse_str(&map)
}

fn solve1(maze: &Maze) -> i64 {
    let visited = maze.solve().unwrap();

    visited
        .iter()
        .map(|v| (v.0, v.1))
        .collect::<HashSet<_>>()
        .len() as i64
}

fn solve2(maze: &Maze) -> i64 {
    let history = maze.solve().unwrap();

    let mut maze = maze.clone();
    let mut blocks = HashSet::new();
    for block in history.iter() {
        if block.0 == maze.guard.y && block.1 == maze.guard.x {
            continue;
        }
        maze.put_block(block.0, block.1);
        if maze.solve().is_none() {
            blocks.insert((block.0, block.1));
        }
        maze.remove_block(block.0, block.1);
    }
    blocks.len() as i64
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    EMPTY,
    WALL,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

#[derive(Debug, Clone)]
struct Maze {
    map: Vec<Vec<Cell>>,
    guard: Guard,
}

impl Maze {
    fn get_width(&self) -> usize {
        self.map[0].len()
    }

    fn get_height(&self) -> usize {
        self.map.len()
    }

    fn parse_str(src: &Vec<String>) -> Self {
        let height = src.len();
        let width = src[0].len();

        let mut guard = None;
        let mut map = vec![vec![Cell::EMPTY; width]; height];
        for (y, row) in src.iter().enumerate() {
            for (x, cell) in row.char_indices() {
                map[y][x] = match cell {
                    '.' => Cell::EMPTY,
                    '#' => Cell::WALL,
                    '^' => {
                        guard = Some(Guard {
                            y,
                            x,
                            direction: Direction::UP,
                        });
                        Cell::EMPTY
                    }
                    _ => Cell::EMPTY,
                }
            }
        }

        Maze {
            map,
            guard: guard.unwrap(),
        }
    }

    fn solve(&self) -> Option<Vec<(usize, usize, Direction)>> {
        let width = self.get_width();
        let height = self.get_height();
        let mut guard = self.guard.clone();

        let mut history = Vec::new();
        loop {
            history.push((guard.y, guard.x, guard.direction));
            if (guard.direction == Direction::DOWN && guard.y == height - 1)
                || (guard.direction == Direction::LEFT && guard.x == 0)
                || (guard.direction == Direction::RIGHT && guard.x == width - 1)
                || (guard.direction == Direction::UP && guard.y == 0)
            {
                return Some(history);
            }

            let (ny, nx) = match guard.direction {
                Direction::DOWN => (guard.y + 1, guard.x),
                Direction::LEFT => (guard.y, guard.x - 1),
                Direction::RIGHT => (guard.y, guard.x + 1),
                Direction::UP => (guard.y - 1, guard.x),
            };
            match self.map[ny][nx] {
                Cell::EMPTY => {
                    guard.y = ny;
                    guard.x = nx;
                }
                Cell::WALL => {
                    guard.direction = match guard.direction {
                        Direction::DOWN => Direction::LEFT,
                        Direction::LEFT => Direction::UP,
                        Direction::RIGHT => Direction::DOWN,
                        Direction::UP => Direction::RIGHT,
                    };
                }
            };
            if history.contains(&(ny, nx, guard.direction)) {
                // Find a loop.
                return None;
            }
        }
    }

    fn put_block(&mut self, y: usize, x: usize) {
        self.map[y][x] = Cell::WALL;
    }

    fn remove_block(&mut self, y: usize, x: usize) {
        self.map[y][x] = Cell::EMPTY;
    }
}

#[derive(Debug, Clone, Copy)]
struct Guard {
    y: usize,
    x: usize,
    direction: Direction,
}
