use std::io::BufRead;

fn main() {
    let data = input();
    println!("{}", solve1(&data));
    println!("{}", solve2(&data));
}

fn input() -> Vec<Vec<char>> {
    let mut data = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.ok().unwrap();
        data.push(line.chars().collect());
    }
    data
}

fn solve1(data: &Vec<Vec<char>>) -> i64 {
    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let h = data.len();
    let w = data[0].len();
    let mut count = 0;
    for (i, row) in data.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            for dij in dirs.iter() {
                let (mut r, mut c) = (i as i32, j as i32);
                let mut contains = true;
                for x in "XMAS".chars() {
                    if r < 0 || c < 0 || r >= h as i32 || c >= w as i32 {
                        contains = false;
                        break;
                    }
                    if x != data[r as usize][c as usize] {
                        contains = false;
                        break;
                    }
                    r += dij.0;
                    c += dij.1;
                }
                if contains {
                    count += 1;
                }
            }
        }
    }
    count
}

fn solve2(data: &Vec<Vec<char>>) -> i64 {
    let h = data.len();
    let w = data[0].len();
    let mut count = 0;
    for (i, row) in data[1..(h - 1)].iter().enumerate() {
        for (j, col) in row[1..(w - 1)].iter().enumerate() {
            if col != &'A' {
                continue;
            }
            let (a, b, c, d) = (
                data[i][j],
                data[i][j + 2],
                data[i + 2][j],
                data[i + 2][j + 2],
            );
            count += match (a, b, c, d) {
                ('M', 'M', 'S', 'S') => 1,
                ('M', 'S', 'M', 'S') => 1,
                ('S', 'M', 'S', 'M') => 1,
                ('S', 'S', 'M', 'M') => 1,
                _ => 0,
            };
        }
    }
    count
}
