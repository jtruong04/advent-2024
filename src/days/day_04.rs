// use std::collections::HashMap;

use crate::Problem;

pub struct Solution;
use regex::Regex;

impl Solution {
    fn split(grid: &str) -> Vec<Vec<char>> {
        grid.lines().map(|line| line.chars().collect()).collect()
    }

    fn unroll_grid(grid: Vec<Vec<char>>) -> Result<Vec<String>, String> {
        // Take a word grid and convert it into a vector of strs
        // each str being a row, column, +diagonal, -diagonal

        let num_rows = grid.len();
        if num_rows == 0 {
            return Err("Empty grid".into());
        }
        let num_cols = grid[0].len();
        if num_cols == 0 {
            return Err("Empty grid".into());
        }

        let mut rows: Vec<String> = vec!["".into(); num_rows];
        let mut cols: Vec<String> = vec!["".into(); num_cols];
        let mut diag_pos: Vec<String> = vec!["".into(); num_rows + num_cols - 1];
        let mut diag_neg: Vec<String> = vec!["".into(); num_rows + num_cols - 1];

        for i in 0..num_rows {
            for j in 0..num_cols {
                let ch = grid[i][j];
                rows[i].push(ch);
                cols[j].push(ch);
                diag_pos[i + j].push(ch);
                diag_neg
                    [(i as i32 - j as i32).rem_euclid((num_rows + num_cols - 1) as i32) as usize]
                    .push(ch);
            }
        }
        rows.append(&mut cols);
        rows.append(&mut diag_pos);
        rows.append(&mut diag_neg);

        Ok(rows)
    }

    fn solve_a(&self, input: &str) -> u32 {
        let unrolled_grid = Solution::unroll_grid(Solution::split(input)).unwrap();
        // println!("{:?}", unrolled_grid);
        let re1 = Regex::new(r"XMAS").unwrap();
        let re2 = Regex::new(r"SAMX").unwrap();

        unrolled_grid
            .into_iter()
            .map(|line| re1.find_iter(&line).count() as u32 + re2.find_iter(&line).count() as u32 )
            .sum()
    }

    fn solve_b(&self, input: &str) -> u32 {
        let grid = Solution::split(input);
        let num_rows = grid.len();
        let num_cols = grid[0].len();
        let mut count = 0;
        for i in 1..num_rows-1 {
            for j in 1..num_cols-1 {
                if grid[i][j] == 'A' {
                    if (grid[i-1][j-1] == 'M' && grid[i+1][j+1] == 'S') || (grid[i-1][j-1] == 'S' && grid[i+1][j+1] == 'M') {
                        if (grid[i-1][j+1] == 'M' && grid[i+1][j-1] == 'S') || (grid[i-1][j+1] == 'S' && grid[i+1][j-1] == 'M') {
                            count += 1;
                        }
                    } 
                }
            }
        }
        count
    }
}

impl Problem for Solution {
    fn part_one(&self, test: bool) -> String {
        let file_path = if test {
            "data/day04/test.txt"
        } else {
            "data/day04/data.txt"
        };
        let input = self.read_file(file_path).unwrap();
        self.solve_a(&input).to_string()
    }

    fn part_two(&self, test: bool) -> String {
        let file_path = if test {
            "data/day04/test.txt"
        } else {
            "data/day04/data.txt"
        };
        let input = self.read_file(file_path).unwrap();
        self.solve_b(&input).to_string()
    }

    fn add_to_registry(self, registry: &mut crate::Registry) {
        registry.register(4, Box::new(self));
    }
}
