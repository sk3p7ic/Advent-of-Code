use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Debug)]
enum Card {
    Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten,
    Jack, King, Queen, Ace
}

impl Card {
    fn to_strength(self: &Self) -> u8 {
        match self {
            Card::Two => 2, Card::Three => 3, Card::Four => 4, Card::Five => 5,
            Card::Six => 6, Card::Seven => 7, Card::Eight => 8, Card::Nine => 9,
            Card::Ten => 10, Card::Jack => 11, Card::Queen => 12,
            Card::King => 13, Card::Ace => 14
        }
    }
}

impl std::str::FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Card::Two), "3" => Ok(Card::Three), "4" => Ok(Card::Four),
            "5" => Ok(Card::Five), "6" => Ok(Card::Six), "7" => Ok(Card::Seven), 
            "8" => Ok(Card::Eight), "9" => Ok(Card::Nine), "T" => Ok(Card::Ten),
            "J" => Ok(Card::Jack), "Q" => Ok(Card::Queen), "K" => Ok(Card::King),
            "A" => Ok(Card::Ace),
            _ => Err(format!("Invalid token '{}'", s).to_string())
        }
    }
}

#[derive(Debug)]
enum HandKind {
    FiveOfKind, FourOfKind, FullHouse, ThreeOfKind, TwoPair, OnePair, HighCard
}

impl HandKind {
    fn from_hand(hand: &Hand, shadow: bool) -> Self {
        let mut values = hand.hand.iter()
            .map(|c| c.to_strength())
            .collect::<Vec<u8>>();
        if shadow {
            let mut hm: HashMap<u8, usize> = HashMap::new();
            for v in &values {
                hm.entry(v.clone()).and_modify(|e| {*e + 1;}).or_insert(1);
            }
            let mut max_v = 0;
            let mut max_c = 0;
            for (v, c) in hm.into_iter() {
                if max_c.max(c) != max_c {
                    max_v = v;
                    max_c = c;
                }
            }
            for (i, v) in values.clone().iter().enumerate() {
                if v == &Card::Jack.to_strength() {
                    values[i] = max_c as u8;
                }
            }
        }
        values.sort();
        let grouped = (2..=14).map(|n| values.iter()
                .filter(|&v| v == &n)
                .map(|&v| v.clone())
                .collect::<Vec<u8>>())
            .filter(|v| v.len() > 0)
            .collect::<Vec<Vec<u8>>>();
        match grouped.len() {
            1 => Self::FiveOfKind,
            2 => {
                let mut lengths_vec = grouped.iter().map(|g| g.len()).collect::<Vec<_>>();
                lengths_vec.sort();
                let lengths: (usize, usize) = (lengths_vec[0], lengths_vec[1]);
                match lengths {
                    (1, 4) => Self::FourOfKind,
                    (2, 3) => Self::FullHouse,
                    _ => unreachable!()
                }
            },
            3 => {
                let mut lengths_vec = grouped.iter().map(|g| g.len()).collect::<Vec<_>>();
                lengths_vec.sort();
                let lengths: (usize, usize, usize) = (lengths_vec[0], lengths_vec[1], lengths_vec[2]);
                match lengths {
                    (1, 1, 3) => Self::ThreeOfKind,
                    (1, 2, 2) => Self::TwoPair,
                    _ => unreachable!()
                }
            }
            4 => Self::OnePair,
            5 => Self::HighCard,
            _ => unreachable!()
        }
    }

    fn strength_range() -> std::ops::RangeInclusive<u8> {
        0u8..=6u8
    }

    fn to_strength(&self) -> u8 {
        match self {
            Self::HighCard => 0, Self::OnePair => 1, Self::TwoPair => 2,
            Self::ThreeOfKind => 3, Self::FullHouse => 4, Self::FourOfKind => 5,
            Self::FiveOfKind => 6
        }
    }
}

#[derive(Debug)]
struct Hand {
    hand: [Card; 5],
    bid: u16
}

