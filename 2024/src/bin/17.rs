use std::io::Read;

use sscanf;

fn main() {
    let computer = input();
    println!("{}", solve1(&computer));
    println!("{}", solve2(&computer));
}

fn input() -> Computer {
    let mut buf = String::new();
    let _ = std::io::stdin().lock().read_to_string(&mut buf);
    Computer::from(&buf as &str)
}

fn solve1(computer: &Computer) -> String {
    let outputs = computer.run();
    outputs
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn solve2(computer: &Computer) -> u64 {
    computer.solve()
}

#[derive(Debug, Clone)]
struct Computer {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,
    program: Vec<u8>,
}

impl Computer {
    const ADV: u8 = 0; // Ra <= Ra / 2^COMBO
    const BXL: u8 = 1; // Rb <= Rb & operand
    const BST: u8 = 2; // Rb <= COMBO % 8
    const JNZ: u8 = 3; // if Ra != 0 then PC <= operand
    const BXC: u8 = 4; // Rb <= Rb ^ Rc
    const OUT: u8 = 5; // Print(COMBO % 8)
    const BDV: u8 = 6; // Rb <= Ra / 2^COMBO
    const CDV: u8 = 7; // Rc <= Ra / 2^COMBO

    fn run(&self) -> Vec<u8> {
        let mut outputs = Vec::new();
        let mut reg_a = self.reg_a;
        let mut reg_b = self.reg_b;
        let mut reg_c = self.reg_c;
        let mut pc = 0;
        loop {
            // eprintln!("Ra: {}", reg_a);
            // eprintln!("Rb: {}", reg_b);
            // eprintln!("Rc: {}", reg_c);
            // eprintln!(
            //     "{}",
            //     self.program
            //         .iter()
            //         .map(|x| x.to_string())
            //         .collect::<Vec<_>>()
            //         .join(",")
            // );
            // eprint!("{:width$}", "", width = pc * 2);
            // eprintln!("^");

            if pc >= self.program.len() {
                break;
            }
            let opcode = self.program[pc];
            let operand = self.program[pc + 1];
            let combo = || match operand {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => reg_a,
                5 => reg_b,
                6 => reg_c,
                _ => 0,
            };
            let mut was_jnz = false;
            match opcode {
                Self::ADV => {
                    // Ra <= Ra / 2^COMBO
                    reg_a >>= combo();
                }
                Self::BXL => {
                    // Rb <= Rb & 1
                    reg_b ^= operand as u64;
                }
                Self::BST => {
                    // Rb <= COMBO % 8
                    reg_b = combo() % 8;
                }
                Self::JNZ => {
                    // if Ra != 0 then PC <= operand
                    if reg_a != 0 {
                        pc = operand as usize;
                        was_jnz = true;
                    }
                }
                Self::BXC => {
                    // Rb <= Rb ^ Rc
                    reg_b ^= reg_c;
                }
                Self::OUT => {
                    // Print(COMBO % 8)
                    outputs.push(combo() as u8 % 8);
                }
                Self::BDV => {
                    // Rb <= Ra / 2^COMBO
                    reg_b = reg_a >> combo();
                }
                Self::CDV => {
                    // Rc <= Ra / 2^COMBO
                    reg_c = reg_a >> combo();
                }
                _ => {
                    eprintln!("Unknown opcode: {}", opcode);
                }
            }

            if was_jnz {
                was_jnz = false;
            } else {
                pc += 2;
            }
        }

        outputs
    }

    fn solve(&self) -> u64 {
        const START: u64 = 0o3004700000000000;
        let mut computer = self.clone();
        let mut ra = START;
        let mut max_len = 0;
        loop {
            computer.reg_a = ra;
            let output = computer.run();
            if output == computer.program {
                return ra;
            }
            if ra < START + 10u64 {
                eprintln!("{:o} > {:?} {:?}", ra, &output, &self.program);
                max_len = output.len();
            }
            ra += 1;
        }
    }
}

impl From<&str> for Computer {
    fn from(value: &str) -> Self {
        let lines: Vec<_> = value.trim().lines().collect();
        let reg_a = sscanf::sscanf!(lines[0], "Register A: {u64}").ok().unwrap();
        let reg_b = sscanf::sscanf!(lines[1], "Register B: {u64}").ok().unwrap();
        let reg_c = sscanf::sscanf!(lines[2], "Register C: {u64}").ok().unwrap();
        let code = lines[4].split(' ').nth(1).unwrap();
        let program = code.split(',').map(|x| x.parse().unwrap()).collect();

        Computer {
            reg_a,
            reg_b,
            reg_c,
            program,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::solve1;
    use crate::solve2;
    use crate::Computer;

    const SAMPLE_INPUT_1: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    const SAMPLE_INPUT_2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

    #[test]
    fn test_solve1() {
        let computer = Computer::from(SAMPLE_INPUT_1);
        let actual = solve1(&computer);
        let expect = "4,6,3,5,6,3,5,2,1,0";
        assert_eq!(&actual as &str, expect);
    }

    #[test]
    fn test_solve2() {
        let computer = Computer::from(SAMPLE_INPUT_2);
        let actual = solve2(&computer);
        let expect = 117440;
        assert_eq!(actual, expect);
    }
}
