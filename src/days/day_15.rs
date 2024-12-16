use std::{cell::RefCell, rc::Rc};

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

struct BoundingBox {
    top_left: Point<isize>,
    bottom_right: Point<isize>,
}

impl BoundingBox {
    fn intersects(&self, other: &BoundingBox) -> bool {
        self.top_left.0 <= other.bottom_right.0
            && self.bottom_right.0 > other.top_left.0
            && self.top_left.1 <= other.bottom_right.1
            && self.bottom_right.1 > other.top_left.1
    }

    fn shift(&self, direction: Direction) -> BoundingBox {
        BoundingBox {
            top_left: self.top_left + direction.get_vector(1),
            bottom_right: self.bottom_right + direction.get_vector(1),
        }
    }
}

struct Object {
    position: Point<isize>,
    size: Point<isize>,
    id: u32,
}
struct Wall {
    position: Point<isize>,
    id: u32,
}

trait Id {
    fn get_id(&self) -> u32;
}

impl Id for Wall {
    fn get_id(&self) -> u32 {
        self.id
    }
}

impl Id for Object {
    fn get_id(&self) -> u32 {
        self.id
    }
}

trait Position {
    fn can_move(&self, _direction: Direction, _scene: &Scene) -> bool {
        false
    }
    fn do_move(&mut self, _direction: Direction, _scene: &Scene) {}
    fn is_at(&self, position: Point<isize>) -> bool;
    fn get_bounding_box(&self) -> BoundingBox;
}

impl Position for Wall {
    fn is_at(&self, position: Point<isize>) -> bool {
        self.position == position
    }
    fn get_bounding_box(&self) -> BoundingBox {
        BoundingBox {
            top_left: self.position,
            bottom_right: Point(self.position.0 + 1, self.position.1 + 1),
        }
    }
}

impl Position for Object {
    fn can_move(&self, direction: Direction, scene: &Scene) -> bool {
        // Get bounding box for target location
        let bb = self.get_bounding_box().shift(direction);
        // Get all blocks that intersect new bounding box
        let intersecting_tiles = scene.tiles.iter().filter(|tile| {
            tile.borrow().get_bounding_box().intersects(&bb)
                && tile.borrow().get_id() != self.get_id()
        });
        // Check if they can be moved
        for tile in intersecting_tiles {
            let tile = tile.borrow();
            // If any cannot, this object cannot
            if !tile.can_move(direction, scene) {
                return false;
            }
        }
        true
    }
    fn do_move(&mut self, direction: Direction, scene: &Scene) {
        self.position = self.position + direction.get_vector(1);
        let bb = self.get_bounding_box().shift(direction);
        for tile in scene.tiles.iter() {
            if tile.borrow().get_bounding_box().intersects(&bb) && tile.borrow().get_id() != self.get_id() {
                tile.borrow_mut().do_move(direction, scene);
            }
        }
    }
    fn is_at(&self, position: Point<isize>) -> bool {
        self.position.0 <= position.0
            && position.0 < self.position.0 + self.size.0
            && self.position.1 <= position.1
            && position.1 < self.position.1 + self.size.1
    }
    fn get_bounding_box(&self) -> BoundingBox {
        BoundingBox {
            top_left: self.position,
            bottom_right: self.position + self.size,
        }
    }
}

impl PositionId for Object {}
impl PositionId for Wall {}

trait PositionId: Position + Id {}

struct Scene {
    tiles: Vec<Rc<RefCell<Box<dyn PositionId>>>>,
    size: (isize, isize),
}

impl Scene {
    fn get_object_at(&self, position: Point<isize>) -> Option<Rc<RefCell<Box<dyn PositionId>>>> {
        self.tiles
            .iter()
            .filter(|tile| tile.borrow().is_at(position))
            .next()
            .cloned()
    }
}

fn parse_map_input(input: &str) -> (Scene, Option<Rc<RefCell<Box<dyn PositionId>>>>) {
    let rows = input.lines().collect::<Vec<&str>>().len() as isize;
    let cols = input.len() as isize / rows;

    let mut objs: Vec<Rc<RefCell<Box<dyn PositionId>>>> = vec![];
    let mut robot: Option<Rc<RefCell<Box<dyn PositionId>>>> = None;

    let mut cur_id = 1_u32;
    for (i, line) in input.lines().enumerate() {
        for (j, cell) in line.chars().enumerate() {
            match cell {
                'O' => {
                    objs.push(Rc::new(RefCell::new(Box::new(Object {
                        id: cur_id,
                        size: Point(1, 1),
                        position: Point(i as isize, j as isize),
                    }))));
                    cur_id += 1;
                }
                '@' => {
                    let robot_ref:Rc<RefCell<Box<dyn PositionId>>> = Rc::new(RefCell::new(Box::new(Object {
                        id: cur_id,
                        size: Point(1, 1),
                        position: Point(i as isize, j as isize),
                    })));
                    objs.push(robot_ref.clone());
                    robot = Some(robot_ref.clone());
                    cur_id += 1;
                }
                '#' => {
                    objs.push(Rc::new(RefCell::new(Box::new(Wall {
                        id: cur_id,
                        position: Point(i as isize, j as isize),
                    }))));
                    cur_id += 1;
                }
                _ => {}
            }
        }
    }

    (
        Scene {
            tiles: objs,
            size: (rows, cols),
        },
        robot,
    )
}

impl Solution {
    fn solve_a(&self, input: &str) -> u32 {
        let (map, route) = input.split_once("\n\n").unwrap();
        let (scene, robot) = parse_map_input(map);
        match robot {
            Some(robot) => {
                for c in route.chars() {
                    let direction = match c {
                        '>' => Some(Direction::RIGHT),
                        'v' => Some(Direction::DOWN),
                        '^' => Some(Direction::UP),
                        '<' => Some(Direction::LEFT),
                        _ => None,
                    };
                    if let Some(direction) = direction {
                        if robot.borrow().can_move(direction, &scene) {
                            robot.borrow_mut().do_move(direction, &scene);
                        }
                    }
                    // {
                    //     scene.print();
                    // }
                    // println!(
                    //     "{},{}",
                    //     robot.borrow(),
                    //     robot.borrow().position.1
                    // );
                }
            },
            None => {return 0;}
        }
        0
    }

    fn solve_b(&self, _input: &str) -> u32 {
        0
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
    fn test_b() {
        let solution = Solution {};
        let input = solution.read_file("data/day15/test.txt").unwrap();
        let result = solution.solve_b(&input);
        assert_eq!(result, 0);
    }
}
