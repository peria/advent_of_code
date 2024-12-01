use std::{collections::HashSet, io::BufRead};

#[derive(Debug)]
struct Card {
    wins: usize,
}

impl Card {
    fn from(s: &str) -> Card {
        let mut iter = s.split(':');
        iter.next();
        let mut iter = iter.next().unwrap().split('|');
        let hands: Vec<_> = iter.next().unwrap().trim().split(' ').collect();
        let hands: HashSet<_> = hands
            .into_iter()
            .filter(|x| x.len() > 0)
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let wins: Vec<_> = iter.next().unwrap().trim().split(' ').collect();
        let wins: HashSet<_> = wins
            .into_iter()
            .filter(|x| x.len() > 0)
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        let wins = hands.intersection(&wins).count();

        Card { wins }
    }

    fn get_points(&self) -> usize {
        match self.wins {
            0 => 0,
            _ => 1 << (self.wins - 1),
        }
    }
}

fn main() {
    let mut cards = Vec::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let card = Card::from(&line);
        cards.push(card);
    }

    let val1: usize = cards.iter().map(|c| c.get_points()).sum();
    let mut num_cards = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let n = card.wins as usize + 1;
        for j in (i + 1)..(i + n).min(cards.len()) {
            num_cards[j] += num_cards[i];
        }
    }
    let val2: i64 = num_cards.iter().sum();

    println!("{}", val1);
    println!("{}", val2);
}
