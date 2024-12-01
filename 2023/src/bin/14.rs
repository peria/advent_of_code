use std::{collections::HashMap, io::Read};

fn main() {
    let dish = Dish::new();

    let t = dish.tilt(&[Direction::North], 1);
    let val = t.total_load();
    println!("{}", val);
    let t = dish.tilt(
        &[
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ],
        1000000000,
    );
    let val = t.total_load();
    println!("{}", val);
}

#[derive(Clone)]
struct Dish {
    field: Vec<Vec<char>>,
}

enum Direction {
    North,
    West,
    South,
    East,
}

impl Dish {
    fn new() -> Dish {
        let mut buf = String::new();
        let _ = std::io::stdin().read_to_string(&mut buf);
        let field = buf
            .trim()
            .split('\n')
            .map(|x| x.chars().collect())
            .collect();
        Dish { field }
    }

    fn tilt(&self, directions: &[Direction], n: usize) -> Dish {
        let mut round: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut dish = self.clone();
        for k in 0..n {
            for d in directions {
                dish = dish.tilt_once(d);
            }
            if !round.contains_key(k) {
                round.insert(k, Vec::new());
            }
            round[k].push(dish.total_load());
        }
        dish
    }

    fn at(&self, r: usize, c: usize) -> char {
        self.field[r][c]
    }

    fn set(&mut self, r: usize, c: usize, x: char) {
        self.field[r][c] = x;
    }

    fn tilt_once(&self, direction: &Direction) -> Dish {
        let mut dish = self.clone();
        let h = dish.field.len();
        let w = dish.field[0].len();

        match direction {
            Direction::North => {
                for c in 0..w {
                    let mut t = 0;
                    for r in 0..h {
                        match dish.at(r, c) {
                            'O' => {
                                dish.set(r, c, '.');
                                dish.set(t, c, 'O');
                                t += 1;
                            }
                            '#' => {
                                t = r + 1;
                            }
                            _ => (),
                        }
                    }
                }
            }
            Direction::West => {
                for r in 0..h {
                    let mut t = 0;
                    for c in 0..w {
                        match dish.at(r, c) {
                            'O' => {
                                dish.set(r, c, '.');
                                dish.set(r, t, 'O');
                                t += 1;
                            }
                            '#' => {
                                t = c + 1;
                            }
                            _ => (),
                        }
                    }
                }
            }
            Direction::South => {
                for c in 0..w {
                    let mut t = h - 1;
                    for r in (0..h).rev() {
                        match dish.at(r, c) {
                            'O' => {
                                dish.set(r, c, '.');
                                dish.set(t, c, 'O');
                                t = t.max(1) - 1;
                            }
                            '#' => {
                                t = r.max(1) - 1;
                            }
                            _ => (),
                        }
                    }
                }
            }
            Direction::East => {
                for r in 0..h {
                    let mut t = w - 1;
                    for c in (0..w).rev() {
                        match dish.at(r, c) {
                            'O' => {
                                dish.set(r, c, '.');
                                dish.set(r, t, 'O');
                                t = t.max(1) - 1;
                            }
                            '#' => {
                                t = c.max(1) - 1;
                            }
                            _ => (),
                        }
                    }
                }
            }
        }

        dish
    }

    fn total_load(&self) -> usize {
        let n = self.field.len();
        let mut ret = 0;
        for (i, r) in self.field.iter().enumerate() {
            let rocks = r.iter().filter(|&x| x == &'O').count();
            ret += (n - i) * rocks;
        }
        ret
    }
}
