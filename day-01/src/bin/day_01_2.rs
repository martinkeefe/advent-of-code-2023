use regex::Regex;

fn main() {
    println!("{}", process(include_str!("./input.txt")));
}

// -----------------------------------------------------------------------------
// Solution
//
// two1nine
// eightwothree
// abcone2threexyz
// xtwone3four
// 4nineeightseven2
// zoneight234
// 7pqrstsixteen

fn digit(s: &str) -> String {
    if s.len() == 1 {
        s
    } else {
        match s {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            _ => unreachable!(),
        }
    }
    .into()
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn first_digit(line: &str) -> String {
    digit(
        Regex::new("[1-9]|one|two|three|four|five|six|seven|eight|nine")
            .unwrap()
            .find(line)
            .unwrap()
            .as_str(),
    )
}

fn last_digit(line: &str) -> String {
    digit(&reverse(
        Regex::new("[1-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin")
            .unwrap()
            .find(&reverse(line))
            .unwrap()
            .as_str(),
    ))
}

fn process(input: &str) -> u32 {
    input.lines().fold(0, |sum, line| {
        sum + [first_digit(line), last_digit(line)]
            .join("")
            .parse::<u32>()
            .unwrap()
    })
}

// -----------------------------------------------------------------------------
// Testing

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_sample() {
        assert_eq!(process(include_str!("./sample_2.txt")), 281)
    }
}
