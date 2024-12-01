use std::collections::HashMap;
use std::io::BufRead;

#[derive(Clone)]
enum Job {
    Node {
        left: String,
        operator: char,
        right: String,
    },
    Number(i64),
}

impl Job {
    fn as_number(&self) -> i64 {
        if let Job::Number(x) = self {
            return *x;
        }
        0
    }
}

fn main() {
    let jobs = get_input();

    println!("{}", get_root(&jobs));
    println!("{}", get_humn(&jobs));
}

fn get_root(jobs: &HashMap<String, Job>) -> i64 {
    let mut jobs: HashMap<String, Job> = jobs.clone();
    resolve(&mut jobs);
    jobs["root"].as_number()
}

fn get_humn(jobs: &HashMap<String, Job>) -> i64 {
    let mut jobs = jobs.clone();
    let root = jobs["root"].clone();
    if let Job::Node {
        left,
        operator: _,
        right,
    } = root
    {
        jobs.insert(
            "root".to_string(),
            Job::Node {
                left: left,
                operator: '=',
                right: right,
            },
        );
    }
    let (mut low, mut high) = (0, 10000000000000);
    while low + 1 < high {
        let humn = (low + high) / 2;
        let mut jobs2 = jobs.clone();
        jobs2.insert("humn".to_string(), Job::Number(humn));
        resolve(&mut jobs2);
        match jobs2["root"].as_number() {
            -1 => {
                high = humn;
            }
            1 => {
                low = humn;
            }
            0 => {
                return humn;
            }
            _ => {}
        }
    }
    0
}

fn resolve(jobs: &mut HashMap<String, Job>) {
    loop {
        let mut new_jobs = HashMap::new();
        for (k, v) in jobs.iter() {
            match v {
                Job::Node {
                    left: l,
                    operator: op,
                    right: r,
                } => {
                    if let (Job::Number(l), Job::Number(r)) = (&jobs[l], &jobs[r]) {
                        new_jobs.insert(
                            k.to_string(),
                            Job::Number(match op {
                                '+' => l + r,
                                '-' => l - r,
                                '*' => l * r,
                                '/' => l / r,
                                '=' => l.cmp(r) as i64,
                                _ => 0,
                            }),
                        );
                    }
                }
                Job::Number(_) => {}
            }
        }
        if new_jobs.is_empty() {
            break;
        }
        jobs.extend(new_jobs);
    }
}

fn get_input() -> HashMap<String, Job> {
    let re = regex::Regex::new("[-+/*]").unwrap();
    let mut jobs = HashMap::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let terms: Vec<&str> = line.split(": ").collect();
        let name = terms[0];
        let job = if re.is_match(&terms[1]) {
            let terms: Vec<&str> = terms[1].split(" ").collect();
            Job::Node {
                left: terms[0].to_string(),
                operator: terms[1].chars().nth(0).unwrap(),
                right: terms[2].to_string(),
            }
        } else {
            Job::Number(terms[1].parse().unwrap())
        };
        jobs.insert(name.to_string(), job);
    }
    jobs
}
