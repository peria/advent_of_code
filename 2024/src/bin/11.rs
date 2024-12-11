use std::{collections::HashMap, io::BufRead};

fn main() {
    let stones = input();
    println!("{}", solve1(&stones));
    println!("{}", solve2(&stones));
}

fn input() -> Stones {
    let mut line = String::new();
    let _ = std::io::stdin().lock().read_line(&mut line);
    Stones::from(&line as &str)
}

fn solve1(stones: &Stones) -> i64 {
    let mut stones = stones.clone();
    for _i in 0..25 {
        stones.blink();
    }

    stones.len() as i64
}

fn solve2(stones: &Stones) -> i64 {
    let mut stones = stones.clone();
    for _i in 0..75 {
        stones.blink();
    }

    stones.len() as i64
}

#[derive(Clone, Debug)]
struct Stones {
    stone_count: HashMap<u64, usize>,
}

impl Stones {
    fn blink(&mut self) {
        let mut next = HashMap::new();
        for (&s, &n) in self.stone_count.iter() {
            if s == 0 {
                next.insert(1, *next.get(&1).unwrap_or(&0) + n);
                continue;
            }

            let sstr = s.to_string();
            if sstr.len() % 2 == 0 {
                let k = sstr.len() / 2;
                let a: u64 = sstr[..k].parse().ok().unwrap();
                let b: u64 = sstr[k..].parse().ok().unwrap();
                next.insert(a, *next.get(&a).unwrap_or(&0) + n);
                next.insert(b, *next.get(&b).unwrap_or(&0) + n);
            } else {
                assert!(s <= std::u64::MAX / 2024);
                let k = s * 2024;
                next.insert(k, *next.get(&k).unwrap_or(&0) + n);
            }
        }
        self.stone_count = next;
    }

    fn len(&self) -> usize {
        self.stone_count.values().sum()
    }
}

impl From<&str> for Stones {
    fn from(value: &str) -> Self {
        let stones: Vec<u64> = value
            .split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let mut count: HashMap<u64, usize> = HashMap::new();
        for &s in stones.iter() {
            count.insert(s, *count.get(&s).unwrap_or(&0) + 1);
        }
        Stones { stone_count: count }
    }
}

#[cfg(test)]
mod test {
    use crate::solve1;
    use crate::solve2;
    use crate::Stones;

    const TEST_INPUT: &str = "125 17";

    #[test]
    fn test_solve1() {
        let stones = Stones::from(TEST_INPUT);
        let actual = solve1(&stones);
        let expect = 55312;
        assert_eq!(expect, actual);
    }
}
