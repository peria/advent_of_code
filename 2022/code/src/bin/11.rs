struct Monkey {
    items: Vec<i128>,
    operation: fn(i128) -> i128,
    denominator: i128,
    if_true: usize,
    if_false: usize,
    num_inspection: usize,
}

impl Monkey {
    fn action(&mut self, reduce: fn(i128) -> i128) -> Vec<(usize, i128)> {
        let mut throws = Vec::new();
        for x in &self.items {
            let x = reduce((self.operation)(*x));
            if x % self.denominator == 0 {
                throws.push((self.if_true, x));
            } else {
                throws.push((self.if_false, x));
            }
        }
        self.num_inspection += self.items.len();
        self.items.clear();

        throws
    }
}

fn main() {
    println!("{}", run_actions(20, |x| x / 3));
    println!(
        "{}",
        run_actions(10000, |x| x % (2 * 3 * 5 * 7 * 11 * 13 * 17 * 19))
    );
}

fn run_actions(n: usize, reduce: fn(i128) -> i128) -> usize {
    let mut monkeys = get_monkeys();

    for _ in 0..n {
        for i in 0..monkeys.len() {
            let throws = monkeys[i].action(reduce);
            for (j, x) in throws {
                monkeys[j].items.push(x);
            }
        }
    }

    let mut xs: Vec<usize> = monkeys.iter().map(|x| x.num_inspection).collect();
    xs.sort();
    xs.reverse();
    xs[0] * xs[1]
}

fn get_monkeys() -> Vec<Monkey> {
    if false {
        // Sample
        vec![
            Monkey {
                items: vec![79, 98],
                operation: |x| x * 19,
                denominator: 23,
                if_true: 2,
                if_false: 3,
                num_inspection: 0,
            },
            Monkey {
                items: vec![54, 65, 75, 74],
                operation: |x| x + 6,
                denominator: 19,
                if_true: 2,
                if_false: 0,
                num_inspection: 0,
            },
            Monkey {
                items: vec![79, 60, 97],
                operation: |x| x * x,
                denominator: 13,
                if_true: 1,
                if_false: 3,
                num_inspection: 0,
            },
            Monkey {
                items: vec![74],
                operation: |x| x + 3,
                denominator: 17,
                if_true: 0,
                if_false: 1,
                num_inspection: 0,
            },
        ]
    } else {
        vec![
            Monkey {
                items: vec![74, 64, 74, 63, 53],
                operation: |x| x * 7,
                denominator: 5,
                if_true: 1,
                if_false: 6,
                num_inspection: 0,
            },
            Monkey {
                items: vec![69, 99, 95, 62],
                operation: |x| x * x,
                denominator: 17,
                if_true: 2,
                if_false: 5,
                num_inspection: 0,
            },
            Monkey {
                items: vec![59, 81],
                operation: |x| x + 8,
                denominator: 7,
                if_true: 4,
                if_false: 3,
                num_inspection: 0,
            },
            Monkey {
                items: vec![50, 67, 63, 57, 63, 83, 97],
                operation: |x| x + 4,
                denominator: 13,
                if_true: 0,
                if_false: 7,
                num_inspection: 0,
            },
            Monkey {
                items: vec![61, 94, 85, 52, 81, 90, 94, 70],
                operation: |x| x + 3,
                denominator: 19,
                if_true: 7,
                if_false: 3,
                num_inspection: 0,
            },
            Monkey {
                items: vec![69],
                operation: |x| x + 5,
                denominator: 3,
                if_true: 4,
                if_false: 2,
                num_inspection: 0,
            },
            Monkey {
                items: vec![54, 55, 58],
                operation: |x| x + 7,
                denominator: 11,
                if_true: 1,
                if_false: 5,
                num_inspection: 0,
            },
            Monkey {
                items: vec![79, 51, 83, 88, 93, 76],
                operation: |x| x * 3,
                denominator: 2,
                if_true: 0,
                if_false: 6,
                num_inspection: 0,
            },
        ]
    }
}
