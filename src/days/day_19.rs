use std::{cell::RefCell, collections::HashMap};

use crate::days::Problem;

pub struct Solution;


fn can_be_made<'a>(towel: &'a str, patterns: &Vec<&str>, cache: &'a RefCell< HashMap<&'a str, bool>>) -> bool {
    if let Some(c) = cache.borrow().get(towel) {
        return *c;
    }
    for pattern in patterns {
        if *pattern == towel {
            cache.borrow_mut().insert(towel, true);
            return true;
        }
        if let Some(substring) = towel.strip_prefix(pattern) {
            if can_be_made(substring, patterns, cache) {
                cache.borrow_mut().insert(towel, true);
                return true;
            }
        }
    }
    cache.borrow_mut().insert(towel, false);
    false
}

fn count_ways_to_make<'a>(towel: &'a str, patterns: &Vec<&str>, cache: &'a RefCell< HashMap<&'a str, u64>>) -> u64 {
    let mut count = 0;
    if let Some(c) = cache.borrow().get(towel) {
        return *c;
    }
    for pattern in patterns {
        if *pattern == towel {
            count += 1;
        } else if let Some(substring) = towel.strip_prefix(pattern) {
            count += count_ways_to_make(substring, patterns, cache);
        }
    }
    cache.borrow_mut().insert(towel, count);
    count
}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (patterns, towels) = input.split_once("\n\n").unwrap();
    let patterns = patterns.split(", ").collect();
    let towels = towels.lines().collect();
    (patterns, towels)
}

impl Solution {
    fn solve_a(&self, input: &str) -> u64 {
        let (patterns, towels) = parse(input);
        towels.iter().map(|t| can_be_made(t, &patterns, &RefCell::new(HashMap::new()))).filter(|r| *r).count() as u64
    }

    fn solve_b(&self, input: &str) -> u64 {
        let (patterns, towels) = parse(input);
        towels.iter().map(|t| count_ways_to_make(t, &patterns, &RefCell::new(HashMap::new()))).sum()
    }
}

impl Problem for Solution {
    fn part_one(&self) -> String {
        let input = self.read_file("data/day19/data.txt").unwrap();
        self.solve_a(&input).to_string()
    }

    fn part_two(&self) -> String {
        let input = self.read_file("data/day19/data.txt").unwrap();
        self.solve_b(&input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_can_be_made() {
        assert!(can_be_made("brwrr", &vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"], &RefCell::new(HashMap::new())));
    }
    #[test]
    fn test_count_ways_to_make_0() {
        assert_eq!(count_ways_to_make("brwrr", &vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"], &RefCell::new(HashMap::new())), 2);
    }
    #[test]
    fn test_count_ways_to_make_1() {
        assert_eq!(count_ways_to_make("bggr", &vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"], &RefCell::new(HashMap::new())), 1);
    }
    #[test]
    fn test_count_ways_to_make_2() {
        assert_eq!(count_ways_to_make("gbbr", &vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"], &RefCell::new(HashMap::new())), 4);
    }
    #[test]
    fn test_count_ways_to_make_3() {
        assert_eq!(count_ways_to_make("rrbgbr", &vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"], &RefCell::new(HashMap::new())), 6);
    }
    #[test]
    fn test_count_ways_to_make_4() {
        assert_eq!(count_ways_to_make("bbrgwb", &vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"], &RefCell::new(HashMap::new())), 0);
    }

    #[test]
    fn test_cant_be_made() {
        assert!(!can_be_made("bbrgwb", &vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"], &RefCell::new(HashMap::new())));
    }


    #[test]
    fn test_a() {
        let solution = Solution {};
        let input = solution.read_file("data/day19/test.txt").unwrap();
        let result = solution.solve_a(&input);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_b() {
        let solution = Solution {};
        let input = solution.read_file("data/day19/test.txt").unwrap();
        let result = solution.solve_b(&input);
        assert_eq!(result, 16);
    }
}
