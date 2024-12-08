use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

fn main() {
    let field = input();
    println!("{}", solve1(&field));
    println!("{}", solve2(&field));
}

fn input() -> Vec<Vec<char>> {
    let mut field = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.ok().unwrap();
        field.push(line.chars().collect());
    }
    field
}

fn solve1(field: &Vec<Vec<char>>) -> i64 {
    let height = field.len() as i64;
    let width = field[0].len() as i64;

    let mut anntenas: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    for (i, row) in field.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '.' {
                continue;
            }

            if !anntenas.contains_key(&cell) {
                anntenas.insert(cell, Vec::new());
            }
            anntenas.get_mut(&cell).unwrap().push((i as i64, j as i64));
        }
    }

    let mut nodes: HashSet<(i64, i64)> = HashSet::new();
    for xs in anntenas.values() {
        let n = xs.len();
        if n < 2 {
            continue;
        }
        for a in xs.iter() {
            for b in xs.iter() {
                if a == b {
                    continue;
                }
                let d = (a.0 - b.0, a.1 - b.1);
                let x = (a.0 + d.0, a.1 + d.1);
                if 0 <= x.0 && x.0 < height && 0 <= x.1 && x.1 < width {
                    nodes.insert(x);
                }
            }
        }
    }

    nodes.len() as i64
}

fn solve2(field: &Vec<Vec<char>>) -> i64 {
    let height = field.len() as i64;
    let width = field[0].len() as i64;

    let mut anntenas: HashMap<char, Vec<(i64, i64)>> = HashMap::new();
    for (i, row) in field.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '.' {
                continue;
            }

            if !anntenas.contains_key(&cell) {
                anntenas.insert(cell, Vec::new());
            }
            anntenas.get_mut(&cell).unwrap().push((i as i64, j as i64));
        }
    }

    let mut nodes: HashSet<(i64, i64)> = HashSet::new();
    for xs in anntenas.values() {
        for &x in xs.iter() {
            nodes.insert(x);
        }
        let n = xs.len();
        if n < 2 {
            continue;
        }
        for a in xs.iter() {
            for b in xs.iter() {
                if a == b {
                    continue;
                }
                let d = (a.0 - b.0, a.1 - b.1);
                let mut x = (a.0 + d.0, a.1 + d.1);
                while 0 <= x.0 && x.0 < height && 0 <= x.1 && x.1 < width {
                    nodes.insert(x);
                    x.0 += d.0;
                    x.1 += d.1;
                }
            }
        }
    }

    nodes.len() as i64
}
