use std::{collections::{HashMap, HashSet}, u32::{self, MAX}};

use priority_queue::DoublePriorityQueue;

use crate::{days::Problem, utils::world::World};

pub struct Solution;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn_left(self) -> Direction {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }
    fn turn_right(self) -> Direction {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
struct SearchState(usize, usize, Direction);

impl SearchState {
    fn go_forward(self) -> SearchState {
        match self.2 {
            Direction::North => SearchState(self.0 - 1, self.1, self.2),
            Direction::West => SearchState(self.0, self.1 - 1, self.2),
            Direction::South => SearchState(self.0 + 1, self.1, self.2),
            Direction::East => SearchState(self.0, self.1 + 1, self.2),
        }
    }
    fn turn_left(self) -> SearchState {
        SearchState(self.0, self.1, self.2.turn_left())
    }
    fn turn_right(self) -> SearchState {
        SearchState(self.0, self.1, self.2.turn_right())
    }
}

fn find_shortest_path(map: &World<char>, initial_state: SearchState, max_dist: u32) -> Option<u32> {
    let mut queue = DoublePriorityQueue::new();
    // Build out initial set of states
    for (i, row) in map.map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell != '#' {
                queue.push(SearchState(i, j, Direction::North), u32::MAX);
                queue.push(SearchState(i, j, Direction::South), u32::MAX);
                queue.push(SearchState(i, j, Direction::East), u32::MAX);
                queue.push(SearchState(i, j, Direction::West), u32::MAX);
            }
        }
    }
    queue.change_priority(&initial_state, 0);

    while let Some((search_state, score)) = queue.pop_min() {
        // Check if we are at the end
        if map.map[search_state.0][search_state.1] == 'E' {
            return Some(score);
        }
        if score > max_dist {
            continue;
        }
        // Update scores for adjacent nodes
        // Turn left
        let turn_left_state = search_state.turn_left();
        if let Some(p) = queue.get_priority(&turn_left_state) {
            queue.change_priority(&turn_left_state, u32::min(score + 1000, *p));
        }
        // Turn right
        let turn_right_state = search_state.turn_right();
        if let Some(p) = queue.get_priority(&turn_right_state) {
            queue.change_priority(&turn_right_state, u32::min(score + 1000, *p));
        }
        // Go forward
        let go_forward_state = search_state.go_forward();
        if let Some(p) = queue.get_priority(&go_forward_state) {
            queue.change_priority(&go_forward_state, u32::min(score + 1, *p));
        }
    }
    None // No path was found
}

fn find_shortest_path_from_e_to_all(map: &World<char>, initial_state: SearchState, max_dist: u32) -> HashMap<SearchState, u32> {
    let mut queue = DoublePriorityQueue::new();
    let mut max_dist = max_dist;
    // Build out initial set of states
    for (i, row) in map.map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell != '#' {
                queue.push(SearchState(i, j, Direction::North), u32::MAX);
                queue.push(SearchState(i, j, Direction::South), u32::MAX);
                queue.push(SearchState(i, j, Direction::East), u32::MAX);
                queue.push(SearchState(i, j, Direction::West), u32::MAX);
            }
        }
    }
    queue.change_priority(&initial_state, 0);
    let mut scores: HashMap<SearchState, u32> = HashMap::new();
    while let Some((search_state, score)) = queue.pop_min() {
        // Check if we are at the end
        if map.map[search_state.0][search_state.1] == 'E' {
            max_dist = score;
        }
        if score > max_dist {
            continue;
        }
        scores.insert(search_state, score);
        // Update scores for adjacent nodes
        // Turn left
        let turn_left_state = search_state.turn_left();
        if let Some(p) = queue.get_priority(&turn_left_state) {
            queue.change_priority(&turn_left_state, u32::min(score + 1000, *p));
        }
        // Turn right
        let turn_right_state = search_state.turn_right();
        if let Some(p) = queue.get_priority(&turn_right_state) {
            queue.change_priority(&turn_right_state, u32::min(score + 1000, *p));
        }
        // Go forward
        let go_forward_state = search_state.go_forward();
        if let Some(p) = queue.get_priority(&go_forward_state) {
            queue.change_priority(&go_forward_state, u32::min(score + 1, *p));
        }
    }
    scores
}

fn parse_map(input: &str) -> (World<char>, SearchState) {
    let mut grid = Vec::new();
    let mut start_state: SearchState = SearchState(0, 0, Direction::East);
    for (i, line) in input.lines().enumerate() {
        let mut row: Vec<char> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            if let Ok(c) = c.to_string().parse::<char>() {
                if c ==        // let mut count = 0;
                'S' {
                    start_state.0 = i;
                    start_state.1 = j;
                }
                row.push(c);
            }
        }
        grid.push(row);
    }
    (
        World::<char> {
            height: grid.len(),
            width: if grid.len() > 0 { grid[0].len() } else { 0 },
            map: grid,
        },
        start_state,
    )
}

impl Solution {
    fn solve_a(&self, input: &str) -> u32 {
        let (map, initial_state) = parse_map(input);
        find_shortest_path(&map, initial_state, MAX).unwrap()
    }
    
    fn solve_b(&self, input: &str) -> u32 {
        let dist_s_to_e = self.solve_a(input);

        let (map, initial_state) = parse_map(input);
        // Get shortest distance from S to any position
        let shortest_dist_map = find_shortest_path_from_e_to_all(&map, initial_state, dist_s_to_e);
        let mut viewing_spots: HashSet<(usize, usize)> = HashSet::new(); // Only check position so we don't double count smae spot but two directions.
        // For each location find shortest distance from it to E
        for (k, dist_s_to_b) in shortest_dist_map.iter() {
            if *dist_s_to_b > dist_s_to_e {
                continue;
            }
            if let Some(dist_b_to_e) = find_shortest_path(&map, *k, dist_s_to_e-*dist_s_to_b) {
                if  dist_b_to_e <= dist_s_to_e-dist_s_to_b {
                    viewing_spots.insert((k.0, k.1));
                }
            }
        }
        viewing_spots.len() as u32
    }
}

impl Problem for Solution {
    fn part_one(&self) -> String {
        let input = self.read_file("data/day16/data.txt").unwrap();
        self.solve_a(&input).to_string()
    }

    fn part_two(&self) -> String {
        let input = self.read_file("data/day16/data.txt").unwrap();
        self.solve_b(&input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let solution = Solution {};
        let input = solution.read_file("data/day16/test.txt").unwrap();
        let result = solution.solve_a(&input);
        assert_eq!(result, 7036);
    }

    #[test]
    fn test_a2() {
        let solution = Solution {};
        let input = solution.read_file("data/day16/test2.txt").unwrap();
        let result = solution.solve_a(&input);
        assert_eq!(result, 11048);
    }

    #[test]
    fn test_b() {
        let solution = Solution {};
        let input = solution.read_file("data/day16/test.txt").unwrap();
        let result = solution.solve_b(&input);
        assert_eq!(result, 45);
    }

    #[test]
    fn test_b2() {
        let solution = Solution {};
        let input = solution.read_file("data/day16/test2.txt").unwrap();
        let result = solution.solve_b(&input);
        assert_eq!(result, 64);
    }
}
