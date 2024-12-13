use std::collections::HashSet;

use crate::{
    days::Problem,
    utils::{
        grid::parse_into_grid,
        point::{Direction, Point},
    },
};

type Region = HashSet<Point<i64>>;
type Map = Vec<Vec<char>>;

fn compute_perimeter(region: &Region) -> u32 {
    let mut perimeter: u32 = 0;
    for point in region {
        if !region.contains(&(*point + Direction::UP)) {
            perimeter += 1;
        }
        if !region.contains(&(*point + Direction::DOWN)) {
            perimeter += 1;
        }
        if !region.contains(&(*point + Direction::LEFT)) {
            perimeter += 1;
        }
        if !region.contains(&(*point + Direction::RIGHT)) {
            perimeter += 1;
        }
    }
    perimeter
}

fn compute_sides(region: &Region) -> u32 {
    let mut sides: u32 = 0;
    for point in region {
        if !region.contains(&(*point + Direction::UP)) {
            // Up is not in region. Need to check if left is in region and up-left is not in region
            if !region.contains(&(*point + Direction::LEFT)) || ( region.contains(&(*point + Direction::LEFT)) && region.contains(&(*point + Direction::UP + Direction::LEFT)) ) {
                sides += 1;
            }
        }
        if !region.contains(&(*point + Direction::DOWN)) {
            if !region.contains(&(*point + Direction::RIGHT)) || (  region.contains(&(*point + Direction::RIGHT)) && region.contains(&(*point + Direction::DOWN + Direction::RIGHT)) ){
                sides += 1;
            }
        }
        if !region.contains(&(*point + Direction::LEFT)) {
            if !region.contains(&(*point + Direction::DOWN)) || ( region.contains(&(*point + Direction::DOWN)) && region.contains(&(*point + Direction::LEFT + Direction::DOWN))) {
                sides += 1;
            }
        }
        if !region.contains(&(*point + Direction::RIGHT)) {
            if !region.contains(&(*point + Direction::UP)) || ( region.contains(&(*point + Direction::UP)) && region.contains(&(*point + Direction::RIGHT + Direction::UP))) {
                sides += 1;
            }
        }
    }
    sides
}

fn size(map: &Map) -> (usize, usize) {
    (map.len(), map[0].len())
}

fn find_grid(grid: &Map, init: Point<i64>) -> Region {
    let region_code = grid[init.0 as usize][init.1 as usize];
    let mut region: Region = HashSet::new();
    let mut queue: Vec<Point<i64>> = vec![init];
    let (num_rows, num_cols) = size(&grid);

    while let Some(p) = queue.pop() {
        region.insert(p);
        // check each directions
        // North
        if p.0 > 0
            && grid[(p.0 - 1) as usize][p.1 as usize] == region_code
            && !region.contains(&Point(p.0 - 1, p.1))
        {
            queue.push(Point(p.0 - 1, p.1));
        }
        // East
        if p.1 < num_cols as i64 - 1
            && grid[p.0 as usize][(p.1 + 1) as usize] == region_code
            && !region.contains(&Point(p.0, p.1 + 1))
        {
            queue.push(Point(p.0, p.1 + 1));
        }
        // South
        if p.0 < num_rows as i64 - 1
            && grid[(p.0 + 1) as usize][p.1 as usize] == region_code
            && !region.contains(&Point(p.0 + 1, p.1))
        {
            queue.push(Point(p.0 + 1, p.1));
        }
        // West
        if p.1 > 0
            && grid[p.0 as usize][(p.1 - 1) as usize] == region_code
            && !region.contains(&Point(p.0, p.1 - 1))
        {
            queue.push(Point(p.0, p.1 - 1));
        }
    }
    region
}

fn parse_into_regions(input: &str) -> Vec<Region> {
    let mut regions = Vec::new();
    let mut visited: HashSet<Point<i64>> = HashSet::new();

    let grid = parse_into_grid::<char>(input).unwrap();

    for (i, row) in grid.iter().enumerate() {
        for (j, _cell) in row.iter().enumerate() {
            let p = Point(i as i64, j as i64);
            if !visited.contains(&p) {
                let region = find_grid(&grid, p);
                for point in region.clone() {
                    visited.insert(point);
                }
                regions.push(region);
            }
        }
    }

    regions
}

pub struct Solution;

impl Solution {
    fn solve_a(&self, input: &str) -> u32 {
        let regions = parse_into_regions(input);
        regions
            .iter()
            .map(|r| r.len() as u32 * compute_perimeter(r))
            .sum()
        }
        
        fn solve_b(&self, input: &str) -> u32 {
        let regions = parse_into_regions(input);
        regions
            .iter()
            .map(|r| r.len() as u32 * compute_sides(r))
            .sum()
    }
}

impl Problem for Solution {
    fn part_one(&self) -> String {
        let input = self.read_file("data/day12/data.txt").unwrap();
        self.solve_a(&input).to_string()
    }

    fn part_two(&self) -> String {
        let input = self.read_file("data/day12/data.txt").unwrap();
        self.solve_b(&input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measure_perimeter() {
        let mut region: HashSet<Point<i64>> = HashSet::new();
        region.insert(Point(1, 1));
        region.insert(Point(1, 2));
        region.insert(Point(1, 3));
        region.insert(Point(2, 3));
        assert_eq!(compute_perimeter(&region), 10);
    }

    #[test]
    fn test_a() {
        let solution = Solution {};
        let input = solution.read_file("data/day12/test.txt").unwrap();
        let result = solution.solve_a(&input);
        assert_eq!(result, 1930);
    }

    #[test]
    fn test_b() {
        let solution = Solution {};
        let input = solution.read_file("data/day12/test.txt").unwrap();
        let result = solution.solve_b(&input);
        assert_eq!(result, 1206);
    }
}
