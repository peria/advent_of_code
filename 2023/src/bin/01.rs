use std::io::BufRead;

fn main() {
    let (val1, val2) = solve();
    println!("{}", val1);
    println!("{}", val2);
}

fn solve() -> (i64, i64) {
    let mut sum1 = 0;
    let mut sum2 = 0;
    for line in std::io::stdin().lock().lines() {
        match line {
            Ok(digs) => {
                let val1 = parse_digits(&digs);
                let val2 = parse_digits(&replace_spellings(&digs));
                sum1 += val1;
                sum2 += val2;
                eprintln!("{} -> {} / {}", &digs, val1, val2);
            }
            _ => (),
        }
    }
    (sum1, sum2)
}

fn parse_digits(s: &str) -> i64 {
    let digits: String = s.chars().filter(|c| c.is_numeric()).collect();
    let val = (digits.chars().nth(0).unwrap() as i64 - '0' as i64) * 10
        + digits.chars().nth_back(0).unwrap() as i64
        - '0' as i64;
    val
}

fn replace_spellings(s: &str) -> String {
    let table = [
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    let mut t: Vec<char> = s.chars().collect();
    let s = s.to_string();
    for (x, c) in table.iter() {
        s.match_indices(x).for_each(|(i, _)| t[i] = *c);
    }
    t.iter().collect()
}
