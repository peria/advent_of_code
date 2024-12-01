use std::io::Read;

#[derive(Debug)]
struct Rule {
    from: i64,
    to: i64,
    offset: i64,
}

impl Rule {
    fn new(s: &str) -> Rule {
        let xs: Vec<i64> = s.split(' ').map(|y| y.parse().unwrap()).collect::<Vec<_>>();
        Rule {
            from: xs[1],
            to: xs[1] + xs[2],
            offset: xs[0] - xs[1],
        }
    }
}

#[derive(Debug)]
struct Map {
    rules: Vec<Rule>,
}

impl Map {
    fn new(s: &str) -> Map {
        let lines: Vec<_> = s.split('\n').collect();
        let rules = lines[1..].iter().map(|x| Rule::new(x)).collect();
        Map { rules }
    }

    fn apply(&self, x: i64) -> i64 {
        for rule in self.rules.iter() {
            if x >= rule.from && x < rule.to {
                return x + rule.offset;
            }
        }
        x
    }

    fn apply_range(&self, (from, to): (i64, i64)) -> Vec<(i64, i64)> {
        let mut preprocess = vec![(from, to)];
        let mut processed = Vec::new();
        for r in self.rules.iter() {
            let o = r.offset;
            let mut unprocessed = Vec::new();
            for (f, t) in preprocess.into_iter() {
                if r.from >= t || r.to <= f {
                    unprocessed.push((f, t));
                    continue;
                }

                if f < r.from {
                    if t < r.to {
                        // [f ------- t)
                        //      [rf ------ rt)
                        assert!(f <= r.from);
                        assert!(r.from <= t);
                        assert!(t <= r.to);
                        unprocessed.push((f, r.from));
                        processed.push((r.from + o, t + o));
                    } else {
                        // [f -------------- t)
                        //      [rf --- rt)
                        assert!(f <= r.from);
                        assert!(r.from <= r.to);
                        assert!(r.to <= t);
                        unprocessed.push((f, r.from));
                        processed.push((r.from + o, r.to + o));
                        unprocessed.push((r.to, t));
                    }
                } else {
                    if t < r.to {
                        //      [f -- t)
                        // [rf ---------- rt)
                        assert!(r.from <= f);
                        assert!(f <= t);
                        assert!(t <= r.to);
                        processed.push((f + o, t + o));
                    } else {
                        //      [f ------- t)
                        // [rf ------ rt)
                        assert!(r.from <= f);
                        assert!(f <= r.to);
                        assert!(r.to <= t);
                        processed.push((f + o, r.to + o));
                        unprocessed.push((r.to, t));
                    }
                }
            }
            preprocess = unprocessed;
        }
        processed.extend(preprocess);
        processed
    }
}

fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_to_string(&mut buffer);
    let buffer = buffer.trim();
    let mut groups = Vec::new();
    let mut from = 0;
    for (n, _) in buffer.match_indices("\n\n") {
        groups.push(buffer[from..n].to_string());
        from = n + 2;
    }
    groups.push(buffer[from..].to_string());

    let seeds = parse_seeds(&groups[0]);
    let mut maps = Vec::new();
    for g in groups[1..].iter() {
        maps.push(Map::new(g));
    }

    let val1 = seeds
        .iter()
        .map(|x| {
            let mut x = *x;
            for m in maps.iter() {
                x = m.apply(x);
            }
            x
        })
        .min()
        .unwrap();
    println!("{}", val1);
    let val2 = find_min_location(&seeds, &maps);
    println!("{}", val2);
}

fn parse_seeds(s: &str) -> Vec<i64> {
    s.split(' ')
        .filter(|x| x.len() > 0 && x.chars().nth(0).unwrap().is_numeric())
        .map(|x| x.parse().unwrap())
        .collect()
}

fn find_min_location(seeds: &Vec<i64>, maps: &Vec<Map>) -> i64 {
    let mut ranges = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        let from = seeds[i];
        let width = seeds[i + 1];
        ranges.push((from, from + width));
    }
    for m in maps.iter() {
        let mut new_ranges = Vec::new();
        for r in ranges.iter() {
            new_ranges.extend(m.apply_range(*r));
        }
        ranges = new_ranges;
        ranges.sort();
        eprintln!("{:?}", &ranges);
    }
    ranges[0].0
}
