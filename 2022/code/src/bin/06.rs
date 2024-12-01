use std::collections::HashSet;

fn main() {
    let mut signal = String::new();
    std::io::stdin().read_line(&mut signal).unwrap();

    println!("{}", get_marker(&signal));
    println!("{}", get_message(&signal));
}

fn get_marker(s: &String) -> usize {
    const LENGTH: usize = 4;
    for i in 0..s.len() - LENGTH + 1 {
        let marker = &s[i..(i + LENGTH)];
        let m: HashSet<char> = HashSet::from_iter(marker.chars().into_iter());
        if m.len() == LENGTH {
            return i + LENGTH;
        }
    }
    0
}

fn get_message(s: &String) -> usize {
    const LENGTH: usize = 14;
    for i in 0..s.len() - LENGTH + 1 {
        let marker = &s[i..(i + LENGTH)];
        let m: HashSet<char> = HashSet::from_iter(marker.chars().into_iter());
        if m.len() == LENGTH {
            return i + LENGTH;
        }
    }
    0
}
