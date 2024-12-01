use std::collections::{HashSet, VecDeque};
use std::io::BufRead;

type BluePrint = Vec<Vec<usize>>;

fn main() {
    let blueprints = get_input();
    println!("{}", get_sum_quality_level(&blueprints));
    println!("{}", prod_first_three(&blueprints));
}

fn get_sum_quality_level(blueprints: &Vec<BluePrint>) -> i64 {
    let mut sum = 0;
    for (i, blue) in blueprints.iter().enumerate() {
        let quality = get_quality_level(blue, 24);
        eprintln!("{:?} -> {}", blue, quality);
        sum += (i + 1) as i64 * quality;
    }
    sum
}

fn prod_first_three(blues: &Vec<BluePrint>) -> i64 {
    let mut prod = 1;
    let n = blues.len().min(3);
    for blue in blues[..n].iter() {
        let quality = get_quality_level(blue, 32);
        eprintln!("{:?} -> {}", blue, quality);
        prod *= quality;
    }
    prod
}

fn get_quality_level(blueprint: &BluePrint, minutes: usize) -> i64 {
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    let mut ret = 0;
    let time = 0;
    let resources = vec![0; 4];
    let robots = vec![1, 0, 0, 0];
    q.push_back((time, resources.clone(), robots.clone()));
    visited.insert((time, resources, robots));
    while let Some((time, resources, mut robots)) = q.pop_front() {
        for i in 0..4 {
            let costs = &blueprint[i];
            let time_to_build: Vec<Option<usize>> = costs
                .iter()
                .zip(resources.iter())
                .map(|(&cost, &resource)| cost as i64 - resource as i64)
                .zip(robots.iter())
                .map(|(need, &robot)| {
                    if need <= 0 {
                        Some(0)
                    } else if robot == 0 {
                        None
                    } else {
                        Some((need as usize + robot - 1) / robot)
                    }
                })
                .collect();
            // Impossible to build the robot.
            if time_to_build.iter().any(|x| x.is_none()) {
                continue;
            }
            let runtime = time_to_build.iter().map(|x| x.unwrap()).max().unwrap() as usize;
            let time = time + runtime + 1;
            if time <= minutes {
                let resources: Vec<usize> = resources
                    .iter()
                    .zip(robots.iter())
                    .map(|(x, y)| x + y * runtime + y)
                    .zip(costs.iter())
                    .map(|(x, y)| x - y)
                    .collect();
                robots[i] += 1;

                let t = minutes - time;
                let geode = resources[3] + robots[3] * t;
                ret = ret.max(geode);
                if t > 0 && geode + t * (t - 1) / 2 > ret {
                    if visited.insert((time, resources.clone(), robots.clone())) {
                        q.push_back((time, resources, robots.clone()));
                    }
                }
                robots[i] -= 1;
            }
        }
    }
    ret as i64
}

fn get_input() -> Vec<BluePrint> {
    let mut blueprints = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let s: Vec<&str> = line.split(": ").collect();
        let mut costs = vec![vec![0; 4]; 4];
        for si in s[1].split(". ") {
            let s: Vec<&str> = si.split(" costs ").collect();
            let mut robot_name = String::new();
            let _ = scanf::sscanf!(s[0], "Each {} robot", robot_name);
            let rid = get_index(&robot_name);
            for cost in s[1].split(" and ") {
                let mut amount = 0;
                let mut name = String::new();
                let cost = cost.trim_end_matches('.');
                let _ = scanf::sscanf!(cost, "{} {}", amount, name);
                costs[rid][get_index(&name)] = amount;
            }
        }
        blueprints.push(costs);
    }
    blueprints
}

fn get_index(k: &str) -> usize {
    match k {
        "ore" => 0,
        "clay" => 1,
        "obsidian" => 2,
        "geode" => 3,
        _ => 4,
    }
}
