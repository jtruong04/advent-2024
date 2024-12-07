use std::collections::HashMap;

use crate::days::Problem;

pub struct Solution;

impl Solution {
    fn solve_a(&self, input: &str) -> u32 {
        let mut col_1 = Vec::<u32>::new();
        let mut col_2 = Vec::<u32>::new();
        input.lines().for_each(|l| {
            let (left, right) = l.split_once("   ").unwrap();
            col_1.push(left.parse::<u32>().unwrap());
            col_2.push(right.parse::<u32>().unwrap());
        });

        col_1.sort();
        col_2.sort();

        col_1
            .iter()
            .zip(col_2.iter())
            .fold(0, |acc, (x, y)| acc + (*x).abs_diff(*y))
    }

    fn solve_b(&self, input: &str) -> u32 {
        let mut col_1 = Vec::<u32>::new();
        let mut col_2 = HashMap::<u32, u32>::new();
        input.lines().for_each(|l| {
            let (left, right) = l.split_once("   ").unwrap();
            col_1.push(left.parse::<u32>().unwrap());
            col_2
                .entry(right.parse::<u32>().unwrap())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        });

        col_1
            .iter()
            .fold(0, |acc, e| acc + (col_2.get(e).unwrap_or(&0) * e))
    }
}

impl Problem for Solution {
    fn part_one(&self) -> String {
        let input = self.read_file("data/day01/data.txt").unwrap();
        self.solve_a(&input).to_string()
    }

    fn part_two(&self) -> String {
        let input = self.read_file("data/day01/data.txt").unwrap();
        self.solve_b(&input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let solution = Solution {};
        let input = solution.read_file("data/day01/test.txt").unwrap();
        let result = solution.solve_a(&input);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_b() {
        let solution = Solution {};
        let input = solution.read_file("data/day01/test.txt").unwrap();
        let result = solution.solve_b(&input);
        assert_eq!(result, 31);
    }
}
