use image::{GrayImage, Luma};
use regex::Regex;

use crate::{
    days::Problem,
    utils::{point::Point, world::Tick},
};
use anyhow::{bail, Result};

pub struct Solution;

fn print_robots(robots: &Vec<Robot>, height: usize, width: usize, time: usize) {
    let mut img = GrayImage::new(width as u32, height as u32);
    for robot in robots {
        let (y, x) = (robot.position.0, robot.position.1);
        img.put_pixel(x as u32, y as u32, Luma([255]));
    }
    let _ = img.save(format!("images/{}.png", time + 1));
}

impl Solution {
    fn solve_a(&self, input: &str, width: usize, height: usize) -> u32 {
        let mut counts: [u32; 4] = [0, 0, 0, 0];
        for line in input.lines() {
            let robot = Robot::new_from_string(line, (width, height)).unwrap();
            if let Some(q) = get_quadrant(robot.position_at_time(100), width, height) {
                counts[q] += 1;
            };
        }

        counts[0] * counts[1] * counts[2] * counts[3]
    }

    fn solve_b(&self, input: &str, width: usize, height: usize) -> u32 {
        let mut robots = Vec::new();
        for line in input.lines() {
            let robot = Robot::new_from_string(line, (width, height)).unwrap();
            robots.push(robot);
        }
        for time in 0..101 * 103 {
            robots.iter_mut().for_each(|r| r.tick(1));
            print_robots(&robots, height, width, time);
        }
        0
    }
}

fn get_quadrant(position: Point<i64>, width: usize, height: usize) -> Option<usize> {
    let x = (position.1.rem_euclid(width as i64)) as usize;
    let y = (position.0.rem_euclid(height as i64)) as usize;

    let (hx_l, hx_r) = if width % 2 == 0 {
        (width / 2, width / 2)
    } else {
        (width / 2, width / 2 + 1)
    };
    let (hy_t, hy_b) = if height % 2 == 0 {
        (height / 2, height / 2)
    } else {
        (height / 2, height / 2 + 1)
    };

    if x < hx_l && y < hy_t {
        Some(1)
    } else if x < hx_l && y >= hy_b {
        Some(2)
    } else if x >= hx_r && y < hy_t {
        Some(0)
    } else if x >= hx_r && y >= hy_b {
        Some(3)
    } else {
        None
    }
}
struct Robot {
    position: Point<i64>,
    velocity: Point<i64>,
    world_limits: (usize, usize),
}

impl Robot {
    fn new_from_string(input: &str, world_limits: (usize, usize)) -> Result<Self> {
        let re = Regex::new(r"p=(?<px>\d+),(?<py>\d+) v\=(?<vx>\-?\d+),(?<vy>\-?\d+)").unwrap();
        let Some(caps) = re.captures(input) else {
            bail!("Unable to parse input!")
        };

        Ok(Self {
            position: Point(caps["py"].parse().unwrap(), caps["px"].parse().unwrap()),
            velocity: Point(caps["vy"].parse().unwrap(), caps["vx"].parse().unwrap()),
            world_limits,
        })
    }

    fn position_at_time(&self, time: i64) -> Point<i64> {
        let mut new_position = self.position + self.velocity * time;
        new_position.0 = new_position.0.rem_euclid(self.world_limits.1 as i64);
        new_position.1 = new_position.1.rem_euclid(self.world_limits.0 as i64);
        new_position
    }
}

impl Tick<i64> for Robot {
    fn tick(&mut self, dt: i64) {
        let mut new_position = self.position + self.velocity * dt;
        new_position.0 = new_position.0.rem_euclid(self.world_limits.1 as i64);
        new_position.1 = new_position.1.rem_euclid(self.world_limits.0 as i64);
        self.position = new_position;
    }
}

impl Problem for Solution {
    fn part_one(&self) -> String {
        let input = self.read_file("data/day14/data.txt").unwrap();
        self.solve_a(&input, 101, 103).to_string()
    }

    fn part_two(&self) -> String {
        let input = self.read_file("data/day14/data.txt").unwrap();
        self.solve_b(&input, 101, 103).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let solution = Solution {};
        let input = solution.read_file("data/day14/test.txt").unwrap();
        let result = solution.solve_a(&input, 11, 7);
        assert_eq!(result, 12);
    }
    #[test]
    fn test_b() {
        let solution = Solution {};
        let input = solution.read_file("data/day14/data.txt").unwrap();
        let result = solution.solve_b(&input, 101, 103);
        assert_eq!(result, 0);
    }
}