impl Hand {
    fn to_sortable_key(&self) -> u64 {
        let n = self.hand.iter()
            .fold(String::new(), |s, c| match s.len() {
                0 => format!("{:02}", c.to_strength()).to_string(),
                _ => format!("{:02}{:02}", s, c.to_strength()).to_string()
            })
            .parse().expect("to be a number");
        n
    }
}

impl std::str::FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_once(" ").expect("to be valid Hand string");
        let hand = parts.0.chars()
            .map(|c| c.to_string().parse().expect("to be valid card"))
            .collect::<Vec<Card>>()
            .try_into().expect("to have 5 Cards");
        let bid = parts.1.trim().parse().expect("to be a valid number");
        Ok(Self { hand, bid })
    }
}

fn parse_input(input: &str) -> Vec<Hand> {
    input.trim().lines()
        .map(|line| line.parse().expect("to be a valid Hand"))
        .collect()
}

fn process_p1(input: &str) -> u64 {
    let hands = parse_input(&input);
    let mut kinds = hands.iter().map(|h| (h, HandKind::from_hand(h, false))).collect::<Vec<_>>();
    kinds.sort_by(|a, b| b.1.to_strength().partial_cmp(&a.1.to_strength())
        .expect("to be orderable"));
    let mut kind_groups = HandKind::strength_range().map(|s|
        kinds.iter().filter(|&k| k.1.to_strength() == s).collect::<Vec<_>>())
        .filter(|g| g.len() > 0)
        .collect::<Vec<Vec<_>>>();
    kind_groups.sort_by(|a, b| b[0].1.to_strength().partial_cmp(&a[0].1.to_strength()).expect("to sort"));
    for (i, kg) in kind_groups.clone().iter().enumerate() {
        let mut kgc = kg.clone();
        kgc.sort_by(|a, b| b.0.to_sortable_key().partial_cmp(&a.0.to_sortable_key())
            .expect("group to be sortable"));
        kind_groups[i] = kgc;
    }
    let all_kinds = kind_groups.into_iter().flatten().collect::<Vec<_>>();
    let mut result = 0;
    for (i, k) in all_kinds.clone().into_iter().enumerate() {
        result += (all_kinds.len() - i) as u64 * k.0.bid as u64;
    }
    result
}

fn process_p2(input: &str) -> u64 {
    let hands = parse_input(&input);
    let mut kinds = hands.iter().map(|h| (h, HandKind::from_hand(h, true))).collect::<Vec<_>>();
    kinds.sort_by(|a, b| b.1.to_strength().partial_cmp(&a.1.to_strength())
        .expect("to be orderable"));
    let mut kind_groups = HandKind::strength_range().map(|s|
        kinds.iter().filter(|&k| k.1.to_strength() == s).collect::<Vec<_>>())
        .filter(|g| g.len() > 0)
        .collect::<Vec<Vec<_>>>();
    kind_groups.sort_by(|a, b| b[0].1.to_strength().partial_cmp(&a[0].1.to_strength()).expect("to sort"));
    for (i, kg) in kind_groups.clone().iter().enumerate() {
        let mut kgc = kg.clone();
        kgc.sort_by(|a, b| b.0.to_sortable_key().partial_cmp(&a.0.to_sortable_key())
            .expect("group to be sortable"));
        kind_groups[i] = kgc;
    }
    let all_kinds = kind_groups.into_iter().flatten().collect::<Vec<_>>();
    let mut result = 0;
    for (i, k) in all_kinds.clone().into_iter().enumerate() {
        result += (all_kinds.len() - i) as u64 * k.0.bid as u64;
    }
    result
}

fn main() {
    let input = std::fs::read_to_string("./inputs/day07.in.txt").expect("file to exist");
    println!("[2023] D07P01: {}", process_p1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn p1_works() {
        assert_eq!(6440, process_p1(&SAMPLE));
    }

    #[test]
    fn p2_works() {
        assert_eq!(5905, process_p2(&SAMPLE));
    }
}
