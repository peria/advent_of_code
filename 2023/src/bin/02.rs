use sscanf::sscanf;
use std::io::BufRead;

struct Game {
    id: i64,
    sets: Vec<(usize, usize, usize)>,
}

impl Game {
    fn is_possible(&self, (rt, gt, bt): (usize, usize, usize)) -> bool {
        for (r, g, b) in self.sets.iter() {
            if *r > rt || *g > gt || *b > bt {
                return false;
            }
        }
        true
    }

    fn water(&self) -> i64 {
        let (mut r, mut g, mut b) = (0, 0, 0);
        self.sets.iter().for_each(|(rr, gg, bb)| {
            r = r.max(*rr);
            g = g.max(*gg);
            b = b.max(*bb);
        });
        (r * g * b) as i64
    }
}

fn main() {
    let mut games = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut a = line.split(':');
        let id = sscanf!(a.next().unwrap(), "Game {}", i64).unwrap();
        let sets = parse_sets(a.next().unwrap());
        games.push(Game { id, sets });
    }

    let val1 = games
        .iter()
        .filter(|&g| g.is_possible((12, 13, 14)))
        .map(|g| g.id)
        .collect::<Vec<_>>();
    let val1 = val1.iter().sum::<i64>();
    println!("{}", val1);
    let val2 = games.iter().map(|g| g.water()).sum::<i64>();
    println!("{}", val2);
}

fn parse_sets(s: &str) -> Vec<(usize, usize, usize)> {
    let mut ret = Vec::new();
    for x in s.split(';') {
        let mut val = (0, 0, 0);
        for y in x.split(',') {
            match sscanf!(y, " {} red", usize) {
                Ok(n) => {
                    val.0 = n;
                }
                _ => (),
            };
            match sscanf!(y, " {} green", usize) {
                Ok(n) => {
                    val.1 = n;
                }
                _ => (),
            };
            match sscanf!(y, " {} blue", usize) {
                Ok(n) => {
                    val.2 = n;
                }
                _ => (),
            };
        }
        ret.push(val);
    }
    ret
}
