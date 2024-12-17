use std::collections::HashSet;

use crate::{days::Problem, utils::point::Point};

pub struct Solution;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn get_vector(&self, dist: isize) -> Point<isize> {
        match self {
            Direction::UP => Point(-dist, 0),
            Direction::DOWN => Point(dist, 0),
            Direction::LEFT => Point(0, -dist),
            Direction::RIGHT => Point(0, dist),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Robot,
    Box,
    DoubleBox(u8),
    Space,
}

struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn move_robot(&mut self, direction: Direction) {
        // Find robot
        let mut position = Point(0, 0);
        for i in 0..self.tiles.len() {
            for j in 0..self.tiles[i].iter().len() {
                if self.get_tile(Point(i as isize, j as isize)) == Tile::Robot {
                    position = Point(i as isize, j as isize);
                }
            }
        }
        self.move_tile(position, direction);
    }

    fn move_tile(&mut self, position: Point<isize>, direction: Direction) {
        let mut tiles_to_move: HashSet<Point<isize>> = HashSet::new();
        tiles_to_move.insert(position);
        let tile = self.get_tile(position);

        if direction == Direction::UP || direction == Direction::DOWN {
            if tile == Tile::DoubleBox(0) {
                tiles_to_move.insert(position + Point(0, 1));
            }
            if tile == Tile::DoubleBox(1) {
                tiles_to_move.insert(position + Point(0, -1));
            }
        }

        if tiles_to_move.iter().all(|p| self.can_move_from(*p, direction)) {
            for pos in tiles_to_move {
                let current_tile = self.get_tile(pos);
                // Move all target spaces
                let target_tiles = self.get_target_tiles(pos, direction);
                for target_space in target_tiles {
                    self.move_tile(target_space, direction);
                }
                // Move self
                let displacement = direction.get_vector(1);
                self.tiles[(pos.0 + displacement.0) as usize][(pos.1 + displacement.1) as usize] =
                    current_tile;
                self.tiles[(pos.0) as usize][(pos.1) as usize] = Tile::Space;
            }
        }
    }

    fn can_move_into(&self, position: Point<isize>, direction: Direction) -> bool {
        let tile = self.get_tile(position);
        match tile {
            Tile::Space => true,
            Tile::Wall => false,
            Tile::Robot | Tile::Box => self.can_move_from(position, direction),
            Tile::DoubleBox(0) => {
                if direction == Direction::DOWN || direction == Direction::UP {
                    self.can_move_from(position, direction)
                        && self.can_move_from(position + Point(0, 1), direction)
                } else {
                    self.can_move_from(position, direction)
                }
            }
            Tile::DoubleBox(1) => {
                if direction == Direction::DOWN || direction == Direction::UP {
                    self.can_move_from(position, direction)
                        && self.can_move_from(position + Point(0, -1), direction)
                } else {
                    self.can_move_from(position, direction)
                }
            }
            _ => false,
        }
    }

    fn can_move_from(&self, position: Point<isize>, direction: Direction) -> bool {
        let tile = self.get_tile(position);
        match tile {
            Tile::Space => false,
            Tile::Wall => false,
            Tile::Robot | Tile::Box => {
                self.can_move_into(position + direction.get_vector(1), direction)
            }
            Tile::DoubleBox(0) => {
                if direction == Direction::DOWN || direction == Direction::UP {
                    self.can_move_into(position + direction.get_vector(1), direction)
                        && self.can_move_into(
                            position + direction.get_vector(1) + Point(0, 1),
                            direction,
                        )
                } else {
                    self.can_move_into(position + direction.get_vector(1), direction)
                }
            }
            Tile::DoubleBox(1) => {
                if direction == Direction::DOWN || direction == Direction::UP {
                    self.can_move_into(position + direction.get_vector(1), direction)
                        && self.can_move_into(
                            position + direction.get_vector(1) + Point(0, -1),
                            direction,
                        )
                } else {
                    self.can_move_into(position + direction.get_vector(1), direction)
                }
            }
            _ => false,
        }
    }

    fn get_tile(&self, position: Point<isize>) -> Tile {
        *self
            .tiles
            .get(position.0 as usize)
            .unwrap()
            .get(position.1 as usize)
            .unwrap()
    }

    fn get_target_tiles(
        &self,
        position: Point<isize>,
        direction: Direction,
    ) -> HashSet<Point<isize>> {
        let tile = self.get_tile(position + direction.get_vector(1));
        let mut set = HashSet::new();
        match tile {
            Tile::Robot | Tile::Box => {
                set.insert(position + direction.get_vector(1));
            }
            Tile::DoubleBox(x) => {
                set.insert(position + direction.get_vector(1));
                if direction == Direction::UP || direction == Direction::DOWN {
                    if x == 0 {
                        set.insert(position + Point(0, 1) + direction.get_vector(1));
                    } else if x == 1 {
                        set.insert(position + Point(0, -1) + direction.get_vector(1));
                    }
                };
            }
            _ => {
                set.insert(position + direction.get_vector(1));
            }
        };
        set
    }

