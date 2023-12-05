use std::collections::HashSet;

fn main() {
    println!("{}", process(include_str!("./input.txt")));
}

// -----------------------------------------------------------------------------
// Types

#[derive(Debug)]
struct Card {
    win_nums: HashSet<u32>,
    your_nums: HashSet<u32>,
}

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
    let (_, nums) = line
        .strip_prefix("Card ")
        .unwrap()
        .split_once(": ")
        .unwrap();
    let (s_win, s_your) = nums.split_once(" | ").unwrap();

    Card {
        win_nums: s_win.split(' ').filter_map(|s| s.parse().ok()).collect(),
        your_nums: s_your.split(' ').filter_map(|s| s.parse().ok()).collect(),
    }
}

// -----------------------------------------------------------------------------
// Solution

fn score(card: Card) -> u32 {
    let wins = card
        .your_nums
        .intersection(&card.win_nums)
        .collect::<Vec<_>>()
        .len() as u32;

    if wins == 0 {
        0
    } else {
        2_u32.pow(wins - 1)
    }
}

fn process(input: &str) -> u32 {
    input.lines().map(parse_card).map(score).sum()
}

// -----------------------------------------------------------------------------
// Testing

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 13)
    }
}
