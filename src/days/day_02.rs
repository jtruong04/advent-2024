// use std::collections::HashMap;

use crate::Problem;

pub struct Solution;

impl Solution {
    fn is_safe(levels: &Vec<i8>) -> bool {
        let differences:Vec<i8>= levels.windows(2).map(|pair| pair[0] - pair[1]).collect();
        (differences.iter().all(|f| *f < 0) || differences.iter().all(|f| *f > 0)) && differences.iter().all(|f| f.abs() > 0 && f.abs() <= 3)
    }

    fn is_safe_b(levels: &Vec<i8>) -> bool {
        if Solution::is_safe(&levels) {
            return true;
        }
        for i in 0..levels.len() {
            let mut levels_less_one = levels.clone();
            levels_less_one.remove(i);
            if Solution::is_safe(&levels_less_one) {
                return true;
            }
        }
        false
    }

    fn solve_a(&self, input: &str) -> u32 {
        input
            .lines()
            .map(|line| {
                Solution::is_safe(
                    &line.split_ascii_whitespace()
                        .map(|num| num.parse::<i8>().unwrap())
                        .collect(),
                )
            })
            .filter(|val| *val)
            .count() as u32
    }

    fn solve_b(&self, input: &str) -> u32 {
        input
            .lines()
            .map(|line| {
                Solution::is_safe_b(
                    &line.split_ascii_whitespace()
                        .map(|num| num.parse::<i8>().unwrap())
                        .collect(),
                )
            })
            .filter(|val| *val)
            .count() as u32
    }
}

impl Problem for Solution {
    fn part_one(&self, test: bool) -> String {
        let file_path = if test {
            "data/day02/test.txt"
        } else {
            "data/day02/data.txt"
        };
        let input = self.read_file(file_path).unwrap();
        self.solve_a(&input).to_string()
    }

    fn part_two(&self, test: bool) -> String {
        let file_path = if test {
            "data/day02/test.txt"
        } else {
            "data/day02/data.txt"
        };
        let input = self.read_file(file_path).unwrap();
        self.solve_b(&input).to_string()
    }

    fn add_to_registry(self, registry: &mut crate::Registry) {
        registry.register(2, Box::new(self));
    }
}
