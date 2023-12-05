use std::collections::{HashMap, HashSet};

fn main() {
    println!("{}", process(include_str!("./input.txt")));
}

// -----------------------------------------------------------------------------
// Types

#[derive(Debug)]
struct Card {
    id: u32,
    win_nums: HashSet<u32>,
    your_nums: HashSet<u32>,
}

type CardIndex = HashMap<u32, Card>;
type CountMemo = HashMap<u32, u32>;

// -----------------------------------------------------------------------------
// Parsing
//
// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11

fn parse_card(line: &str) -> Card {
    let (s_id, nums) = line
        .strip_prefix("Card ")
        .unwrap()
        .split_once(": ")
        .unwrap();
    let (s_win, s_your) = nums.split_once(" | ").unwrap();

    Card {
        id: s_id.trim().parse().unwrap(),
        win_nums: s_win.split(' ').filter_map(|s| s.parse().ok()).collect(),
        your_nums: s_your.split(' ').filter_map(|s| s.parse().ok()).collect(),
    }
}

// -----------------------------------------------------------------------------
// Solution

fn count(index: &CardIndex, memo: &mut CountMemo, card: &Card) -> u32 {
    if memo.contains_key(&card.id) {
        *memo.get(&card.id).unwrap()
    } else {
        let wins = card
            .your_nums
            .intersection(&card.win_nums)
            .collect::<Vec<_>>()
            .len();

        let mut n = 1;

        for i in 0..wins as u32 {
            n += count(index, memo, index.get(&(card.id + 1 + i)).unwrap());
        }

        memo.insert(card.id, n);

        n
    }
}

fn process(input: &str) -> u32 {
    let mut card_index: CardIndex = HashMap::new();
    let mut count_memo: CountMemo = HashMap::new();

    for card in input.lines().map(parse_card) {
        card_index.insert(card.id, card);
    }

    card_index
        .values()
        .map(|card| count(&card_index, &mut count_memo, card))
        .sum()
}

// -----------------------------------------------------------------------------
// Testing

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 30)
    }
}
