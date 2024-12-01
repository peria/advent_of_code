use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::BufRead;

const DXYZ: &'static [(i32, i32, i32)] = &[
    (0, 0, -1),
    (0, 0, 1),
    (0, -1, 0),
    (0, 1, 0),
    (-1, 0, 0),
    (1, 0, 0),
];

fn main() {
    let cubes = get_input();

    println!("{}", get_surface(&cubes));
    println!("{}", get_exterior_surface(&cubes));
}

fn get_surface(cubes: &HashSet<(i32, i32, i32)>) -> i64 {
    let mut s = cubes.len() as i64 * 6;
    for (x, y, z) in cubes.iter() {
        for (dx, dy, dz) in DXYZ.iter() {
            let nx = x + dx;
            let ny = y + dy;
            let nz = z + dz;
            if cubes.contains(&(nx, ny, nz)) {
                s -= 1;
            }
        }
    }
    s
}

fn get_exterior_surface(cubes: &HashSet<(i32, i32, i32)>) -> i64 {
    let (mut x0, mut y0, mut z0) = cubes.iter().next().unwrap();
    let (mut x1, mut y1, mut z1) = cubes.iter().next().unwrap();
    for (x, y, z) in cubes.iter() {
        x0 = x0.min(*x - 1);
        y0 = y0.min(*y - 1);
        z0 = z0.min(*z - 1);
        x1 = x1.max(*x + 1);
        y1 = y1.max(*y + 1);
        z1 = z1.max(*z + 1);
    }
    let mut s = 0;
    let mut e = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((x0, y0, z0));
    while let Some((cx, cy, cz)) = q.pop_front() {
        for (dx, dy, dz) in DXYZ {
            let n = (cx + dx, cy + dy, cz + dz);
            if e.contains(&n) {
                continue;
            }
            if cubes.contains(&n) {
                s += 1;
                continue;
            }
            let (nx, ny, nz) = n;
            if nx < x0 || nx > x1 || ny < y0 || ny > y1 || nz < z0 || nz > z1 {
                continue;
            }
            q.push_back(n.clone());
            e.insert(n);
        }
    }
    s
}

fn get_input() -> HashSet<(i32, i32, i32)> {
    let mut cubes = HashSet::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let xyz: Vec<i32> = line.split(",").map(|x| x.parse().unwrap()).collect();
        cubes.insert((xyz[0], xyz[1], xyz[2]));
    }
    cubes
}
