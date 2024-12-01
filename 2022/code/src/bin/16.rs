use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::BufRead;

fn main() {
    let (rates, distances) = get_input();
    println!("{}", control_by_one(&rates, &distances));
    println!("{}", control_by_two(&rates, &distances));
}

fn control_by_one(rates: &Vec<usize>, distances: &Vec<Vec<usize>>) -> usize {
    let pressures = compute_pressures(rates, distances, 30);
    *pressures.iter().max().unwrap()
}

fn control_by_two(rates: &Vec<usize>, distances: &Vec<Vec<usize>>) -> usize {
    let pressures = compute_pressures(rates, distances, 26);
    let n = pressures.len();
    let mut max_sum = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            if i & j != 0 {
                continue;
            }
            let pressure = pressures[i] + pressures[j];
            max_sum = max_sum.max(pressure);
        }
    }
    max_sum
}

fn compute_pressures(rates: &Vec<usize>, distances: &Vec<Vec<usize>>, limit: i64) -> Vec<usize> {
    let n = rates.len();
    let mut pressures = vec![0; 1 << (n - 1)];

    // (-time, bit set, position, pressure)
    let mut q = BinaryHeap::new();
    q.push((0, 1usize, 0, 0));
    while let Some((t, bits, from, pressure)) = q.pop() {
        pressures[bits >> 1] = pressures[bits >> 1].max(pressure);
        for to in 0..n {
            let b = 1 << to;
            if (b & bits) != 0 {
                continue;
            }
            let bits = bits | b;
            let t = t - distances[from][to] as i64 - 1;
            if t < -limit {
                continue;
            }
            let pressure = pressure + rates[to] * (limit + t) as usize;
            q.push((t, bits, to, pressure));
        }
    }
    pressures
}

fn get_input() -> (Vec<usize>, Vec<Vec<usize>>) {
    let re_valves = regex::Regex::new(r"([A-Z]{2})").unwrap();
    let re_rate = regex::Regex::new(r"(\d+)").unwrap();
    let mut rates: HashMap<String, usize> = HashMap::new();
    let mut nexts: HashMap<String, Vec<String>> = HashMap::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let valves: Vec<&str> = re_valves
            .captures_iter(&line)
            .map(|x| x.get(0).unwrap().as_str())
            .collect();
        let rate = re_rate.captures(&line).unwrap().get(1).unwrap().as_str();
        let name = valves[0].to_string();
        rates.insert(name.to_string(), rate.parse().unwrap());
        nexts.insert(
            name.to_string(),
            valves[1..].iter().map(|x| x.to_string()).collect(),
        );
    }
    // Convert `rates` and `nexts` to a vector-based distance matrix.
    let mut names = vec!["AA".to_string()];
    let iter = rates
        .iter()
        .filter(|(k, &v)| v > 0 && **k != "AA")
        .map(|(k, _v)| k.clone());
    names.extend(iter);
    // `names` has valve names whose rates are > 0 or "AA".

    let n = names.len();
    let rates = names.iter().map(|k| rates[k]).collect();
    let mut distances = vec![vec![1000; n]; n];
    for (i, name) in names.iter().enumerate() {
        distances[i][i] = 0;
        let mut known = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back((name, 0usize));
        known.insert(name);
        while let Some((from, d)) = q.pop_front() {
            let d = d + 1;
            for next in &nexts[from] {
                if known.contains(next) {
                    continue;
                }
                known.insert(next);
                match names.iter().position(|x| x == next) {
                    Some(index) => distances[i][index] = d,
                    None => q.push_back((next, d)),
                }
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                distances[i][j] = distances[i][j].min(distances[i][k] + distances[k][j]);
            }
        }
    }

    (rates, distances)
}
