use itertools::Itertools;
use std::io::Read;

fn main() {
    let universe = Universe::from_stdin();
    let val1 = universe.shortest_distances(2);
    println!("{}", val1);
    let val2 = universe.shortest_distances(1000000);
    println!("{}", val2);
}

struct Universe {
    galaxies: Vec<(usize, usize)>,
}

impl Universe {
    fn from_stdin() -> Universe {
        let mut map = String::new();
        let _ = std::io::stdin().read_to_string(&mut map);
        let map: Vec<_> = map.trim().split('\n').collect();
        let mut galaxies = Vec::new();
        for (i, row) in map.iter().enumerate() {
            for (j, _) in row.char_indices().filter(|x| x.1 == '#') {
                galaxies.push((i, j));
            }
        }
        Universe { galaxies }
    }

    fn shortest_distances(&self, r: usize) -> usize {
        let expand = |d, n, r| -> usize {
            if d == 0 {
                0
            } else {
                (d - 1 - n) * (r - 1) + d
            }
        };

        let mut sum = 0;
        let n = self.galaxies.len();
        let gxs = self
            .galaxies
            .iter()
            .map(|(x, _)| *x)
            .unique()
            .collect::<Vec<_>>();
        let gys = self
            .galaxies
            .iter()
            .map(|(_, y)| *y)
            .unique()
            .collect::<Vec<_>>();
        for i in 0..n {
            for j in (i + 1)..n {
                let (xi, yi) = self.galaxies[i];
                let (xj, yj) = self.galaxies[j];
                let (xi, xj) = (xi.min(xj), xi.max(xj));
                let (yi, yj) = (yi.min(yj), yi.max(yj));
                let xs = gxs.iter().filter(|&x| xi < *x && *x < xj).count();
                let ys = gys.iter().filter(|&y| yi < *y && *y < yj).count();
                let d = expand(xj - xi, xs, r) + expand(yj - yi, ys, r);
                sum += d;
            }
        }
        sum
    }
}
