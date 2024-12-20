use std::collections::HashMap;

use priority_queue::DoublePriorityQueue;
// use itertools::Itertools;

use crate::{days::Problem, utils::world::World};

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Hash, Clone, Copy, PartialEq, Eq, Debug)]
struct Coordinates(isize, isize);

impl Coordinates {
    fn go(self, direction: Direction) -> Coordinates {
        match direction {
            Direction::North => Coordinates(self.0 - 1, self.1),
            Direction::West => Coordinates(self.0, self.1 - 1),
            Direction::South => Coordinates(self.0 + 1, self.1),
            Direction::East => Coordinates(self.0, self.1 + 1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Cheat {
    start: Coordinates,
    end: Coordinates,
}

fn dijkstra(map: &World<char>, initial_state: Coordinates) -> Vec<Vec<Option<u32>>> {
    let mut queue = DoublePriorityQueue::new();
    // Build out initial set of states
    for (i, row) in map.map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell != '#' {
                queue.push(Coordinates(i as isize, j as isize), u32::MAX);
            }
        }
    }
    queue.change_priority(&initial_state, 0);
    // let mut scores: HashMap<Coordinates, u32> = HashMap::new();
    let mut scores_2: Vec<Vec<Option<u32>>> = vec![vec![None; map.height]; map.width];

    while let Some((search_state, score)) = queue.pop_min() {
        // scores.insert(search_state, score);
        scores_2[search_state.0 as usize][search_state.1 as usize] = Some(score);
        // Update scores for adjacent nodes
        if search_state.0 > 0 {
            let go_north_state = search_state.go(Direction::North);
            if let Some(p) = queue.get_priority(&go_north_state) {
                queue.change_priority(&go_north_state, u32::min(score + 1, *p));
            }
        }
        if search_state.0 < map.height as isize - 1 {
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
        if search_state.1 < map.width as isize - 1 {
            let go_east_state = search_state.go(Direction::East);
            if let Some(p) = queue.get_priority(&go_east_state) {
                queue.change_priority(&go_east_state, u32::min(score + 1, *p));
            }
        }
    }
    scores_2
}

fn find_shortcuts(
    map: &World<char>,
    distances: &Vec<Vec<Option<u32>>>,
    cheat_duration: isize,
) -> HashMap<Cheat, u32> {
    let mut cheats = HashMap::new();

    for (i, row) in map.map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if *cell == '#' {
                continue;
            }
            let i = i as isize;
            let j = j as isize;
            let c1 = Coordinates(i, j);
            let s1 = distances[i as usize][j as usize].unwrap();

            for m in i - cheat_duration..i + cheat_duration {
                for n in j - cheat_duration..j + cheat_duration {
                    if m < 0 || m >= map.height as isize || n < 0 || n >= map.width as isize {
                        continue;
                    }
                    if let Some(s2) = distances[m as usize][n as usize] {
                        let c2 = Coordinates(m, n);
                        let dist = ((c1.0 - c2.0).abs() + (c1.1 - c2.1).abs()) as u32;
                        if dist > cheat_duration as u32 {
                            continue;
                        }
                        if s1 > s2 + dist {
                            cheats.insert(Cheat { start: c1, end: c2 }, s1 - s2 - dist);
                        } else if s2 > s1 + dist {
                            cheats.insert(Cheat { start: c2, end: c1 }, s2 - s1 - dist);
                        }
                    };
                }
            }
        }
    }

    // for pair in distances.iter().combinations(2) {
    //     let ((c1, s1), (c2, s2)) = (pair[0], pair[1]);
    //     let dist =( (c1.0 - c2.0).abs() + (c1.1 - c2.1).abs()) as u32;
    //     if dist > cheat_duration {
    //         continue;
    //     }
    //     if *s1 > *s2 + dist {
    //         cheats.insert(Cheat {start: *c1, end: *c2}, s1-s2-dist);
    //     } else if *s2 > *s1 + dist {
    //         cheats.insert(Cheat {start: *c2, end: *c1}, s2-s1-dist);
    //     }

    // };
    cheats
}

fn parse_map(input: &str) -> (World<char>, Coordinates, Coordinates) {
    let mut grid = Vec::new();
    let mut start_state: Coordinates = Coordinates(0, 0);
    let mut end_state: Coordinates = Coordinates(0, 0);
    for (i, line) in input.lines().enumerate() {
        let mut row: Vec<char> = Vec::new();
        for (j, c) in line.chars().enumerate() {
            if let Ok(c) = c.to_string().parse::<char>() {
                if c == 'S' {
                    start_state.0 = i as isize;
                    start_state.1 = j as isize;
                }
                if c == 'E' {
                    end_state.0 = i as isize;
                    end_state.1 = j as isize;
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
        end_state,
    )
}

pub struct Solution;

impl Solution {
    fn solve_a(&self, input: &str) -> u32 {
        let (map, _initial_state, end_state) = parse_map(input);
        let distances_from_end = dijkstra(&map, end_state);
        let shortcuts = find_shortcuts(&map, &distances_from_end, 2);

        // println!("{:?}", shortcuts);
        // let mut count = HashMap::new();
        // for savings in shortcuts.values() {
        //     count.entry(savings).and_modify(|counter| *counter += 1).or_insert(1);
        // }
        // println!("{:?}", count);

        shortcuts.values().filter(|v| **v >= 100).count() as u32
    }

    fn solve_b(&self, input: &str) -> u32 {
        let (map, _initial_state, end_state) = parse_map(input);
        let distances_from_end = dijkstra(&map, end_state);
        let shortcuts = find_shortcuts(&map, &distances_from_end, 20);
        shortcuts.values().filter(|v| **v >= 100).count() as u32
    }
}

impl Problem for Solution {
    fn part_one(&self) -> String {
        let input = self.read_file("data/day20/data.txt").unwrap();
        self.solve_a(&input).to_string()
    }

    fn part_two(&self) -> String {
        let input = self.read_file("data/day20/data.txt").unwrap();
        self.solve_b(&input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let solution = Solution {};
        let input = solution.read_file("data/day20/test.txt").unwrap();
        let result = solution.solve_a(&input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_b() {
        let solution = Solution {};
        let input = solution.read_file("data/day20/test.txt").unwrap();
        let result = solution.solve_b(&input);
        assert_eq!(result, 0);
    }
}
