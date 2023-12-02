use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    input.lines().fold(0, |acc, line| {
        acc + [first_digit(line), last_digit(line)]
            .join("")
            .parse::<i32>()
            .expect("cannot parse")
    })
}

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

fn first_digit(line: &str) -> String {
    let rx = Regex::new("[1-9]|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let m = rx.find(line).expect("no digits in line").as_str();
    digit(m)
}

fn last_digit(line: &str) -> String {
    let rx = Regex::new("[1-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin").unwrap();
    let rev_line = line.chars().rev().collect::<String>();
    let m = rx.find(&rev_line).expect("no digits in line").as_str();
    digit(&m.chars().rev().collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_sample() {
        let result = process(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, 281)
    }
}
