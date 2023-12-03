use std::collections::{HashMap, HashSet};

fn main() {
    println!("{}", process(include_str!("./input.txt")));
}

// -----------------------------------------------------------------------------
// Types

#[derive(Debug)]
struct Schematic {
    width: i32,
    height: i32,
    chars: Vec<Vec<char>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Number {
    row: i32,
    col: i32,
    len: i32,
    value: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Symbol {
    row: i32,
    col: i32,
    value: char,
}

// -----------------------------------------------------------------------------
// Parsing

// 467..114..
// ...*......
// ..35..633.
// ......#...
// 617*......
// .....+.58.
// ..592.....
// ......755.
// ...$.*....
// .664.598..

fn parse_schematic(input: &str) -> Schematic {
    let chars: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    Schematic {
        width: chars[0].len() as i32,
        height: chars.len() as i32,
        chars,
    }
}

// -----------------------------------------------------------------------------
// Solution

fn is_digit(ch: char) -> bool {
    ch.is_ascii_digit()
}

fn is_symbol(ch: &char) -> bool {
    *ch != '.' && !ch.is_ascii_alphanumeric()
}

fn u32_from(chs: &[char]) -> u32 {
    chs.iter().collect::<String>().parse::<u32>().unwrap()
}

fn number(row: i32, next_col: i32, num: &[char]) -> Number {
    let len = num.len() as i32;
    Number {
        row,
        col: next_col - len,
        len,
        value: u32_from(num),
    }
}

fn symbol(row: i32, col: i32, value: char) -> Symbol {
    Symbol { row, col, value }
}

impl Schematic {
    fn at(&self, row: i32, col: i32) -> char {
        if 0 <= row && row < self.height && 0 <= col && col < self.width {
            self.chars[row as usize][col as usize]
        } else {
            '.'
        }
    }
    fn neighbors(&self, row: i32, col: i32) -> Vec<(char, i32, i32)> {
        vec![
            (self.at(row - 1, col - 1), row - 1, col - 1),
            (self.at(row - 1, col), row - 1, col),
            (self.at(row - 1, col + 1), row - 1, col + 1),
            (self.at(row, col - 1), row, col - 1),
            (self.at(row, col + 1), row, col + 1),
            (self.at(row + 1, col - 1), row + 1, col - 1),
            (self.at(row + 1, col), row + 1, col),
            (self.at(row + 1, col + 1), row + 1, col + 1),
        ]
    }
    fn part_numbers(&self) -> Vec<(Number, Symbol)> {
        let mut nums = vec![];

        for r in 0..self.height {
            let mut digits: Vec<char> = vec![];
            for c in 0..self.width {
                let ch = self.at(r, c);
                if is_digit(ch) {
                    digits.push(ch);
                } else if !digits.is_empty() {
                    let num = number(r, c, &digits);
                    digits.clear();
                    if let Some(sym) = self.is_part_number(&num) {
                        nums.push((num, sym));
                    }
                }
            }
            if !digits.is_empty() {
                let num = number(r, self.width, &digits);
                if let Some(sym) = self.is_part_number(&num) {
                    nums.push((num, sym));
                }
            }
        }

        nums
    }
    fn is_part_number(&self, num: &Number) -> Option<Symbol> {
        for col in num.col..(num.col + num.len) {
            let ns = self.neighbors(num.row, col);
            for (n, r, c) in ns {
                if is_symbol(&n) {
                    return Some(symbol(r, c, n));
                }
            }
        }
        None
    }
}

fn process(input: &str) -> u32 {
    let mut map: HashMap<Symbol, HashSet<Number>> = HashMap::new();

    for (num, sym) in parse_schematic(input).part_numbers() {
        if sym.value == '*' {
            map.entry(sym).or_insert_with(HashSet::new).insert(num);
        }
    }

    map.iter()
        .filter_map(|(_, nums)| {
            if nums.len() == 2 {
                Some(nums.iter().map(|num| num.value).product::<u32>())
            } else {
                None
            }
        })
        .sum()
}

// -----------------------------------------------------------------------------
// Testing

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 467835)
    }
}
