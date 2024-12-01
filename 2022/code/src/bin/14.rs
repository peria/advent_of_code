use std::io::BufRead;

const Y_MAX: usize = 200;
const X_MAX: usize = 700;

const EMPTY: char = '.';
const WALL: char = '#';
const STONE: char = 'o';

fn main() {
    let (mut map, ymax) = get_input();

    let count = simulate(&mut map);
    println!("{}", count);
    for x in 0..X_MAX {
        map[ymax + 2][x] = WALL;
    }
    println!("{}", count + simulate(&mut map));
}

fn simulate(map: &mut Vec<Vec<char>>) -> usize {
    let mut count = 0;
    loop {
        let (mut x, mut y) = (500, 0);
        loop {
            if map[y][x] == STONE {
                return count;
            }
            y += 1;
            if y >= Y_MAX {
                return count;
            }
            if map[y][x] == EMPTY {
                continue;
            }
            if map[y][x - 1] == EMPTY {
                x -= 1;
                continue;
            }
            if map[y][x + 1] == EMPTY {
                x += 1;
                continue;
            }
            break;
        }
        map[y - 1][x] = STONE;
        count += 1;
        /*
        for r in 0..80 {
            eprintln!("{:?}", &map[r].iter().collect::<String>()[470..540]);
        }
        eprintln!("");
        */
    }
}

fn get_input() -> (Vec<Vec<char>>, usize) {
    let mut map = vec![vec![EMPTY; X_MAX]; Y_MAX];
    let mut y_max = 0;
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let vertexs: Vec<(i32, i32)> = line
            .split(" -> ")
            .map(|s| {
                let mut iter = s.split(',');
                let x: i32 = iter.next().unwrap().parse().unwrap();
                let y: i32 = iter.next().unwrap().parse().unwrap();
                (x, y)
            })
            .collect();
        let n = vertexs.len();
        for i in 0..(n - 1) {
            let (x0, y0) = vertexs[i];
            let (x1, y1) = vertexs[i + 1];
            let dx = (x1 - x0).signum();
            let dy = (y1 - y0).signum();
            let (mut x, mut y) = (x0, y0);
            loop {
                map[y as usize][x as usize] = WALL;
                x += dx;
                y += dy;
                if (x, y) == (x1, y1) {
                    break;
                }
            }
            map[y1 as usize][x1 as usize] = WALL;
            y_max = usize::max(usize::max(y_max, y1 as usize), y0 as usize);
        }
    }
    (map, y_max)
}
