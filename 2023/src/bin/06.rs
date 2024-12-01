use std::io::BufRead;

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    fn ways(&self) -> i64 {
        if self.distance >= self.measure(self.time / 2) {
            return 0;
        }
        let mut ok = self.time / 2;
        let mut ng = 0;
        while ng + 1 < ok {
            let m = (ng + ok) / 2;
            if self.distance < self.measure(m) {
                ok = m;
            } else {
                ng = m;
            }
        }
        let v = self.time - 2 * ok + 1;
        eprintln!("<{}>", v);
        v
    }

    fn measure(&self, x: i64) -> i64 {
        x * (self.time - x)
    }
}

fn main() {
    let lines: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|x| parse_line(&x.unwrap()))
        .collect();
    let races: Vec<_> = lines[0]
        .iter()
        .zip(lines[1].iter())
        .map(|(x, y)| Race {
            time: *x,
            distance: *y,
        })
        .collect();
    let val1 = races.iter().map(|r| r.ways()).fold(1, |p, x| p * x);

    let time = races
        .iter()
        .map(|r| r.time.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<i64>()
        .unwrap();
    let distance = races
        .iter()
        .map(|r| r.distance.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<i64>()
        .unwrap();
    let val2 = Race { time, distance }.ways();
    println!("{}", val1);
    println!("{}", val2);
}

fn parse_line(s: &str) -> Vec<i64> {
    let terms: Vec<_> = s.split(' ').filter(|x| !x.is_empty()).collect();
    terms[1..].iter().map(|x| x.parse().unwrap()).collect()
}
