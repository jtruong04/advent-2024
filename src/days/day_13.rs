use regex::Regex;

use crate::days::Problem;

pub struct Solution;

/// Solves the equations
///     N_a a_x + N_b * b_x = X,      
///     N_a a_y + N_b * b_y = Y,      
/// for positive integers (A,B).
fn solve(ax: i64, ay: i64, bx: i64, by: i64, x: i64, y: i64) -> Option<(u64, u64)> {
    let a = (x * by - y * bx) / (ax * by - ay * bx);
    let b = (x * ay - y * ax) / (bx * ay - by * ax);

    if a * ax + b * bx == x && a * ay + b * by == y && a > 0 && b > 0 {
        Some((a as u64, b as u64))
    } else {
        None
    }
}

fn cost(button_presses: (u64, u64), button_costs: (u64, u64)) -> u64 {
    button_presses.0 * button_costs.0 + button_presses.1 * button_costs.1
}

impl Solution {
    fn solve_a(&self, input: &str) -> u64 {
        let mut total_cost = 0;
        let re_buttons = Regex::new(r"X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
        let re_prize = Regex::new(r"X=(?<x>\d+), Y=(?<y>\d+)").unwrap();
        let machines: Vec<&str> = input.split("\n\n").collect();
        for machine in machines {
            let mut lines = machine.lines();
            let button_a_line = lines.next().unwrap();
            let button_b_line = lines.next().unwrap();
            let target_line = lines.next().unwrap();
            let Some(caps_a) = re_buttons.captures(button_a_line) else {continue;};
            let Some(caps_b) = re_buttons.captures(button_b_line) else {continue;};
            let Some(caps_prize) = re_prize.captures(target_line) else {continue;};

            let combo = solve(
                caps_a["x"].parse::<i64>().unwrap(),
                caps_a["y"].parse::<i64>().unwrap(),
                caps_b["x"].parse::<i64>().unwrap(),
                caps_b["y"].parse::<i64>().unwrap(),
                caps_prize["x"].parse::<i64>().unwrap(),
                caps_prize["y"].parse::<i64>().unwrap()
            );

            if let Some(combo)=combo {
                total_cost += cost(combo, (3,1));
            }

            
        }
        total_cost
    }
    
    fn solve_b(&self, input: &str) -> u64 {
        let mut total_cost = 0;
        let re_buttons = Regex::new(r"X\+(?<x>\d+), Y\+(?<y>\d+)").unwrap();
        let re_prize = Regex::new(r"X=(?<x>\d+), Y=(?<y>\d+)").unwrap();
        let machines: Vec<&str> = input.split("\n\n").collect();
        for machine in machines {
            let mut lines = machine.lines();
            let button_a_line = lines.next().unwrap();
            let button_b_line = lines.next().unwrap();
            let target_line = lines.next().unwrap();
            let Some(caps_a) = re_buttons.captures(button_a_line) else {continue;};
            let Some(caps_b) = re_buttons.captures(button_b_line) else {continue;};
            let Some(caps_prize) = re_prize.captures(target_line) else {continue;};
    
            let combo = solve(
                caps_a["x"].parse::<i64>().unwrap(),
                caps_a["y"].parse::<i64>().unwrap(),
                caps_b["x"].parse::<i64>().unwrap(),
                caps_b["y"].parse::<i64>().unwrap(),
                caps_prize["x"].parse::<i64>().unwrap() + 10000000000000,
                caps_prize["y"].parse::<i64>().unwrap() + 10000000000000
            );
    
            if let Some(combo)=combo {
                total_cost += cost(combo, (3,1));
            }
    
            
        }
        total_cost
    }
}

impl Problem for Solution {
    fn part_one(&self) -> String {
        let input = self.read_file("data/day13/data.txt").unwrap();
        self.solve_a(&input).to_string()
    }

    fn part_two(&self) -> String {
        let input = self.read_file("data/day13/data.txt").unwrap();
        self.solve_b(&input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_solution() {
        let solution = solve(94,34,22,67,8400,5400).unwrap();
        assert_eq!(solution, (80, 40))
    }

    #[test]
    fn test_cost_function() {
        let costs = cost((80, 40), (3, 1));
        assert_eq!(costs, 280);
    }

    #[test]
    fn test_a() {
        let solution = Solution {};
        let input = solution.read_file("data/day13/test.txt").unwrap();
        let result = solution.solve_a(&input);
        assert_eq!(result, 480);
    }
}
