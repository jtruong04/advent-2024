use crate::days::Problem;

pub struct Solution;

impl Solution {
    fn solve_a(&self, input: &str) -> u32 {
        0
    }

    fn solve_b(&self, input: &str) -> u32 {
        0
    }
}

impl Problem for Solution {
    fn part_one(&self) -> String {
        let input = self.read_file("data/day00/data.txt").unwrap();
        self.solve_a(&input).to_string()
    }

    fn part_two(&self) -> String {
        let input = self.read_file("data/day00/data.txt").unwrap();
        self.solve_b(&input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let solution = Solution {};
        let input = solution.read_file("data/day00/test.txt").unwrap();
        let result = solution.solve_a(&input);
        assert_eq!(result, 143);
    }

    #[test]
    fn test_b() {
        let solution = Solution {};
        let input = solution.read_file("data/day00/test.txt").unwrap();
        let result = solution.solve_b(&input);
        assert_eq!(result, 123);
    }
}
