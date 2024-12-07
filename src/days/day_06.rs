use std::collections::HashSet;

use anyhow::{bail, Error};

use crate::days::Problem;

#[derive(Clone, Copy, PartialEq)]
enum State {
    Obstacle,
    Visited(u8),
    Open,
    OutOfBounds,
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn bitmask(self) -> u8 {
        match self {
            Self::Up => 1,
            Self::Right => 2,
            Self::Down => 4,
            Self::Left => 8,
        }
    }
}

#[derive(Clone)]
struct Map {
    grid: Vec<Vec<State>>,
}

impl Map {
    fn count_visited(&self) -> u32 {
        let mut count = 0;
        for row in &self.grid {
            for tile in row {
                if let State::Visited(_) = *tile {
                    count += 1
                }
            }
        }
        count
    }

    fn get_state(&self, row: i32, col: i32) -> Result<State, Error> {
        if row < 0
            || col < 0
            || row >= i32::try_from(self.grid.len())?
            || col >= i32::try_from(self.grid[0].len())?
        {
            return Ok(State::OutOfBounds);
        }
        let row: usize = i32::try_into(row)?;
        let col: usize = i32::try_into(col)?;
        Ok(self.grid[row][col])
    }

    fn visit(&mut self, row: i32, col: i32, direction: &Direction) -> Result<(), Error> {
        if row < 0
            || col < 0
            || row >= i32::try_from(self.grid.len())?
            || col >= i32::try_from(self.grid[0].len())?
        {
            bail!("Out of bounds!");
        }

        let direction_bitmask = direction.bitmask();
        let row: usize = i32::try_into(row)?;
        let col: usize = i32::try_into(col)?;
        if let State::Visited(previous_dir) = self.grid[row][col] {
            self.grid[row][col] = State::Visited(previous_dir | direction_bitmask);
        } else {
            self.grid[row][col] = State::Visited(direction_bitmask);
        }
        Ok(())
    }
}

#[derive(Clone)]
struct Traveler {
    init_row: i32,
    init_col: i32,
    row: i32,
    col: i32,
    direction: Direction,
    loops_found: HashSet<(i32, i32)>,
}

impl Traveler {
    fn patrol(&mut self, map: &mut Map) -> Result<(), Error> {
        while let Ok(current_state) = map.get_state(self.row, self.col) {
            if current_state == State::OutOfBounds {
                break;
            }
            if self.check_for_loop(map) {
                self.loops_found.insert(self.next_step());
            }
            if let State::Visited(visited_directions) = current_state {
                if self.direction.bitmask() & visited_directions > 0 {
                    bail!("Loop detected");
                }
            }
            if let Ok(next_state) = self.look_ahead(map) {
                let _ = map.visit(self.row, self.col, &self.direction);
                match next_state {
                    State::Open | State::Visited(_) | State::OutOfBounds => {
                        self.walk_forward();
                    }
                    State::Obstacle => {
                        self.turn_right();
                    }
                }
            }
        }
        Ok(())
    }

    fn plan_patrol(&mut self, map: &mut Map) -> Result<(), Error> {
        while let Ok(current_state) = map.get_state(self.row, self.col) {
            if current_state == State::OutOfBounds {
                break;
            }
            if let State::Visited(visited_directions) = current_state {
                if self.direction.bitmask() & visited_directions > 0 {
                    bail!("Loop detected");
                }
            }
            if let Ok(next_state) = self.look_ahead(map) {
                let _ = map.visit(self.row, self.col, &self.direction);
                match next_state {
                    State::Open | State::Visited(_) | State::OutOfBounds => {
                        self.walk_forward();
                    }
                    State::Obstacle => {
                        self.turn_right();
                    }
                }
            }
        }
        Ok(())
    }

    fn check_for_loop(&self, map: &mut Map) -> bool {
        let mut clone = self.clone();
        let mut map = map.clone();
        let next = clone.next_step();
        if let Ok(State::Open) = clone.look_ahead(&map) {
            map.grid[next.0 as usize][next.1 as usize] = State::Obstacle;
        }
        if let Err(_) = clone.plan_patrol(&mut map) {
            return true;
        }
        false
    }

    fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }

    fn walk_forward(&mut self) {
        let next_step = self.next_step();
        self.row = next_step.0;
        self.col = next_step.1;
    }

    fn next_step(&self) -> (i32, i32) {
        self.next_step_in_direction(self.direction)
    }

    fn next_step_in_direction(&self, direction: Direction) -> (i32, i32) {
        match direction {
            Direction::Up => (self.row - 1, self.col),
            Direction::Right => (self.row, self.col + 1),
            Direction::Down => (self.row + 1, self.col),
            Direction::Left => (self.row, self.col - 1),
        }
    }

    fn look_ahead(&self, map: &Map) -> Result<State, Error> {
        let next_step = self.next_step();
        map.get_state(next_step.0, next_step.1)
    }
}

impl Default for Traveler {
    fn default() -> Self {
        Traveler {
            row: 0,
            col: 0,
            init_col: 0,
            init_row: 0,
            direction: Direction::Up,
            loops_found: HashSet::new(),
        }
    }
}

fn parse(grid_str: &str) -> Result<(Map, Traveler), anyhow::Error> {
    let mut grid: Vec<Vec<State>> = Vec::new();
    let mut traveler: Traveler = Traveler {
        ..Default::default()
    };
    for row in grid_str.lines() {
        let mut row_vec: Vec<State> = Vec::new();
        for tile in row.chars() {
            match tile {
                '.' => {
                    row_vec.push(State::Open);
                }
                '#' => {
                    row_vec.push(State::Obstacle);
                }
                '^' => {
                    traveler.init_row = i32::try_from(grid.len()).unwrap_or_default();
                    traveler.row = i32::try_from(grid.len()).unwrap_or_default();
                    traveler.init_col = i32::try_from(row_vec.len()).unwrap_or_default();
                    traveler.col = i32::try_from(row_vec.len()).unwrap_or_default();
                    row_vec.push(State::Open);
                }
                _ => {
                    bail!("Invalid character: {}", tile);
                }
            }
        }
        grid.push(row_vec);
    }
    Ok((Map { grid }, traveler))
}

pub struct Solution;

impl Solution {
    fn solve_a(&self, input: &str) -> u32 {
        let (mut grid, mut traveler) = parse(input).unwrap();
        let _ = traveler.patrol(&mut grid);
        grid.count_visited()
    }

    fn solve_b(&self, input: &str) -> u32 {
        let (mut grid, mut traveler) = parse(input).unwrap();
        let _ = traveler.patrol(&mut grid);
        traveler
            .loops_found
            .iter()
            .filter(|cell| cell.0 != traveler.init_row || cell.1 != traveler.init_col)
            .count()
            .try_into()
            .unwrap()
    }
}

impl Problem for Solution {
    fn part_one(&self) -> String {
        let input = self.read_file("data/day06/data.txt").unwrap();
        self.solve_a(&input).to_string()
    }

    fn part_two(&self) -> String {
        let input = self.read_file("data/day06/data.txt").unwrap();
        self.solve_b(&input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let solution = Solution {};
        let input = solution.read_file("data/day06/test.txt").unwrap();
        let result = solution.solve_a(&input);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_b() {
        let solution = Solution {};
        let input = solution.read_file("data/day06/test.txt").unwrap();
        let result = solution.solve_b(&input);
        assert_eq!(result, 6);
    }
}
