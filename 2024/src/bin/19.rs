use std::{collections::BTreeSet, io::Read};

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().read_to_string(&mut buf).unwrap();

    let (towels, designs) = input(&buf as &str);
    println!("{}", solve1(&towels, &designs));
    println!("{}", solve2(&towels, &designs));
}

fn input(data: &str) -> (Vec<String>, Vec<String>) {
    let mut iter = data.lines();

    let towels = parse_towel(iter.next().unwrap());
    let mut designs = Vec::new();
    iter.next(); // empty line
    for line in iter {
        designs.push(line.to_string());
    }
    (towels, designs)
}

fn parse_towel(data: &str) -> Vec<String> {
    data.split(',').map(|x| x.trim().to_string()).collect()
}

fn solve1(towels: &Vec<String>, designs: &Vec<String>) -> i64 {
    designs.iter().filter(|d| count_ways(towels, d) > 0).count() as i64
}

fn count_ways(towels: &Vec<String>, design: &str) -> usize {
    let mut dp = vec![0; design.len() + 1];
    let mut q = BTreeSet::new();
    q.insert(0);
    dp[0] = 1;
    while !q.is_empty() {
        let s = q.pop_first().unwrap();
        let des = &design[s..];
        for towel in towels.iter() {
            if des.starts_with(towel) {
                let next = s + towel.len();
                dp[next] += dp[s];
                q.insert(next);
            }
        }
    }
    dp[design.len()]
}

fn solve2(towels: &Vec<String>, designs: &Vec<String>) -> i64 {
    designs.iter().map(|d| count_ways(towels, d)).sum::<usize>() as i64
}

#[cfg(test)]
mod test {
    use crate::input;
    use crate::solve1;
    use crate::solve2;

    const SAMPLE_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_solve1() {
        let (towels, designs) = input(SAMPLE_INPUT);
        assert_eq!(solve1(&towels, &designs), 6);
    }

    #[test]
    fn test_solve2() {
        let (towels, designs) = input(SAMPLE_INPUT);
        assert_eq!(solve2(&towels, &designs), 16);
    }
}
