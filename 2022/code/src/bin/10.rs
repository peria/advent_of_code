use std::io::BufRead;

enum Operation {
    Noop,
    AddX { v: i64 },
}

struct Machine {
    clock: i64,
    x: i64,
    target: i64,
    total_strength: i64,
    pixels: String,
}

impl Machine {
    fn process(&mut self, operation: &Operation) {
        match operation {
            Operation::Noop => {
                // eprintln!("{} {} nop", self.clock, self.x);
                self.tick();
            }
            Operation::AddX { v } => {
                // eprintln!("{} {} addx", self.clock, self.x);
                self.tick();
                // eprintln!("{} {} ----", self.clock, self.x);
                self.tick();
                self.x += v;
            }
        };
    }

    fn add_signal_strength(&mut self) {
        self.total_strength += self.x * self.target;
        self.target += 40;
    }

    fn tick(&mut self) {
        let x = self.x - 1 + 40 * 100;
        let l = (x - if x % 40 == 1 { 0 } else { 1 } + 1) % 40;
        let r = (x + if x % 40 == 0 { 0 } else { 1 }) % 40 + 1;
        let c = (self.clock + 39) % 40;
        // eprintln!("{} {} {} : {}", l, x, r, c);
        self.clock += 1;

        if self.clock >= self.target {
            self.add_signal_strength();
        }
        self.pixels.push(if l <= c && c <= r { '#' } else { '.' });
        if c == 39 {
            self.pixels.push('\n');
        }
    }
}

fn main() {
    let operations = read_input();
    let mut machine = Machine {
        clock: 1,
        x: 1,
        target: 20,
        total_strength: 0,
        pixels: "".to_string(),
    };
    for operation in operations.iter() {
        machine.process(operation);
    }
    println!("{}", machine.total_strength);
    println!("{}", machine.pixels);
}

fn read_input() -> Vec<Operation> {
    let mut operations = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let terms: Vec<&str> = line.split(' ').collect();
        let operation = match terms[0] {
            "noop" => Operation::Noop {},
            "addx" => Operation::AddX {
                v: terms[1].parse().unwrap(),
            },
            _ => Operation::Noop {},
        };
        operations.push(operation);
    }
    operations
}
