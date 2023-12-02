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

fn first_digit(line: &str) -> String {
    line.chars()
        .find(|c| c.is_ascii_digit())
        .expect("no digits in line")
        .into()
}

fn last_digit(line: &str) -> String {
    line.chars()
        .rev()
        .find(|c| c.is_ascii_digit())
        .expect("no digits in line")
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_sample() {
        let result = process(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, 142)
    }
}
