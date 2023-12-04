use std::collections::{HashMap, HashSet};

use crate::tools::vec_lines;
use regex::Regex;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Card {
    id: u32,
    winnings: HashSet<u32>,
    numbers: HashSet<u32>,
    win_count: u32,
    win_score: u32,
    exp_cards: Vec<u32>,
}

type CardSet = HashMap<u32, Card>;

fn parse_card(source: String) -> Card {
    let card_regex = Regex::new(r"Card[ ]*(?<i>[0-9]+): (?<w>[\ 0-9]+)\|(?<m>[\ 0-9]+)").unwrap();
    let cap = card_regex.captures(source.as_str()).unwrap();
    let id = cap.name("i").unwrap().as_str().parse::<u32>().unwrap();
    let winnings = cap
        .name("w")
        .map(|ws| {
            ws.as_str()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<HashSet<u32>>()
        })
        .unwrap();
    let numbers = cap
        .name("m")
        .map(|ws| {
            ws.as_str()
                .split_whitespace()
                .map(|x| x.trim().parse::<u32>().unwrap())
                .collect::<HashSet<u32>>()
        })
        .unwrap();
    let win_count = numbers.intersection(&winnings).count() as u32;
    let win_score = match win_count {
        0 => 0,
        _ => u32::pow(2, win_count - 1),
    };
    let exp_cards = (1..=win_count).map(|i| id + i).collect();
    Card {
        id,
        winnings,
        numbers,
        win_count,
        win_score,
        exp_cards,
    }
}

pub fn prepare_card_set(source: Vec<String>) -> CardSet {
    let mut card_set = CardSet::new();
    source.iter().for_each(|l| {
        let c = parse_card(l.to_string());
        card_set.insert(c.id, c);
    });
    card_set
}

fn new_cards<'a>(source: &'a CardSet, cards: &Vec<&Card>) -> Vec<&'a Card> {
    let mut new_cards_list = vec![];
    for card in cards {
        let cnt = card.win_count;
        for i in 1..=cnt {
            new_cards_list.push(source.get(&(card.id + i)).unwrap());
        }
    }
    new_cards_list
}

pub fn expand<'a>(source: &'a CardSet) -> Vec<&'a Card> {
    let mut expanded = vec![];
    source.values().for_each(|card| {
        expanded.push(card);
    });
    let mut added = new_cards(source, &expanded);
    while !added.is_empty() {
        expanded.append(&mut added.clone());
        added = new_cards(source, &added);
    }

    expanded
}

pub fn run() {
    let cs = prepare_card_set(vec_lines("4_input.txt"));
    println!(
        "Points: {:?}",
        &cs.values().map(|x| x.win_score).sum::<u32>()
    );
    println!("Card stack: {:?}", expand(&cs).len());
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::e4::{expand, parse_card, prepare_card_set, Card};

    #[test]
    fn test_expand() {
        let cs = prepare_card_set(vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string(),
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string(),
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string(),
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string(),
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string(),
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string(),
        ]);
        assert_eq!(expand(&cs).len(), 30);
    }

    #[test]
    fn test_parse_card() {
        assert_eq!(
            parse_card("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string()),
            Card {
                id: 1,
                winnings: HashSet::from([41, 48, 83, 86, 17]),
                numbers: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
                win_count: 4,
                win_score: 8,
                exp_cards: vec![2, 3, 4, 5]
            }
        )
    }
}
