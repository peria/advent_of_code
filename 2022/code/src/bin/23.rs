use std::collections::HashSet;
use std::io::BufRead;

type Elf = (i64, i64);

fn main() {
    let elves = get_input();

    println!("{}", count_empty_after(&elves, 10));
    println!("{}", count_until_stop(&elves));
}

fn count_empty_after(elves: &HashSet<Elf>, n: usize) -> i64 {
    let mut elves = elves.clone();
    for i in 0..n {
        let _ = simulate(&mut elves, i);
    }
    let rmin = elves.iter().map(|(r, _c)| r).min().unwrap();
    let rmax = elves.iter().map(|(r, _c)| r).max().unwrap();
    let cmin = elves.iter().map(|(_r, c)| c).min().unwrap();
    let cmax = elves.iter().map(|(_r, c)| c).max().unwrap();
    (cmax - cmin + 1) * (rmax - rmin + 1) - elves.len() as i64
}

fn count_until_stop(elves: &HashSet<Elf>) -> i64 {
    let mut elves = elves.clone();
    let mut i = 0i64;
    loop {
        if !simulate(&mut elves, i as usize) {
            return i + 1;
        }
        i += 1;
        if i % 10 == 0 {
            eprintln!("Count: {}", i);
        }
    }
}

fn simulate(elves: &mut HashSet<Elf>, k: usize) -> bool {
    let mut dests = HashSet::new();
    let mut dup_dests = HashSet::new();
    let mut is_any_updated = false;
    for elf in elves.iter() {
        let (is_updated, dest) = move_to(elves, elf, k);
        if !dests.insert(dest) {
            dup_dests.insert(dest);
        }
        is_any_updated |= is_updated;
    }
    if !is_any_updated {
        return false;
    }

    let mut new_elves = HashSet::new();
    for elf in elves.iter() {
        let (_i, dest) = move_to(elves, elf, k);
        new_elves.insert(if dup_dests.contains(&dest) {
            *elf
        } else {
            dest
        });
    }
    *elves = new_elves;
    true
}

fn move_to(elves: &HashSet<Elf>, (er, ec): &Elf, k: usize) -> (bool, Elf) {
    const AROUND: [Elf; 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    const MOVES: [[Elf; 3]; 4] = [
        [(-1, -1), (-1, 0), (-1, 1)],
        [(1, -1), (1, 0), (1, 1)],
        [(-1, -1), (0, -1), (1, -1)],
        [(-1, 1), (0, 1), (1, 1)],
    ];
    if !AROUND
        .iter()
        .any(|(dr, dc)| elves.contains(&(er + dr, ec + dc)))
    {
        return (false, (*er, *ec));
    }

    for i in 0..4 {
        let mut is_possible = true;
        for (dr, dc) in MOVES[(k + i) % 4] {
            let d = (er + dr, ec + dc);
            if elves.contains(&d) {
                is_possible = false;
            }
        }
        if is_possible {
            let (dr, dc) = MOVES[(k + i) % 4][1];
            return (true, (er + dr, ec + dc));
        }
    }
    (true, (*er, *ec))
}

fn get_input() -> HashSet<Elf> {
    let field: Vec<String> = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.unwrap())
        .collect();
    let mut elves = HashSet::new();
    for (i, l) in field.iter().enumerate() {
        elves.extend(
            l.chars()
                .enumerate()
                .filter(|(_j, x)| x == &'#')
                .map(|(j, _x)| (i as i64, j as i64)),
        );
    }
    elves
}
