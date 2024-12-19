use crate::{days::Problem, utils::world::World};
use priority_queue::DoublePriorityQueue;

pub struct Solution;

fn parse_into_grid(
    input: &str,
    num_bytes: usize,
    grid_size: usize,
) -> (World<char>, (usize, usize)) {
    let mut grid = vec![vec![' '; grid_size]; grid_size];
    let mut count = 0;
    let mut last_byte = (0,0);
    for line in input.lines() {
        let (x, y) = line.split_once(",").unwrap();
        let (x, y): (usize, usize) = (x.parse().unwrap(), y.parse().unwrap());
        last_byte = (x,y);
        grid[y][x] = '#';

        count += 1;
        if count == num_bytes {
            return (
                World {
                    map: grid,
                    width: grid_size,
                    height: grid_size,
                },
                last_byte,
            );
        }
    }
    (
        World {
            map: grid,
            width: grid_size,
            height: grid_size,
        },
        last_byte,
    )
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
struct SearchState(usize, usize);

impl SearchState {
    fn go(self, direction: Direction) -> SearchState {
        match direction {
            Direction::North => SearchState(self.0 - 1, self.1),
            Direction::West => SearchState(self.0, self.1 - 1),
            Direction::South => SearchState(self.0 + 1, self.1),
            Direction::East => SearchState(self.0, self.1 + 1),
        }
    }
}

fn find_shortest_path(map: &World<char>, initial_state: SearchState) -> Option<u32> {
    let mut queue = DoublePriorityQueue::new();
    // Build out initial set of states
    for (i, row) in map.map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell != '#' {
                queue.push(SearchState(i, j), u32::MAX);
                queue.push(SearchState(i, j), u32::MAX);
                queue.push(SearchState(i, j), u32::MAX);
                queue.push(SearchState(i, j), u32::MAX);
            }
        }
    }
    queue.change_priority(&initial_state, 0);

    while let Some((search_state, score)) = queue.pop_min() {
        // Check if we are at the end
        if search_state.0 == map.height - 1 && search_state.1 == map.width - 1 {
            return Some(score);
        }
        if score == u32::MAX {
            break;
        }

        // Update scores for adjacent nodes
        if search_state.0 > 0 {
            let go_north_state = search_state.go(Direction::North);
            if let Some(p) = queue.get_priority(&go_north_state) {
                queue.change_priority(&go_north_state, u32::min(score + 1, *p));
            }
        }
        if search_state.0 < map.height - 1 {
            let go_south_state = search_state.go(Direction::South);
            if let Some(p) = queue.get_priority(&go_south_state) {
                queue.change_priority(&go_south_state, u32::min(score + 1, *p));
            }
        }
        if search_state.1 > 0 {
            let go_west_state = search_state.go(Direction::West);
            if let Some(p) = queue.get_priority(&go_west_state) {
                queue.change_priority(&go_west_state, u32::min(score + 1, *p));
            }
        }
        if search_state.1 < map.width - 1 {
            let go_east_state = search_state.go(Direction::East);
            if let Some(p) = queue.get_priority(&go_east_state) {
                queue.change_priority(&go_east_state, u32::min(score + 1, *p));
            }
        }
    }
    None // No path was found
}

impl Solution {
    fn solve_a(&self, input: &str, num_bytes: usize, grid_size: usize) -> u32 {
        let (grid, _last_byte) = parse_into_grid(input, num_bytes, grid_size);
        find_shortest_path(&grid, SearchState(0, 0)).unwrap()
    }
    
    fn solve_b(&self, input: &str, starting_num_bytes: usize, grid_size: usize) -> (usize, usize) {
        let mut num_bytes_left = starting_num_bytes;
        let mut num_bytes_right = input.split("\n").count();
        
        loop {
            let num_bytes = (num_bytes_left + num_bytes_right) / 2;
            let (grid, _) = parse_into_grid(input,num_bytes, grid_size);
            let shortest_path_distance = find_shortest_path(&grid, SearchState(0, 0));
            match shortest_path_distance {
                Some(_) => {
                    num_bytes_left = num_bytes;
                }
                None => {
                    num_bytes_right = num_bytes;
                }
            }
            if num_bytes_right - num_bytes_left == 1 {
                let (_, last_byte) = parse_into_grid(input,num_bytes_right, grid_size);
                return last_byte;
            }


        }
    }
}

impl Problem for Solution {
    fn part_one(&self) -> String {
        let input = self.read_file("data/day18/data.txt").unwrap();
        self.solve_a(&input, 1024, 71).to_string()
    }

    fn part_two(&self) -> String {
        let input = self.read_file("data/day18/data.txt").unwrap();
        let (x,y) = self.solve_b(&input, 1024, 71);
        format!("{},{}", x,y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let solution = Solution {};
        let input = solution.read_file("data/day18/test.txt").unwrap();
        let result = solution.solve_a(&input, 12, 7);
        assert_eq!(result, 22);
    }

    #[test]
    fn test_b() {
        let solution = Solution {};
        let input = solution.read_file("data/day18/test.txt").unwrap();
        let result = solution.solve_b(&input, 12, 7);
        assert_eq!(result, (6,1));
    }
}
