use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

fn main() {
    let diagram = get_input();
    let answer1 = solve1(&diagram);
    let answer2 = solve2(&diagram);
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn get_input() -> Vec<Vec<char>> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

fn solve1(diagram: &Vec<Vec<char>>) -> i64 {
    let mut beams = HashSet::new();
    // starting point
    beams.insert(diagram[0].iter().position(|c| *c == 'S').unwrap());
    let mut count = 0;
    for row in diagram.iter().skip(1) {
        let mut new_beams = HashSet::new();
        for &x in beams.iter() {
            match row[x] {
                '^' => {
                    if x > 0 {
                        new_beams.insert(x - 1);
                    }
                    if x + 1 < row.len() {
                        new_beams.insert(x + 1);
                    }
                    count += 1;
                }
                '.' => {
                    new_beams.insert(x);
                }
                _ => {}
            }
        }
        beams = new_beams;
    }
    count
}

fn solve2(diagram: &Vec<Vec<char>>) -> i64 {
    let mut tachyon: HashMap<usize, usize> = HashMap::new();
    // starting point
    tachyon.insert(diagram[0].iter().position(|c| *c == 'S').unwrap(), 1);
    for row in diagram.iter().skip(1) {
        let mut new_tachyon = HashMap::new();
        for (&x, &k) in tachyon.iter() {
            match row[x] {
                '^' => {
                    if x > 0 {
                        *new_tachyon.entry(x - 1).or_insert(0) += k;
                    }
                    if x + 1 < row.len() {
                        *new_tachyon.entry(x + 1).or_insert(0) += k;
                    }
                }
                '.' => {
                    *new_tachyon.entry(x).or_insert(0) += k;
                }
                _ => {}
            }
        }
        tachyon = new_tachyon;
    }
    tachyon.values().sum::<usize>() as i64
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_solve1() {
        let input: Vec<Vec<_>> = SAMPLE_INPUT
            .split('\n')
            .map(|s| s.chars().collect())
            .collect();
        assert_eq!(solve1(&input), 21);
    }

    #[test]
    fn test_solve2() {
        let input: Vec<Vec<_>> = SAMPLE_INPUT
            .split('\n')
            .map(|s| s.chars().collect())
            .collect();
        assert_eq!(solve2(&input), 40);
    }
}
