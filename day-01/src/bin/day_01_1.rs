fn main() {
    println!("{}", process(include_str!("./input.txt")));
}

// -----------------------------------------------------------------------------
// Solution
//
// 1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet

fn first_digit(line: &str) -> String {
    line.chars().find(|c| c.is_ascii_digit()).unwrap().into()
}

fn last_digit(line: &str) -> String {
    line.chars()
        .rev()
        .find(|c| c.is_ascii_digit())
        .unwrap()
        .into()
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
        assert_eq!(process(include_str!("./sample_1.txt")), 142)
    }
}
