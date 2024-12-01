use std::io::Read;

fn main() {
    let mut maze = String::new();
    let _ = std::io::stdin().read_to_string(&mut maze);
    let maze: Vec<Vec<char>> = maze
        .trim()
        .split('\n')
        .map(|x| x.chars().collect())
        .collect();
    eprintln!("{:?}", &maze);

    let val1 = get_farthest_len(&maze);
    println!("{}", val1);
}

fn get_farthest_len(maze: &Vec<Vec<char>>) -> i64 {
    let h = maze.len();
    let w = maze[0].len();
    let mut uf = UnionFind::new(h * w);
    for i in 0..h {
        for j in 0..w {
            let c = maze[i][j];
            if i > 0 && "|JL".contains(c) && "|7F".contains(maze[i - 1][j]) {
                uf.unite(i * w + j, (i - 1) * w + j);
            }
            if i < h - 1 && "|7F".contains(c) && "|JL".contains(maze[i + 1][j]) {
                uf.unite(i * w + j, (i + 1) * w + j);
            }
            if j > 0 && "-J7".contains(c) && "-FL".contains(maze[i][j - 1]) {
                uf.unite(i * w + j, i * w + j - 1);
            }
            if j < w - 1 && "-FL".contains(c) && "-J7".contains(maze[i][j + 1]) {
                uf.unite(i * w + j, i * w + j + 1);
            }
        }
    }
    let (sr, row) = maze
        .iter()
        .enumerate()
        .find(|x| x.1.contains(&'S'))
        .unwrap();
    let (sc, _) = row.iter().enumerate().find(|x| x.1 == &'S').unwrap();
    eprintln!("S: {}, {}", sr, sc);

    let size = if uf.root((sr + 1) * w + sc) == uf.root(sr * w + sc + 1)
        || uf.root((sr + 1) * w + sc) == uf.root(sr * w + sc - 1)
        || uf.root((sr + 1) * w + sc) == uf.root((sr - 1) * w + sc)
    {
        uf.size((sr + 1) * w + sc)
    } else if uf.root(sr * w + sc - 1) == uf.root(sr * w + sc + 1)
        || uf.root(sr * w + sc - 1) == uf.root((sr - 1) * w + sc)
    {
        uf.size(sr * w + sc - 1)
    } else {
        uf.size(sr * w + sc + 1)
    };
    ((size + 1) / 2) as i64
}

struct UnionFind {
    parents: Vec<i64>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            parents: vec![-1; n],
        }
    }

    fn unite(&mut self, mut a: usize, mut b: usize) {
        a = self.root(a);
        b = self.root(b);
        if a == b {
            return;
        }
        if self.size(a) < self.size(b) {
            self.parents[a] += self.parents[b];
            self.parents[b] = a as i64;
        } else {
            self.parents[b] += self.parents[a];
            self.parents[a] = b as i64;
        }
    }

    fn size(&mut self, a: usize) -> usize {
        let r = self.root(a);
        (-self.parents[r]) as usize
    }

    fn root(&mut self, a: usize) -> usize {
        let p = self.parents[a];
        if p < 0 {
            return a;
        }
        let p = self.root(p as usize);
        self.parents[a] = p as i64;
        p
    }
}
