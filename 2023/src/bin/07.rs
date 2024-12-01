use std::{cmp::Ordering, io::BufRead};

#[derive(Debug, Clone)]
struct Hand {
    cards: String,
    hand_type: Type,
    hand_joker: Type,
    bid: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeCards,
    FullHouse,
    FourCards,
    FiveCards,
}

impl Hand {
    fn new(s: &str) -> Hand {
        let tokens = s.split(' ').collect::<Vec<_>>();
        let cards = tokens[0].to_string();
        let hand_type = Self::get_type(&cards);
        let hand_joker = Self::get_type_joker(&cards);
        let bid = tokens[1].parse().unwrap();
        Hand {
            cards,
            hand_type,
            hand_joker,
            bid,
        }
    }

    fn get_type(cs: &str) -> Type {
        let cs: Vec<_> = cs.chars().collect();
        let mut n = 0;
        for (i, a) in cs.iter().enumerate() {
            for b in cs[(i + 1)..].iter() {
                if a == b {
                    n += 1;
                }
            }
        }
        match n {
            10 => Type::FiveCards,
            6 => Type::FourCards,
            4 => Type::FullHouse,
            3 => Type::ThreeCards,
            2 => Type::TwoPairs,
            1 => Type::OnePair,
            0 => Type::HighCard,
            _ => panic!("Unknown {}", n),
        }
    }

    fn get_type_joker(cs: &str) -> Type {
        let mut my_hand = Type::HighCard;
        for c in "A23456789TQK".chars() {
            let cards = cs
                .chars()
                .clone()
                .map(|x| if x == 'J' { c } else { x })
                .collect::<String>();
            let hand = Self::get_type(&cards);
            if my_hand < hand {
                my_hand = hand;
            }
        }
        my_hand
    }
}

fn main() {
    let hands: Vec<_> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| Hand::new(&l.unwrap()))
        .collect();
    eprintln!("{:?}", &hands);

    let val1 = total_winning(&hands);
    println!("{}", val1);
    let val2 = total_winning_with_joker(&hands);
    println!("{}", val2);
}

fn total_winning(hands: &Vec<Hand>) -> i64 {
    let mut hands = (*hands).clone();
    hands.sort_by(|a, b| {
        a.hand_type
            .cmp(&b.hand_type)
            .then(cmp_cards(&a.cards, &b.cards))
    });
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i as i64 + 1) * h.bid)
        .fold(0, |s, x| s + x)
}

fn total_winning_with_joker(hands: &Vec<Hand>) -> i64 {
    let mut hands = (*hands).clone();
    hands.sort_by(|a, b| {
        a.hand_joker
            .cmp(&b.hand_joker)
            .then(cmp_cards2(&a.cards, &b.cards))
    });
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i as i64 + 1) * h.bid)
        .fold(0, |s, x| s + x)
}

fn cmp_cards(a: &str, b: &str) -> Ordering {
    const STRENGTH: &str = "23456789TJQKA";

    for (ai, bi) in a.chars().zip(b.chars()) {
        let sa = STRENGTH.find(ai).unwrap_or(100);
        let sb = STRENGTH.find(bi).unwrap_or(100);
        if sa == sb {
            continue;
        }
        return if sa < sb {
            Ordering::Less
        } else {
            Ordering::Greater
        };
    }
    Ordering::Equal
}

fn cmp_cards2(a: &str, b: &str) -> Ordering {
    const STRENGTH: &str = "J23456789TQKA";

    for (ai, bi) in a.chars().zip(b.chars()) {
        let sa = STRENGTH.find(ai).unwrap_or(100);
        let sb = STRENGTH.find(bi).unwrap_or(100);
        if sa == sb {
            continue;
        }
        return if sa < sb {
            Ordering::Less
        } else {
            Ordering::Greater
        };
    }
    Ordering::Equal
}
