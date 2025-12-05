use std::io::{BufRead, Read};

fn main() {
    let (ranges, ids) = get_input();
    let answer1 = solve1(&ranges, &ids);
    let answer2 = solve2(&ranges, &ids);
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn get_input() -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap();
    let mut input_groups = buffer.split("\n\n");

    let parse_range = |line: &str| -> (i64, i64) {
        let mut iter = line.split('-').map(|x| x.parse::<i64>().unwrap());
        (iter.next().unwrap(), iter.next().unwrap())
    };
    let ranges = input_groups
        .next()
        .unwrap()
        .split("\n")
        .map(|line| parse_range(line))
        .collect();

    let ids: Vec<_> = input_groups
        .next()
        .unwrap()
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    (ranges, ids)
}

fn solve1(ranges: &Vec<(i64, i64)>, ids: &Vec<i64>) -> i64 {
    let in_range = |id: &i64| -> bool {
        ranges
            .iter()
            .any(|(start, end)| *start <= *id && *id <= *end)
    };

    ids.iter().filter(|id| in_range(id)).count() as i64
}

fn solve2(ranges: &Vec<(i64, i64)>, _ids: &Vec<i64>) -> i64 {
    let mut ranges = ranges.clone();
    ranges.sort();

    let mut merged_ranges = Vec::new();
    for (start, end) in ranges.iter() {
        if let Some((last_start, last_end)) = merged_ranges.last_mut() {
            if *start <= *last_end {
                *last_end = (*last_end).max(*end);
                continue;
            }
        }
        merged_ranges.push((*start, *end));
    }

    merged_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    const SAMPLE_RANGES: [(i64, i64); 4] = [(3, 5), (10, 14), (16, 20), (12, 18)];
    const SAMPLE_IDS: [i64; 6] = [1, 5, 8, 11, 17, 32];

    #[test]
    fn test_solve1() {
        let ranges = SAMPLE_RANGES.to_vec();
        let ids = SAMPLE_IDS.to_vec();

        assert_eq!(solve1(&ranges, &ids), 3);
    }

    #[test]
    fn test_solve2() {
        let ranges = SAMPLE_RANGES.to_vec();
        let ids = SAMPLE_IDS.to_vec();

        assert_eq!(solve2(&ranges, &ids), 14);
    }
}
