fn main() {
    println!("{}", process(include_str!("./input.txt")));
}

// -----------------------------------------------------------------------------
// Types

#[derive(Debug)]
struct CubeCounts {
    red: u32,
    green: u32,
    blue: u32,
}

type Bag = CubeCounts;
type Hand = CubeCounts;

#[derive(Debug)]
struct Game {
    id: u32,
    hands: Vec<Hand>,
}

// -----------------------------------------------------------------------------
// Parsing
//
// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

// e.g. 5 blue, 13 green
fn parse_hand(s: &str) -> Hand {
    s.split(", ").fold(
        Hand {
            red: 0,
            green: 0,
            blue: 0,
        },
        |hand, c| {
            let (n, clr) = c.split_once(' ').unwrap();
            let n = n.parse::<u32>().unwrap();
            match clr {
                "red" => Hand { red: n, ..hand },
                "green" => Hand { green: n, ..hand },
                "blue" => Hand { blue: n, ..hand },
                _ => unreachable!(),
            }
        },
    )
}

fn parse_game(line: &str) -> Game {
    let (id, hs) = line
        .strip_prefix("Game ")
        .unwrap()
        .split_once(": ")
        .unwrap();

    Game {
        id: id.parse::<u32>().unwrap(),
        hands: hs.split("; ").map(parse_hand).collect(),
    }
}

// -----------------------------------------------------------------------------
// Solution

// curried function!
fn possible_with(bag: Bag) -> impl Fn(&Hand) -> bool {
    move |hand| hand.red <= bag.red && hand.green <= bag.green && hand.blue <= bag.blue
}

fn process(input: &str) -> u32 {
    let valid = possible_with(Bag {
        red: 12,
        green: 13,
        blue: 14,
    });

    input
        .lines()
        .map(parse_game)
        .filter(|game| game.hands.iter().all(&valid))
        .fold(0, |sum, game| sum + game.id)
}

// -----------------------------------------------------------------------------
// Testing

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 8)
    }
}
