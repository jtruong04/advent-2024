use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use crate::{
    days::Problem,
    utils::{gcd, Point},
};

struct City {
    antennas: HashMap<char, HashSet<Point>>,
    width: u32,
    height: u32,
}

impl City {
    fn new(input: &str) -> Self {
        let mut antennas = HashMap::new();
        // Get all antennas and their positions
        for (row, line) in input.lines().enumerate() {
            for (col, cell) in line.chars().enumerate() {
                if cell != '.' {
                    antennas
                        .entry(cell)
                        .or_insert(HashSet::new())
                        .insert(Point(row as i32, col as i32));
                }
            }
        }
        // Get dimensions of the city
        let height = input.lines().count() as u32;
        let width = input.lines().next().unwrap().len() as u32;

        Self {
            antennas,
            width,
            height,
        }
    }

    fn contains(&self, point: &Point) -> bool {
        point.0 >= 0 && point.0 < self.height as i32 && point.1 >= 0 && point.1 < self.width as i32
    }

    fn get_first_order_antinodes(&self) -> HashSet<Point> {
        let mut antinodes = HashSet::new();

        for antennas in self.antennas.values() {
            for pair in antennas.iter().combinations(2) {
                let (a, b) = (*pair[0], *pair[1]);
                let delta = a-b;
                let node_1 = a - delta;
                let node_2 = b + delta;
                if self.contains(&node_1) {
                    antinodes.insert(node_1);
                }
                if self.contains(&node_2) {
                    antinodes.insert(node_2);
                }
            }
        }

        antinodes
    }

    fn get_antinodes(&self) -> HashSet<Point> {
        let mut antinodes = HashSet::<Point>::new();

        for antennas in self.antennas.values() {
            for pair in antennas.iter().combinations(2) {
                let (a, b) = (*pair[0], *pair[1]);
                let delta = a-b;
                let delta = delta / gcd(delta.0, delta.1);

                // Keep subtracting from a until we go off map
                let mut search_node = a;
                while self.contains(&search_node) {
                    antinodes.insert(search_node);
                    search_node = search_node - delta;
                }
                // Now we go in the other direction
                let mut search_node = a + delta; // Add delta just to avoid a single iteration
                while self.contains(&search_node) {
                    antinodes.insert(search_node);
                    search_node = search_node + delta;
                }
            }
        }

        antinodes
    }
}

pub struct Solution;

impl Solution {
    fn solve_a(&self, input: &str) -> u32 {
        let city = City::new(input);
        city.get_first_order_antinodes().len() as u32
    }

    fn solve_b(&self, input: &str) -> u32 {
        let city = City::new(input);
        city.get_antinodes().len() as u32
    }
}

impl Problem for Solution {
    fn part_one(&self) -> String {
        let input = self.read_file("data/day08/data.txt").unwrap();
        self.solve_a(&input).to_string()
    }

    fn part_two(&self) -> String {
        let input = self.read_file("data/day08/data.txt").unwrap();
        self.solve_b(&input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let solution = Solution {};
        let input = solution.read_file("data/day08/test.txt").unwrap();
        let result = solution.solve_a(&input);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_b() {
        let solution = Solution {};
        let input = solution.read_file("data/day08/test.txt").unwrap();
        let result = solution.solve_b(&input);
        assert_eq!(result, 34);
    }
}
