use std::io::Read;

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().lock().read_to_string(&mut buf);
    let data = input(&buf);
    println!("{}", solve1(&data));
    println!("{}", solve2(&data));
}

fn input(buf: &str) -> Data {
    let data = buf
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    Data { seeds: data }
}

fn solve1(data: &Data) -> i64 {
    let mut sum = 0;
    for seed in data.seeds.iter() {
        let mut secret = *seed;
        for _ in 0..2000 {
            secret = succeed(secret);
        }
        sum += secret;
    }
    sum
}

fn solve2(data: &Data) -> i64 {
    0
}

fn succeed(mut secret: i64) -> i64 {
    secret ^= secret * 64;
    secret &= 16777215;
    secret ^= secret / 32;
    secret &= 16777215;
    secret ^= secret * 2048;
    secret &= 16777215;
    secret
}

#[derive(Debug)]
struct Data {
    seeds: Vec<i64>,
}

#[cfg(test)]
mod test {
    use crate::input;
    use crate::solve1;
    use crate::solve2;
    use crate::succeed;

    const SAMPLE_INPUT: &str = "1
10
100
2024";

    #[test]
    fn test_succeed() {
        let expects = [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        let mut secret = 123;
        for &expect in expects.iter() {
            let actual = succeed(secret);
            assert_eq!(expect, actual);
            secret = expect;
        }
    }

    #[test]
    fn test_solve1() {
        let data = input(SAMPLE_INPUT);
        let actual = solve1(&data);
        let expect = 37327623;
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_solve2() {
        let data = input(SAMPLE_INPUT);
        let actual = solve2(&data);
        let expect = 0;
        assert_eq!(actual, expect);
    }
}
