use std::io::BufRead;

fn main() {
    let problems: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|x| Problem::new(&x.unwrap()))
        .collect();
    let val: i64 = problems.iter().map(|p| p.arranges()).sum();
    println!("{}", val);
}

struct Problem {
    record: String,
    springs: Vec<usize>,
}

impl Problem {
    fn new(s: &str) -> Problem {
        let mut iter = s.split(' ');
        let record = iter.next().unwrap().to_string();
        let springs: Vec<_> = iter
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        Problem { record, springs }
    }

    fn arranges(&self) -> i64 {
        let mut debug = vec!['.'; self.len()];
        eprint!("{} {:?}", &self.record, &self.springs);
        let val = self.count_arrange(0, 0, &mut debug);
        eprintln!(" -> {}", val);
        val
    }

    fn count_arrange(&self, from: usize, i: usize, debug: &mut Vec<char>) -> i64 {
        if i >= self.springs.len() {
            return if self.record[from..].find('#').is_none() {
                1
            } else {
                0
            };
        }
        if from + self.springs[i] > self.len() {
            return 0;
        }

        let s = self.springs[i];
        let mut ret = 0;
        for j in from..=(self.len() - s) {
            if self.record[from..j].find('#').is_some() {
                break;
            }
            let jj = j + s;
            if self.record[j..jj].find('.').is_some() {
                continue;
            }
            (j..jj).for_each(|i| debug[i] = '#');
            if jj == self.len() {
                ret += self.count_arrange(jj, i + 1, debug);
            } else if self.record.chars().nth(jj).unwrap() != '#' {
                ret += self.count_arrange(jj + 1, i + 1, debug);
            }
            debug[j] = '.';
        }
        (from..self.len()).for_each(|i| debug[i] = self.record.chars().nth(i).unwrap());
        ret
    }

    fn len(&self) -> usize {
        self.record.len()
    }

    fn arranges_dp(&self) -> i64 {
        let n = self.len();
        let m = self.springs.len();

        let mut dp = vec![vec![0; n]; m];
        for i in 0..=(n - self.springs[0]) {
            dp[0][i] = if self.record[i..(i + self.springs[0])]
                .chars()
                .any(|c| c == '.')
            {
                0
            } else {
                1
            };
            if self.record.chars().nth(i).unwrap() == '#' {
                break;
            }
        }
        for (i, s) in self.springs[1..].iter().enumerate() {
            let i = i + 1;
        }
        0
    }
}
