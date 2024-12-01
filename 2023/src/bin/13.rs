use std::io::Read;

fn main() {
    let mut buffer = String::new();
    let _ = std::io::stdin().read_to_string(&mut buffer);
    let buffer = buffer.trim();
    let mut patterns = Vec::new();
    let mut from = 0;
    for (n, _) in buffer.match_indices("\n\n") {
        let pattern = Pattern::new(&buffer[from..n]);
        patterns.push(pattern);
        from = n + 2;
    }
    let pattern = Pattern::new(&buffer[from..]);
    patterns.push(pattern);

    let val: usize = patterns.iter().map(|x| x.get_reflection_hash_sum()).sum();
    println!("{}", val);
    let val: usize = patterns.iter().map(|x| x.get_smudge_hash_sum()).sum();
    println!("{}", val);
}

enum Reflection {
    Horizontal(usize),
    Vertical(usize),
}

struct Pattern {
    pattern: Vec<String>,
}

impl Pattern {
    fn new(s: &str) -> Pattern {
        let pattern = s.split('\n').map(|x| x.to_string()).collect();
        Pattern { pattern }
    }

    fn is_same_cols(&self, a: usize, b: usize) -> bool {
        self.diff_cols(a, b) == 0
    }

    fn diff_cols(&self, a: usize, b: usize) -> usize {
        self.pattern
            .iter()
            .filter(|y| y.chars().nth(a).unwrap() != y.chars().nth(b).unwrap())
            .count()
    }

    fn is_same_rows(&self, a: usize, b: usize) -> bool {
        self.pattern[a].eq(&self.pattern[b])
    }

    fn diff_rows(&self, a: usize, b: usize) -> usize {
        self.pattern[a]
            .chars()
            .zip(self.pattern[b].chars())
            .filter(|(ai, bi)| ai != bi)
            .count()
    }

    fn mirror(&self) -> Option<Reflection> {
        let h = self.pattern.len();

        // Try vertical first
        for r in 0..(h - 1) {
            if !self.is_same_rows(r, r + 1) {
                continue;
            }
            if (0..=r)
                .rev()
                .zip((r + 1)..h)
                .all(|(a, b)| self.is_same_rows(a, b))
            {
                return Some(Reflection::Vertical(r + 1));
            }
        }

        let w = self.pattern[0].len();
        for c in 0..(w - 1) {
            if !self.is_same_cols(c, c + 1) {
                continue;
            }
            if (0..=c)
                .rev()
                .zip((c + 1)..w)
                .all(|(a, b)| self.is_same_cols(a, b))
            {
                return Some(Reflection::Horizontal(c + 1));
            }
        }

        None
    }

    fn smudge(&self) -> Option<Reflection> {
        let h = self.pattern.len();

        // Try vertical first
        for r in 0..(h - 1) {
            if self.diff_rows(r, r + 1) > 1 {
                continue;
            }
            if (0..=r)
                .rev()
                .zip((r + 1)..h)
                .map(|(a, b)| self.diff_rows(a, b))
                .sum::<usize>()
                == 1
            {
                return Some(Reflection::Vertical(r + 1));
            }
        }

        let w = self.pattern[0].len();
        for c in 0..(w - 1) {
            if self.diff_cols(c, c + 1) > 1 {
                continue;
            }
            if (0..=c)
                .rev()
                .zip((c + 1)..w)
                .map(|(a, b)| self.diff_cols(a, b))
                .sum::<usize>()
                == 1
            {
                return Some(Reflection::Horizontal(c + 1));
            }
        }

        None
    }

    fn get_reflection_hash_sum(&self) -> usize {
        match self.mirror() {
            Some(Reflection::Vertical(row)) => 100 * row,
            Some(Reflection::Horizontal(col)) => col,
            None => todo!(),
        }
    }

    fn get_smudge_hash_sum(&self) -> usize {
        match self.smudge() {
            Some(Reflection::Vertical(row)) => 100 * row,
            Some(Reflection::Horizontal(col)) => col,
            None => {
                eprintln!("{}", self.pattern.join("\n"));
                todo!()
            }
        }
    }
}
