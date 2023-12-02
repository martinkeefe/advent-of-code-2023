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
    let (_id, hs) = line
        .strip_prefix("Game ")
        .unwrap()
        .split_once(": ")
        .unwrap();

    Game {
        hands: hs.split("; ").map(parse_hand).collect(),
    }
}

// -----------------------------------------------------------------------------
// Solution

fn minimum_bag(game: Game) -> Bag {
    game.hands.iter().fold(
        Bag {
            red: 0,
            green: 0,
            blue: 0,
        },
        |bag, hand| Bag {
            red: bag.red.max(hand.red),
            green: bag.green.max(hand.green),
            blue: bag.blue.max(hand.blue),
        },
    )
}

fn power(cubes: CubeCounts) -> u32 {
    cubes.red * cubes.green * cubes.blue
}

fn process(input: &str) -> u32 {
    input
        .lines()
        .map(parse_game)
        .fold(0, |sum, game| sum + power(minimum_bag(game)))
}

// -----------------------------------------------------------------------------
// Testing

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 2286)
    }
}
