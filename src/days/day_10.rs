use std::collections::HashSet;

use itertools::enumerate;

use crate::{
    days::Problem,
    utils::{grid::parse_into_grid, point::Point},
};

pub struct Solution;

type Map = Vec<Vec<u8>>;

fn size(map: &Map) -> (usize, usize) {
    (map.len(), map[0].len())
}

fn score_trailhead(map: &Map, start: Point<usize>) -> u32 {
    let mut queue: Vec<Point<usize>> = vec![start];
    let (num_rows, num_cols) = size(&map);
    let mut ends: HashSet<Point<usize>> = HashSet::new();
    while let Some(p) = queue.pop() {
        let current_height = map[p.0 as usize][p.1 as usize];
        if current_height == 9 {
            ends.insert(p);
        } else {
            // check each directions
            // North
            if p.0 > 0 && map[p.0 - 1][p.1] == current_height + 1 {
                queue.push(Point(p.0 - 1, p.1));
            }
            // East
            if p.1 < num_cols - 1 && map[p.0][p.1 + 1] == current_height + 1 {
                queue.push(Point(p.0, p.1 + 1));
            }
            // South
            if p.0 < num_rows - 1 && map[p.0 + 1][p.1] == current_height + 1 {
                queue.push(Point(p.0 + 1, p.1));
            }
            // West
            if p.1 > 0 && map[p.0][p.1 - 1] == current_height + 1 {
                queue.push(Point(p.0, p.1 - 1));
            }
        }
    }
    ends.len() as u32
}

fn rate_trailhead(map: &Map, start: Point<usize>) -> u32 {
    let mut queue: Vec<Point<usize>> = vec![start];
    let (num_rows, num_cols) = size(&map);
    let mut rating = 0;    
    while let Some(p) = queue.pop() {
        let current_height = map[p.0 as usize][p.1 as usize];
        if current_height == 9 {
            rating += 1;
        } else {
            // check each directions
            // North
            if p.0 > 0 && map[p.0 - 1][p.1] == current_height + 1 {
                queue.push(Point(p.0 - 1, p.1));
            }
            // East
            if p.1 < num_cols - 1 && map[p.0][p.1 + 1] == current_height + 1 {
                queue.push(Point(p.0, p.1 + 1));
            }
            // South
            if p.0 < num_rows - 1 && map[p.0 + 1][p.1] == current_height + 1 {
                queue.push(Point(p.0 + 1, p.1));
            }
            // West
            if p.1 > 0 && map[p.0][p.1 - 1] == current_height + 1 {
                queue.push(Point(p.0, p.1 - 1));
            }
        }
    }
    rating
}

impl Solution {
    fn solve_a(&self, input: &str) -> u32 {
        let grid = parse_into_grid::<u8>(input).unwrap();
        let mut total_score = 0;
        for (i, row) in enumerate(&grid) {
            for (j, point) in enumerate(row) {
                if *point == 0 {
                    total_score += score_trailhead(&grid, Point(i, j));
                }
            }
        }
        total_score
    }
    
    fn solve_b(&self, input: &str) -> u32 {
        let grid = parse_into_grid::<u8>(input).unwrap();
        let mut total_score = 0;
        for (i, row) in enumerate(&grid) {
            for (j, point) in enumerate(row) {
                if *point == 0 {
                    total_score += rate_trailhead(&grid, Point(i, j));
                }
            }
        }
        total_score
    }
}

impl Problem for Solution {
    fn part_one(&self) -> String {
        let input = self.read_file("data/day10/data.txt").unwrap();
        self.solve_a(&input).to_string()
    }

    fn part_two(&self) -> String {
        let input = self.read_file("data/day10/data.txt").unwrap();
        self.solve_b(&input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let solution = Solution {};
        let input = solution.read_file("data/day10/test.txt").unwrap();
        let result = solution.solve_a(&input);
        assert_eq!(result, 36);
    }

    #[test]
    fn test_b() {
        let solution = Solution {};
        let input = solution.read_file("data/day10/test.txt").unwrap();
        let result = solution.solve_b(&input);
        assert_eq!(result, 81);
    }
}
