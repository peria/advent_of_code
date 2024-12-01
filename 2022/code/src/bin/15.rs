use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let (sensors, beacons) = get_input();
    let y = if sensors[0] == (2, 18) { 10 } else { 2000000 };
    println!("{}", count_no_beacons(&sensors, &beacons, y));
    println!("{}", find_tuning_freq(&sensors, &beacons, y));
}

fn count_no_beacons(sensors: &Vec<(i32, i32)>, beacons: &Vec<(i32, i32)>, ty: i32) -> usize {
    let ranges = get_x_ranges(sensors, beacons, ty);
    eprintln!("There are {} ranges.", ranges.len());
    let ret: i32 = ranges.iter().map(|(l, r)| r - l).sum();
    let mut exists: HashSet<&(i32, i32)> = HashSet::from_iter(beacons.iter().clone());
    exists.extend(HashSet::<&(i32, i32)>::from_iter(sensors.iter().clone()));
    let exists = exists
        .iter()
        .filter(|(x, y)| y == &ty && ranges.iter().any(|(l, r)| l <= x && x < r))
        .count();
    ret as usize - exists
}

fn find_tuning_freq(sensors: &Vec<(i32, i32)>, beacons: &Vec<(i32, i32)>, ty: i32) -> usize {
    for y in (ty / 2)..(ty * 2) {
        let rs = get_x_ranges(sensors, beacons, y);
        if rs.len() != 2 {
            continue;
        }
        let (_l0, r0) = rs[0];
        let (l1, _r1) = rs[1];
        if r0 + 1 != l1 {
            continue;
        }
        let x = r0 as usize;
        let freq = x * 4000000 + (y as usize);
        eprintln!("({}, {}) -> {}", x, y, freq);
        return freq;
    }
    0
}

fn get_x_ranges(sensors: &Vec<(i32, i32)>, beacons: &Vec<(i32, i32)>, ty: i32) -> Vec<(i32, i32)> {
    let mut ranges = Vec::new();
    for (s, b) in sensors.iter().zip(beacons.iter()) {
        let (sx, sy) = s;
        let (bx, by) = b;
        let d = (sx - bx).abs() + (sy - by).abs();
        let w = d - (sy - ty).abs();
        if w < 0 {
            continue;
        }
        ranges.push((sx - w, sx + w + 1));
    }
    ranges.sort();
    merge_ranges(&ranges)
}

fn merge_ranges(xs: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut ys = Vec::new();
    let mut l = None;
    let mut r = None;
    for x in xs {
        let (xl, xr) = x;
        if l.is_none() {
            l = Some(xl);
            r = Some(xr);
            continue;
        }
        if r.unwrap() < xl {
            ys.push((*l.unwrap(), *r.unwrap()));
            l = Some(xl);
        }
        r = Some(xr.max(r.unwrap()));
    }
    ys.push((*l.unwrap(), *r.unwrap()));
    ys
}

fn get_input() -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let mut sensors = Vec::new();
    let mut beacons = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let (mut sx, mut sy) = (0, 0);
        let (mut bx, mut by) = (0, 0);
        let _ = scanf::sscanf!(
            &line,
            "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            sx,
            sy,
            bx,
            by
        );
        sensors.push((sx, sy));
        beacons.push((bx, by));
    }
    (sensors, beacons)
}

/*
               1    1    2    2
     0    5    0    5    0    5
-2 ###########.......#########.
-1 ############...#.###########
 0 ####S########.##############
 1 ######################S#####
 2 ###############S############
 3 ################SB##########
 4 ###########################.
 5 ##########################..
 6 #########################...
 7 .#########S#######S#####....
 8 ..#######################...
 9 .#########################..
10 ####B######################.
11 ##S#############.###########
12 ############################
13 .###########################
14 .#############S#######S#####
15 B###########################
16 ###########SB###############
17 ################S##########B
18 ####S######################.
19 ##########################..
20 ############S######S#####...
21 ##################...###....
22 .#######..#####.#.....#B....
 */
