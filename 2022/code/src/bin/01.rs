use std::io::BufRead;

fn main() {
    let elves = get_elves_calories();
    let mut elves: Vec<i64> = elves.iter().map(|xs| xs.iter().sum()).collect();
    elves.sort();
    elves.reverse();

    println!("{}", &elves[0]);
    println!("{}", &elves[0..3].iter().sum::<i64>());
}

fn get_elves_calories() -> Vec<Vec<i64>> {
    let mut elves = Vec::new();
    let mut calories = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let x = line.unwrap();
        if x.len() == 0 {
            elves.push(calories);
            calories = Vec::new();
        } else {
            calories.push(x.parse::<i64>().unwrap());
        }
    }
    if calories.len() > 0 {
        elves.push(calories);
    }
    elves
}
