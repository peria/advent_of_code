use std::{collections::VecDeque, io::Read};

fn main() {
    let mut buf = String::new();
    let _ = std::io::stdin().read_to_string(&mut buf);
    let mut lpf = LPF::from(
        buf.trim()
            .split(',')
            .map(|x| x.to_string())
            .collect::<Vec<_>>(),
    );

    println!("{}", lpf.hash_sum());
    lpf.hashmap();
    println!("{}", lpf.focus_power());
}

struct LPF {
    inits: Vec<String>,
    boxes: Vec<Box>,
}

#[derive(Clone)]
struct Box {
    lens: VecDeque<Lens>,
}

impl Box {
    fn new() -> Box {
        Box {
            lens: VecDeque::new(),
        }
    }

    fn remove(&mut self, label: &str) {
        let t = self.lens.iter().position(|x| x.label == label);
        if let Some(index) = t {
            self.lens.remove(index);
        }
    }

    fn set(&mut self, label: &str, val: u64) {
        let t = self.lens.iter().position(|x| x.label == label);
        if let Some(index) = t {
            self.lens[index].focus = val;
        } else {
            self.lens.push_back(Lens {
                label: label.to_string(),
                focus: val,
            });
        }
    }

    fn focus_power(&self) -> u64 {
        self.lens
            .iter()
            .enumerate()
            .map(|(i, x)| (i as u64 + 1) * x.focus)
            .sum()
    }
}

#[derive(Clone)]
struct Lens {
    label: String,
    focus: u64,
}

impl LPF {
    fn hash(s: &str) -> u8 {
        let mut v: u8 = 0;
        for c in s.chars() {
            v = v.wrapping_add(c as u8);
            v = v.wrapping_mul(17);
        }
        v
    }

    fn hash_sum(&self) -> u64 {
        self.inits.iter().map(|x| LPF::hash(x) as u64).sum()
    }

    fn hashmap(&mut self) {
        for x in self.inits.iter() {
            if x.contains("-") {
                let label = x.split('-').next().unwrap();
                let id = Self::hash(&label) as usize;
                self.boxes[id].remove(label);
            } else {
                let mut iter = x.split('=');
                let label = iter.next().unwrap();
                let val: u64 = iter.next().unwrap().parse().unwrap();
                let id = Self::hash(&label) as usize;
                self.boxes[id].set(label, val);
            }
        }
    }

    fn focus_power(&self) -> u64 {
        self.boxes
            .iter()
            .enumerate()
            .map(|(i, x)| (i as u64 + 1) * x.focus_power())
            .sum()
    }
}

impl From<Vec<String>> for LPF {
    fn from(value: Vec<String>) -> Self {
        LPF {
            inits: value,
            boxes: vec![Box::new(); 256],
        }
    }
}
