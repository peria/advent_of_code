use std::{collections::HashSet, io::BufRead};

fn main() {
    let boxes = get_input();
    let connect = 1000;
    let answer1 = solve1(&boxes, connect);
    let answer2 = solve2(&boxes);
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}

fn get_input() -> Vec<Box> {
    std::io::stdin()
        .lock()
        .lines()
        .map(|line| Box::from(&line.unwrap() as &str))
        .collect()
}

#[derive(Debug)]
struct Box {
    x: i64,
    y: i64,
    z: i64,
}

impl From<&str> for Box {
    fn from(s: &str) -> Self {
        let mut iter = s.split(',').map(|x| x.parse::<i64>().unwrap());
        Box {
            x: iter.next().unwrap(),
            y: iter.next().unwrap(),
            z: iter.next().unwrap(),
        }
    }
}

fn solve1(boxes: &[Box], mut connect: usize) -> i64 {
    let n = boxes.len();
    let mut uf = UnionFind::new(n);
    let mut edges = Vec::new();
    for (i, bi) in boxes.iter().enumerate() {
        for (j, bj) in boxes.iter().enumerate().skip(i + 1) {
            let distance = (bi.x - bj.x) * (bi.x - bj.x)
                + (bi.y - bj.y) * (bi.y - bj.y)
                + (bi.z - bj.z) * (bi.z - bj.z);
            edges.push((distance, i, j));
        }
    }
    edges.sort();

    for (_, i, j) in edges.iter() {
        uf.union(*i, *j);
        connect -= 1;
        if connect == 0 {
            break;
        }
    }

    let roots: Vec<_> = (0..n).map(|i| uf.find(i)).collect();
    let uroots: HashSet<_> = roots.iter().cloned().collect();
    let mut counts: Vec<_> = uroots
        .iter()
        .map(|r| roots.iter().filter(|&i| uf.find(*i) == *r).count())
        .collect();
    counts.sort();
    counts.iter().rev().take(3).product::<usize>() as i64
}

fn solve2(boxes: &[Box]) -> i64 {
    let n = boxes.len();
    let mut uf = UnionFind::new(n);
    let mut edges = Vec::new();
    for (i, bi) in boxes.iter().enumerate() {
        for (j, bj) in boxes.iter().enumerate().skip(i + 1) {
            let distance = (bi.x - bj.x) * (bi.x - bj.x)
                + (bi.y - bj.y) * (bi.y - bj.y)
                + (bi.z - bj.z) * (bi.z - bj.z);
            edges.push((distance, i, j));
        }
    }
    edges.sort();

    for &(_, i, j) in edges.iter() {
        if uf.n > 2 {
            uf.union(i, j);
            continue;
        }
        if uf.find(i) == uf.find(j) {
            continue;
        }
        return boxes[i].x * boxes[j].x;
    }
    0
}

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    pub n: usize,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            rank: vec![0; size],
            n: size,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            if self.rank[root_x] < self.rank[root_y] {
                self.parent[root_x] = root_y;
            } else if self.rank[root_x] > self.rank[root_y] {
                self.parent[root_y] = root_x;
            } else {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
            self.n -= 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
    const SAMPLE_INPUT_2: usize = 10;

    #[test]
    fn test_solve1() {
        let input: Vec<_> = SAMPLE_INPUT.split('\n').map(|s| Box::from(s)).collect();
        assert_eq!(solve1(&input, SAMPLE_INPUT_2), 40);
    }

    #[test]
    fn test_solve2() {
        let input: Vec<_> = SAMPLE_INPUT.split('\n').map(|s| Box::from(s)).collect();
        assert_eq!(solve2(&input), 25272);
    }
}
