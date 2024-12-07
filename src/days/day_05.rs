// use std::collections::HashMap;

use std::collections::{HashMap, HashSet};

use crate::Problem;

pub struct Solution;

impl Solution {
    fn parse_input(input: &str) -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>) {
        let (rules, pages) = input.split_once("\n\n").unwrap();
        let mut ruleset: HashMap<u32, HashSet<u32>> = HashMap::new();

        rules.lines().for_each(|line| {
            let (a,b) = line.split_once("|").unwrap();
            ruleset.entry(a.parse::<u32>().unwrap()).or_default().insert(b.parse::<u32>().unwrap());
        });
        
        let parsed_pages = pages.lines().map(|line| line.split(',').map(|e| e.parse::<u32>().unwrap()).collect()).collect();

        (ruleset, parsed_pages)
    }

    fn check_pages(pages: &Vec<u32>, ruleset: &HashMap<u32, HashSet<u32>>) -> bool {
        let mut visited_pages = HashSet::<u32>::new();
        for page in pages {
            if ruleset.get(&page).unwrap_or(&HashSet::<u32>::new()).intersection(&visited_pages).count() > 0 {
                return false;
            }
            visited_pages.insert(*page);
        }
        true
    }

    fn solve_a(&self, input: &str) -> u32 {
        let (ruleset, pages) = Solution::parse_input(input);
        
        pages.into_iter().map(|line| {
            if Solution::check_pages(&line, &ruleset) {
                // Get middle value
                let len = line.len();
                *line.get((len-1)/2).unwrap()
            } else {
                0
            }
        }).sum()
    }

    fn solve_b(&self, input: &str) -> u32 {
        let (ruleset, pages) = Solution::parse_input(input);
        let bad_pages:Vec<Vec<u32>> = pages.into_iter().filter(|line| !Solution::check_pages(&line, &ruleset)).collect();
        bad_pages.into_iter().map(|line| {
            let mut line = line.clone();
            line.sort_by(|a,b| {
                if ruleset.get(&a).unwrap_or(&HashSet::<u32>::new()).contains(&b) {
                    std::cmp::Ordering::Less
                } else if ruleset.get(&b).unwrap_or(&HashSet::<u32>::new()).contains(&a) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            // Get middle value
            let len = line.len();
            *line.get((len-1)/2).unwrap()
        }).sum()
    }
}

impl Problem for Solution {
    fn part_one(&self, test: bool) -> String {
        let file_path = if test {
            "data/day05/test.txt"
        } else {
            "data/day05/data.txt"
        };
        let input = self.read_file(file_path).unwrap();
        self.solve_a(&input).to_string()
    }

    fn part_two(&self, test: bool) -> String {
        let file_path = if test {
            "data/day05/test.txt"
        } else {
            "data/day05/data.txt"
        };
        let input = self.read_file(file_path).unwrap();
        self.solve_b(&input).to_string()
    }

    fn add_to_registry(self, registry: &mut crate::Registry) {
        registry.register(5, Box::new(self));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let solution = Solution {};
        let input = solution.read_file("data/day05/test.txt").unwrap();
        let result = solution.solve_a(&input);
        assert_eq!(result, 143);
    }
    
    #[test]
    fn test_b() {
        let solution = Solution {};
        let input = solution.read_file("data/day05/test.txt").unwrap();
        let result = solution.solve_b(&input);
        assert_eq!(result, 123);
    }
}