    #[allow(dead_code)]
    fn print(&self) {
        let height = self.tiles.len();
        let width = self.tiles[0].len();

        for i in 0..height {
            for j in 0..width {
                let tile = match self.get_tile(Point(i as isize, j as isize)) {
                    Tile::Space => '.',
                    Tile::Robot => '@',
                    Tile::Wall => '#',
                    Tile::Box => 'O',
                    Tile::DoubleBox(0) => '[',
                    Tile::DoubleBox(1) => ']',
                    _ => ' ',
                };
                print!("{}", tile);
            }
            println!("");
        }
    }
}

fn parse_map_input_part_1(input: &str) -> Map {
    let mut tiles: Vec<Vec<Tile>> = vec![];
    for row in input.lines() {
        let mut row_vals: Vec<Tile> = vec![];
        for cell in row.chars() {
            row_vals.push(match cell {
                '#' => Tile::Wall,
                'O' => Tile::Box,
                '@' => Tile::Robot,
                _ => Tile::Space,
            })
        }
        tiles.push(row_vals);
    }
    Map { tiles }
}

fn parse_map_input_part_2(input: &str) -> Map {
    let mut tiles: Vec<Vec<Tile>> = vec![];
    for row in input.lines() {
        let mut row_vals: Vec<Tile> = vec![];
        for cell in row.chars() {
            row_vals.push(match cell {
                '#' => Tile::Wall,
                'O' => Tile::DoubleBox(0),
                '@' => Tile::Robot,
                _ => Tile::Space,
            });
            row_vals.push(match cell {
                '#' => Tile::Wall,
                'O' => Tile::DoubleBox(1),
                '@' => Tile::Space,
                _ => Tile::Space,
            });
        }
        tiles.push(row_vals);
    }
    Map { tiles }
}

impl Solution {
    fn solve_a(&self, input: &str) -> u32 {
        let (map, route) = input.split_once("\n\n").unwrap();
        let mut map = parse_map_input_part_1(map);
        // map.print();
        for c in route.chars() {
            let direction = match c {
                '>' => Some(Direction::RIGHT),
                'v' => Some(Direction::DOWN),
                '^' => Some(Direction::UP),
                '<' => Some(Direction::LEFT),
                _ => None,
            };
            if let Some(direction) = direction {
                map.move_robot(direction);
                // map.print();
                // println!("");
            }
        }
        let mut sum = 0;
        for (i, row) in map.tiles.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell == Tile::Box {
                    sum += 100 * i as u32 + j as u32;
                }
            }
        }
        sum
    }

    fn solve_b(&self, input: &str) -> u32 {
        let (map, route) = input.split_once("\n\n").unwrap();
        let mut map = parse_map_input_part_2(map);
        // map.print();
        for c in route.chars() {
            let direction = match c {
                '>' => Some(Direction::RIGHT),
                'v' => Some(Direction::DOWN),
                '^' => Some(Direction::UP),
                '<' => Some(Direction::LEFT),
                _ => None,
            };
            if let Some(direction) = direction {
                map.move_robot(direction);
                // map.print();
                // println!("");
            }
        }
        let mut sum = 0;
        for (i, row) in map.tiles.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell == Tile::DoubleBox(0) {
                    sum += 100 * i as u32 + j as u32;
                }
            }
        }
        sum
    }
}

impl Problem for Solution {
    fn part_one(&self) -> String {
        let input = self.read_file("data/day15/data.txt").unwrap();
        self.solve_a(&input).to_string()
    }

    fn part_two(&self) -> String {
        let input = self.read_file("data/day15/data.txt").unwrap();
        self.solve_b(&input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_small() {
        let solution = Solution {};
        let input = solution.read_file("data/day15/test_small.txt").unwrap();
        let result = solution.solve_a(&input);
        assert_eq!(result, 2028);
    }

    #[test]
    fn test_a() {
        let solution = Solution {};
        let input = solution.read_file("data/day15/test.txt").unwrap();
        let result = solution.solve_a(&input);
        assert_eq!(result, 10092);
    }

    #[test]
    fn test_b_small() {
        let solution = Solution {};
        let input = solution.read_file("data/day15/test_small_b.txt").unwrap();
        let result = solution.solve_b(&input);
        assert_eq!(result, 105 + 207 + 306);
    }

    #[test]
    fn test_b() {
        let solution = Solution {};
        let input = solution.read_file("data/day15/test.txt").unwrap();
        let result = solution.solve_b(&input);
        assert_eq!(result, 9021);
    }
}
