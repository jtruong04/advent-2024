use std::collections::HashMap;

use crate::{
    days::Problem,
    utils::math::{count_digits, split_number},
};

fn blink(stones: HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut updated_stones = HashMap::new();
    for (stone, count) in stones {
        if stone == 0 {
            updated_stones
                .entry(1)
                .and_modify(|v| *v += count)
                .or_insert(count);
        } else if count_digits(stone) % 2 == 0 {
            if let Ok((x, y)) = split_number(stone, count_digits(stone) / 2) {
                updated_stones
                    .entry(x)
                    .and_modify(|v| *v += count)
                    .or_insert(count);
                updated_stones
                    .entry(y)
                    .and_modify(|v| *v += count)
                    .or_insert(count);
            }
        } else {
            updated_stones
                .entry(stone * 2024)
                .and_modify(|v| *v += count)
                .or_insert(count);
        }
    }
    updated_stones
}

fn parse(input: &str) -> HashMap<u64, usize> {
    let mut stones = HashMap::new();
    for number in input.split_ascii_whitespace() {
        if let Ok(number) = number.parse::<u64>() {
            stones.entry(number).and_modify(|v| *v += 1).or_insert(1);
        }
    }
    stones
}

pub struct Solution;

impl Solution {
    fn solve_a(&self, input: &str) -> usize {
        let mut stones = parse(input);
        for _ in 0..25 {
            stones = blink(stones);
        }
        stones.values().sum()
    }

    fn solve_b(&self, input: &str) -> usize {
        let mut stones = parse(input);
        for _ in 0..75 {
            stones = blink(stones);
        }
        stones.values().sum()
    }
}

impl Problem for Solution {
    fn part_one(&self) -> String {
        let input = self.read_file("data/day11/data.txt").unwrap();
        self.solve_a(&input).to_string()
    }

    fn part_two(&self) -> String {
        let input = self.read_file("data/day11/data.txt").unwrap();
        self.solve_b(&input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let parsed_vec = parse("125 17");
        let mut expected = HashMap::new();
        expected.insert(125, 1);
        expected.insert(17, 1);
        assert_eq!(parsed_vec, expected);
    }

    #[test]
    fn test_a() {
        let solution = Solution {};
        let result = solution.solve_a("125 17");
        assert_eq!(result, 55312);
    }
}
