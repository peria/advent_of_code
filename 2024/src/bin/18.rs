use std::{
    collections::{HashSet, VecDeque},
    io::Read,
};

fn main() {
    let halls = input();
    println!("{}", solve1(&halls, 1024, 70));
    println!("{}", solve2(&halls));
}

fn input() -> Halls {
    let mut buf = String::new();
    let _ = std::io::stdin().lock().read_to_string(&mut buf);
    Halls::from(&buf as &str)
}

fn solve1(halls: &Halls, len: usize, width: usize) -> i64 {
    const DRC: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    let width = width + 1;
    let blocks: HashSet<_> = halls.halls[..len].iter().collect();
    let mut steps = vec![vec![width * width; width]; width];
    let mut q = VecDeque::new();
    q.push_back((0, 0));
    steps[0][0] = 0;
    while !q.is_empty() {
        let (r0, c0) = q.pop_front().unwrap();
        let s = steps[r0][c0] + 1;
        for &drc in DRC.iter() {
            let (r, c) = (r0 as isize + drc.0, c0 as isize + drc.1);
            if r < 0 || c < 0 {
                continue;
            }
            let (r, c) = (r as usize, c as usize);
            if r >= width || c >= width {
                continue;
            }
            if blocks.contains(&(r, c)) {
                continue;
            }
            if s >= steps[r][c] {
                continue;
            }
            steps[r][c] = s;
            q.push_back((r, c));
        }
    }

    steps[width - 1][width - 1] as i64
}

fn solve2(halls: &Halls) -> i64 {
    0
}

#[derive(Debug)]
struct Halls {
    halls: Vec<(usize, usize)>,
}

impl From<&str> for Halls {
    fn from(value: &str) -> Self {
        let halls: Vec<_> = value
            .lines()
            .map(|x| {
                let p = sscanf::sscanf!(x, "{usize},{usize}").ok().unwrap();
                (p.0, p.1)
            })
            .collect();
        Halls { halls }
    }
}

#[cfg(test)]
mod test {
    use crate::solve1;
    use crate::solve2;
    use crate::Halls;

    const SAMPLE_INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_solve1() {
        let data = Halls::from(SAMPLE_INPUT);
        let actual = solve1(&data, 12, 6);
        let expect = 22;
        assert_eq!(expect, actual);
    }

    #[test]
    fn test_solve2() {
        let data = Halls::from(SAMPLE_INPUT);
        let actual = solve2(&data);
        let expect = 0;
        assert_eq!(expect, actual);
    }
}
