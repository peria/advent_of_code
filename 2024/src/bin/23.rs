use std::io::Read;

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().lock().read_to_string(&mut buf);
    let data = input(&buf);
    println!("{}", solve1(&data));
    println!("{}", solve2(&data));
}

fn input(buf: &str) -> Data {
    Data {}
}

fn solve1(data: &Data) -> i64 {
    0
}

fn solve2(data: &Data) -> i64 {
    0
}

#[derive(Debug)]
struct Data {}

#[cfg(test)]
mod test {
    use crate::input;
    use crate::solve1;
    use crate::solve2;

    const SAMPLE_INPUT: &str = "1
10
100
2024";

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
