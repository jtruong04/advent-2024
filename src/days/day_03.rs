// use std::collections::HashMap;

use crate::days::Problem;

pub struct Solution;
use regex::Regex;

impl Solution {
    fn get_mul(line: &str) -> Vec<(&str, &str)> {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        re.captures_iter(line)
            .map(|caps| {
                let (_, [a, b]) = caps.extract();
                (a, b)
            })
            .collect()
    }

    // fn get_mul_conditional(line: &str) -> Matches<'_, '_> {
    //     // /(do(?!n\'t)|^).*mul\((\d+),(\d+)\)/Ugm

    // }

    fn solve_a(&self, input: &str) -> u32 {
        let pairs = Solution::get_mul(input);
        if pairs.len() == 0 {
            return 0;
        }
        pairs.iter().fold(0, |acc, (a, b)| {
            acc + (a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap())
        })
    }

    fn solve_b(&self, input: &str) -> u32 {
        let re = Regex::new(r"(mul\((\d+),(\d+)\)|do\(\)|don\'t\(\))").unwrap();
        let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let matches = re.find_iter(input);
        let mut flag = true;
        let mut result = 0;
        for m in matches {
            if m.as_str() == "do()" {
                flag = true;
            } else if m.as_str() == "don't()" {
                flag = false;
            } else {
                let caps = mul_re.captures(m.as_str()).unwrap();
                let (_, [a, b]) = caps.extract();
                let a = a.parse::<u32>().unwrap();
                let b = b.parse::<u32>().unwrap();
                if flag {
                    result += a * b;
                }
            }
        }
        result
    }
}

impl Problem for Solution {
    fn part_one(&self) -> String {
        let input = self.read_file("data/day03/data.txt").unwrap();
        self.solve_a(&input).to_string()
    }

    fn part_two(&self) -> String {
        let input = self.read_file("data/day03/data.txt").unwrap();
        self.solve_b(&input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let solution = Solution {};
        let input = solution.read_file("data/day03/test.txt").unwrap();
        let result = solution.solve_a(&input);
        assert_eq!(result, 161);
    }

    #[test]
    fn test_b() {
        let solution = Solution {};
        let input = solution.read_file("data/day03/test.txt").unwrap();
        let result = solution.solve_b(&input);
        assert_eq!(result, 48);
    }
}