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
    let mut hand = Hand {
        red: 0,
        green: 0,
        blue: 0,
    };

    for c in s.split(", ") {
        let nc = c.split(' ').collect::<Vec<_>>();
        let n = nc[0].parse::<u32>().unwrap();
        match nc[1] {
            "red" => hand.red = n,
            "green" => hand.green = n,
            "blue" => hand.blue = n,
            _ => unreachable!(),
        }
    }

    hand
}

fn parse_game(line: &str) -> Game {
    Game {
        hands: line
            .strip_prefix("Game ")
            .unwrap()
            .split(": ")
            .collect::<Vec<_>>()[1]
            .split("; ")
            .map(parse_hand)
            .collect::<Vec<_>>(),
    }
}

// -----------------------------------------------------------------------------
// Solution

fn minimum_bag(game: Game) -> Bag {
    let mut bag = Bag {
        red: 0,
        green: 0,
        blue: 0,
    };

    for hand in game.hands {
        bag.red = bag.red.max(hand.red);
        bag.green = bag.green.max(hand.green);
        bag.blue = bag.blue.max(hand.blue);
    }

    bag
}

fn power(cubes: CubeCounts) -> u32 {
    cubes.red * cubes.green * cubes.blue
}

fn process(input: &str) -> u32 {
    let mut sum = 0;

    for game in input.lines().map(parse_game) {
        sum += power(minimum_bag(game));
    }

    sum
}

// -----------------------------------------------------------------------------
// Testing

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(process(include_str!("./sample.txt")), 2286)
    }
}
