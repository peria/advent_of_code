use std::io::Read;

fn main() {
    let machine = input();
    println!("{}", solve1(&machine));
    println!("{}", solve2(&machine));
}

fn input() -> MachineList {
    let mut buf = String::new();
    let _ = std::io::stdin().read_to_string(&mut buf);
    MachineList::from(&buf as &str)
}

fn solve1(machines: &MachineList) -> i64 {
    machines.get_total_prize()
}

fn solve2(machines: &MachineList) -> i64 {
    const DISTANCE: i64 = 10000000000000;
    let mut machines = machines.clone();
    machines.move_locations(DISTANCE);
    machines.get_total_prize()
}

#[derive(Clone)]
struct MachineList {
    machines: Vec<Machine>,
}

impl MachineList {
    fn get_total_prize(&self) -> i64 {
        self.machines.iter().map(|x| x.possible_prize()).sum()
    }

    fn move_locations(&mut self, distance: i64) {
        self.machines.iter_mut().for_each(|x| {
            x.prize[0] += distance;
            x.prize[1] += distance;
        });
    }
}

impl From<&str> for MachineList {
    fn from(value: &str) -> Self {
        let lines: Vec<_> = value.trim().split('\n').collect();
        let mut machines = Vec::new();
        let n = lines.len();
        for i in (0..n).step_by(4) {
            let input = lines[i..(i + 3)].join("\n");
            let machine = Machine::from(&input as &str);
            machines.push(machine);
        }
        MachineList { machines }
    }
}

#[derive(Debug, Clone)]
struct Machine {
    button_a: [i64; 2],
    button_b: [i64; 2],
    prize: [i64; 2],
}

impl Machine {
    fn parse_button(line: &str) -> [i64; 2] {
        let parsed = sscanf::sscanf!(line, "Button {str}: X+{i64}, Y+{i64}").unwrap();
        [parsed.1, parsed.2]
    }

    fn parse_prize(line: &str) -> [i64; 2] {
        let parsed = sscanf::sscanf!(line, "Prize: X={i64}, Y={i64}").unwrap();
        [parsed.0, parsed.1]
    }

    fn possible_prize(&self) -> i64 {
        let a = self.button_a[0];
        let b = self.button_a[1];
        let c = self.button_b[0];
        let d = self.button_b[1];
        let x = self.prize[0];
        let y = self.prize[1];
        let det = a * d - b * c;
        if det == 0 {
            if a * y - b * x == 0 {
                // 2 buttons and the location are all linear.
                eprintln!("all linear: na * {} + nb * {} = {}", a, c, x);
                1
            } else {
                // 2 buttons are linear, but the location is not on the line.
                0
            }
        } else {
            let na = x * d - y * c;
            let nb = -x * b + y * a;
            if na % det != 0 || nb % det != 0 {
                // No solution
                0
            } else {
                let na = na / det;
                let nb = nb / det;
                let cost = na * 3 + nb;
                cost
            }
        }
    }
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let values: Vec<_> = value.split('\n').collect();
        let button_a = Self::parse_button(values[0]);
        let button_b = Self::parse_button(values[1]);
        let prize = Self::parse_prize(values[2]);

        Machine {
            button_a,
            button_b,
            prize,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::solve1;
    use crate::solve2;
    use crate::MachineList;

    const SAMPLE_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_solve1() {
        let machine_list = MachineList::from(SAMPLE_INPUT);
        let actual = solve1(&machine_list);
        let expect = 480;
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_solve2() {
        let machine_list = MachineList::from(SAMPLE_INPUT);
        let actual = solve2(&machine_list);
        let expect = 875318608908;
        assert_eq!(actual, expect);
    }
}
