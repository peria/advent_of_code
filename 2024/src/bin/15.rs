use std::io::Read;

fn main() {
    let house = input();
    println!("{}", solve1(&house));
    println!("{}", solve2(&house));
}

fn input() -> House {
    let mut buf = String::new();
    let _ = std::io::stdin().lock().read_to_string(&mut buf);

    House::from(&buf as &str)
}

fn solve1(house: &House) -> i64 {
    let mut house = house.clone();
    house.simulate();
    house.sum_coordinates()
}

fn solve2(house: &House) -> i64 {
    let mut house = house.clone();
    house.extend();
    house.simulate();
    house.sum_coordinates()
}

#[derive(Clone, Debug)]
struct House {
    map: Vec<Vec<char>>,
    robot: Robot,
}

impl House {
    fn extend(&mut self) {
        self.robot.pos.1 *= 2;
        for line in self.map.iter_mut() {
            let x: String = line
                .iter()
                .map(|x| match x {
                    '#' => "##",
                    '.' => "..",
                    'O' => "[]",
                    '@' => "@.",
                    _ => "XX",
                })
                .collect();
            *line = x.chars().collect();
        }
    }

    fn simulate(&mut self) {
        let mut pos = self.robot.pos.clone();
        let action = self.robot.action.clone();
        for act in action.iter() {
            if self.check_one_step(&pos, act) {
                pos = self.force_one_step(&pos, act);
                self.robot.pos = pos;
                // eprintln!("Move: {}", act);
                // self.dump();
            }
        }
    }

    fn sum_coordinates(&self) -> i64 {
        let mut s = 0;
        for (i, row) in self.map.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c == 'O' || c == '[' {
                    s += i as i64 * 100 + j as i64;
                }
            }
        }
        s
    }

    fn force_one_step(&mut self, pos: &(usize, usize), act: &char) -> (usize, usize) {
        let next = match act {
            &'^' => (pos.0 - 1, pos.1),
            &'>' => (pos.0, pos.1 + 1),
            &'<' => (pos.0, pos.1 - 1),
            &'v' => (pos.0 + 1, pos.1),
            _ => (0, 0),
        };

        let current = self.map[pos.0][pos.1];
        if current == 'O' || current == '@' || act == &'>' || act == &'<' {
            if self.map[next.0][next.1] != '.' {
                self.force_one_step(&next, act);
            }
            self.map[next.0][next.1] = current;
            self.map[pos.0][pos.1] = '.';

            return next;
        }

        if current == '[' {
            let next2 = (next.0, next.1 + 1);
            if self.map[next.0][next.1] != '.' {
                self.force_one_step(&next, act);
            }
            if self.map[next2.0][next2.1] != '.' {
                self.force_one_step(&next2, act);
            }
            self.map[next.0][next.1] = current;
            self.map[next2.0][next2.1] = ']';
            self.map[pos.0][pos.1] = '.';
            self.map[pos.0][pos.1 + 1] = '.';
            return next;
        };

        if current == ']' {
            let next2 = (next.0, next.1 - 1);
            if self.map[next.0][next.1] != '.' {
                self.force_one_step(&next, act);
            }
            if self.map[next2.0][next2.1] != '.' {
                self.force_one_step(&next2, act);
            }
            self.map[next.0][next.1] = current;
            self.map[next2.0][next2.1] = '[';
            self.map[pos.0][pos.1] = '.';
            self.map[pos.0][pos.1 - 1] = '.';
            return next;
        };

        next
    }

    fn check_one_step(&mut self, pos: &(usize, usize), act: &char) -> bool {
        let next = match act {
            &'^' => (pos.0 - 1, pos.1),
            &'>' => (pos.0, pos.1 + 1),
            &'<' => (pos.0, pos.1 - 1),
            &'v' => (pos.0 + 1, pos.1),
            _ => (0, 0),
        };

        let current = &self.map[pos.0][pos.1];
        if current == &'O' || current == &'@' || act == &'>' || act == &'<' {
            if self.map[next.0][next.1] == '.' {
                return true;
            }
            if self.map[next.0][next.1] == '#' {
                return false;
            }
            return self.check_one_step(&next, act);
        }

        if current == &'[' {
            let next2 = (next.0, next.1 + 1);
            if self.map[next.0][next.1] == '.' && self.map[next2.0][next2.1] == '.' {
                return true;
            }
            if self.map[next.0][next.1] == '#' || self.map[next2.0][next2.1] == '#' {
                return false;
            }
            return self.check_one_step(&next, act) && self.check_one_step(&next2, act);
        };

        if current == &']' {
            let next2 = (next.0, next.1 - 1);
            if self.map[next.0][next.1] == '.' && self.map[next2.0][next2.1] == '.' {
                return true;
            }
            if self.map[next.0][next.1] == '#' || self.map[next2.0][next2.1] == '#' {
                return false;
            }
            return self.check_one_step(&next, act) && self.check_one_step(&next2, act);
        };

        current == &'.'
    }

    #[allow(dead_code)]
    fn dump(&self) {
        for l in self.map.iter() {
            let line: String = l.iter().collect();
            eprintln!("{}", line);
        }
        eprintln!("{} {}", self.robot.pos.0, self.robot.pos.1);
    }
}

impl From<&str> for House {
    fn from(value: &str) -> Self {
        let values: Vec<_> = value.trim().split("\n\n").collect();
        let map = values[0].split('\n').map(|x| x.chars().collect()).collect();
        let robot = Robot::new(&map, values[1]);

        House { map, robot }
    }
}

#[derive(Clone, Debug)]
struct Robot {
    pos: (usize, usize),
    action: Vec<char>,
}

impl Robot {
    fn new(map: &Vec<Vec<char>>, action: &str) -> Self {
        let robot_line = map
            .iter()
            .enumerate()
            .find(|(_, l)| l.contains(&'@'))
            .unwrap();
        let r = robot_line.0;
        let line = robot_line.1;
        let c = line.iter().position(|&x| x == '@').unwrap();

        let action = action
            .trim()
            .split('\n')
            .collect::<String>()
            .chars()
            .collect();
        Robot {
            pos: (r, c),
            action,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::solve1;
    use crate::solve2;
    use crate::House;

    const SAMPLE_SMALL: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

    const SAMPLE_LARGE: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    #[test]
    fn test_solve1_small() {
        let house = House::from(SAMPLE_SMALL);
        let actual = solve1(&house);
        let expect = 2028;
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_solve1_large() {
        let house = House::from(SAMPLE_LARGE);
        let actual = solve1(&house);
        let expect = 10092;
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_solve2() {
        let house = House::from(SAMPLE_LARGE);
        let actual = solve2(&house);
        let expect = 9021;
        assert_eq!(actual, expect);
    }
}
