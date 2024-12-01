use std::io::BufRead;

#[derive(Debug)]
enum Move {
    Go(usize),
    Left,
    Right,
}

fn main() {
    let (map, moves) = get_input();

    println!("{}", trace_2d(&map, &moves));
    println!("{}", trace_3d(&map, &moves));
}

fn trace_2d(map: &Vec<Vec<char>>, moves: &Vec<Move>) -> i32 {
    const DIFFS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let start = (
        0,
        map[0].iter().position(|x| x == &'.').unwrap() as i32,
        0usize,
    );

    let h = map.len() as i32;
    let w = map[0].len() as i32;
    let (mut r, mut c, mut d) = start;
    for m in moves.iter() {
        match m {
            Move::Go(n) => {
                let (dr, dc) = &DIFFS[d];
                for _ in 0..*n {
                    let mut nr = (r + dr + h) % h;
                    let mut nc = (c + dc + w) % w;
                    while map[nr as usize][nc as usize] == ' ' {
                        nr = (nr + dr + h) % h;
                        nc = (nc + dc + w) % w;
                    }
                    if map[nr as usize][nc as usize] == '#' {
                        break;
                    }
                    r = nr;
                    c = nc;
                }
            }
            Move::Right => {
                d = (d + 1) % 4;
            }
            Move::Left => {
                d = (d + 3) % 4;
            }
        }
    }
    eprintln!("({}, {}) {}", r, c, d);
    (r + 1) * 1000 + (c + 1) * 4 + d as i32
}

fn trace_3d(map: &Vec<Vec<char>>, moves: &Vec<Move>) -> i32 {
    const DIFFS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let start = (
        0,
        map[0].iter().position(|x| x == &'.').unwrap() as i32,
        0usize,
    );

    let (mut r, mut c, mut d) = start;
    for m in moves.iter() {
        match m {
            Move::Go(n) => {
                /*
                    +-a-+-b-+
                    c   |   d
                    +---+-e-+
                    f   E
                +-F-+---+
                C   |   D
                +---+-g-+
                A   G
                +-B-+
                  */
                for _ in 0..*n {
                    let (dr, dc) = &DIFFS[d];
                    let (mut nr, mut nc, mut nd) = (r + dr, c + dc, d);
                    if nr < 0 {
                        if nc < 100 {
                            (nr, nc, nd) = (150 + nc - 50, 0, 0); // a->A
                        } else {
                            (nr, nc, nd) = (199, nc - 100, 3); // b->B
                        }
                    } else if nc >= 150 {
                        (nr, nc, nd) = (49 - nr + 100, 99, 2); // d->D
                    } else if nc < 0 {
                        if nr < 150 {
                            (nr, nc, nd) = (149 - nr, 50, 0); // C->c
                        } else {
                            (nr, nc, nd) = (0, nr - 150 + 50, 1); // A->a
                        }
                    } else if nr >= 200 {
                        (nr, nc, nd) = (0, nc + 100, 1); // B->b
                    } else if nr < 100 && nc == 49 {
                        if nr < 50 {
                            (nr, nc, nd) = (49 - nr + 100, 0, 0); // c->C
                        } else {
                            (nr, nc, nd) = (100, nr - 50, 1); // f->F
                        }
                    } else if nr >= 50 && nc >= 100 && d == 1 {
                        (nr, nc, nd) = (nc - 100 + 50, 99, 2); // e->E
                    } else if nr >= 50 && nr < 100 && nc == 100 && d == 0 {
                        (nr, nc, nd) = (49, nr - 50 + 100, 3); // E->e
                    } else if nr == 99 && nc < 50 && d == 3 {
                        (nr, nc, nd) = (nc + 50, 50, 0); // F->f
                    } else if nr >= 100 && nr < 150 && nc == 100 && d == 0 {
                        (nr, nc, nd) = (149 - nr, 149, 2); // D->d
                    } else if nr >= 150 && nc >= 50 && d == 1 {
                        (nr, nc, nd) = (nc - 50 + 150, 49, 2); // g->G
                    } else if nr >= 150 && nc == 50 && d == 0 {
                        (nr, nc, nd) = (149, nr - 150 + 50, 3); // G->g
                    }
                    if map[nr as usize][nc as usize] == '#' {
                        break;
                    }
                    r = nr;
                    c = nc;
                    d = nd;
                }
            }
            Move::Right => {
                d = (d + 1) % 4;
            }
            Move::Left => {
                d = (d + 3) % 4;
            }
        }
    }
    eprintln!("({}, {}) {}", r, c, d);
    (r + 1) * 1000 + (c + 1) * 4 + d as i32
}

fn get_input() -> (Vec<Vec<char>>, Vec<Move>) {
    let mut map = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        map.push(line.chars().collect());
    }
    let h = map.len();
    let w = map.iter().map(|x: &Vec<char>| x.len()).max().unwrap();
    for i in 0..h {
        map[i].resize_with(w, || ' ');
    }

    let mut operation = String::new();
    let _ = std::io::stdin().read_line(&mut operation);
    let operation = operation.trim();
    let re = regex::Regex::new(r"(\d+|[RL])").unwrap();
    let mut moves = Vec::new();
    for capture in re.captures_iter(operation) {
        let m = match &capture[1] {
            "L" => Move::Left,
            "R" => Move::Right,
            _ => Move::Go(capture[1].parse::<usize>().unwrap()),
        };
        moves.push(m);
    }
    (map, moves)
}
