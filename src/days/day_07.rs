use crate::days::Problem;
use anyhow::Result;

pub struct Solution;

fn is_valid(value: i64, operands: &mut Vec<i64>) -> bool {
    if operands.len() == 0 || (operands.len() == 1 && operands[0] != value) {
        false
    } else if operands.len() == 1 && operands[0] == value {
        true
    } else {
        let last_digit = operands.pop().unwrap();
        is_valid(value - last_digit, &mut operands.clone())
            || (value % last_digit == 0 && is_valid(value / last_digit, &mut operands.clone()))
    }
}

fn is_valid_with_concat(value: i64, operands: &mut Vec<i64>) -> bool {
    if operands.len() == 0 || (operands.len() == 1 && operands[0] != value) {
        false
    } else if operands.len() == 1 && operands[0] == value {
        true
    } else {
        let last_digit = operands.pop().unwrap();
        is_valid_with_concat(value - last_digit, &mut operands.clone())
            || (value % last_digit == 0 && is_valid_with_concat(value / last_digit, &mut operands.clone()))
            || (value.to_string().ends_with(&last_digit.to_string()) && (value.to_string().len() > last_digit.to_string().len())
                && is_valid_with_concat(drop_ending(value, last_digit), &mut operands.clone()))
    }
}

fn drop_ending(value: i64, ending: i64) -> i64 {
    let value = value.to_string();
    let ending = ending.to_string();
    value
        .rsplit_once(&ending)
        .unwrap()
        .0
        .parse::<i64>()
        .unwrap()
}

fn parse_line(input: &str) -> Option<(i64, Vec<i64>)> {
    match input.split_once(": ") {
        Some((a, b)) => Some((
            a.parse::<i64>().unwrap(),
            b.split(" ").map(|el| el.parse::<i64>().unwrap()).collect(),
        )),
        None => None,
    }
}

fn parse(input: &str) -> Result<Vec<(i64, Vec<i64>)>> {
    Ok(input
        .lines()
        .map(|line| parse_line(line).unwrap())
        .collect())
}

impl Solution {
    fn solve_a(&self, input: &str) -> i64 {
        let mut sum = 0;
        for (value, operands) in parse(input).unwrap().iter() {
            if is_valid(*value, &mut operands.clone()) {
                sum += value;
            }
        }
        sum
    }

    fn solve_b(&self, input: &str) -> i64 {
        let mut sum = 0;
        for (value, operands) in parse(input).unwrap().iter() {
            if is_valid_with_concat(*value, &mut operands.clone()) {
                sum += value;
            }
        }
        sum
    }
}

impl Problem for Solution {
    fn part_one(&self) -> String {
        let input = self.read_file("data/day07/data.txt").unwrap();
        self.solve_a(&input).to_string()
    }

    fn part_two(&self) -> String {
        let input = self.read_file("data/day07/data.txt").unwrap();
        self.solve_b(&input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case((190, vec![10,19]), true)]
    #[case((3267, vec![81, 40, 27]), true)]
    #[case((83, vec![17,5]), false)]
    #[case((156, vec![15,6]), false)]
    #[case((7290, vec![6,8,6,15]), false)]
    #[case((161011, vec![16,10,13]), false)]
    #[case((192, vec![17,8,14]), false)]
    #[case((21037, vec![9,7,18,13]), false)]
    #[case((292, vec![11,6,16,20]), true)]
    fn test_is_valid(#[case] mut input: (i64, Vec<i64>), #[case] expected: bool) {
        assert_eq!(is_valid(input.0, &mut input.1), expected);
    }

    #[rstest]
    #[case((190, vec![10,19]), true)]
    #[case((3267, vec![81, 40, 27]), true)]
    #[case((83, vec![17,5]), false)]
    #[case((156, vec![15,6]), true)]
    #[case((7290, vec![6,8,6,15]), true)]
    #[case((161011, vec![16,10,13]), false)]
    #[case((192, vec![17,8,14]), true)]
    #[case((21037, vec![9,7,18,13]), false)]
    #[case((292, vec![11,6,16,20]), true)]
    fn test_is_valid_with_concat(#[case] mut input: (i64, Vec<i64>), #[case] expected: bool) {
        assert_eq!(is_valid_with_concat(input.0, &mut input.1), expected);
    }

    #[test]
    fn test_a() {
        let solution = Solution {};
        let input = solution.read_file("data/day07/test.txt").unwrap();
        let result = solution.solve_a(&input);
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_b() {
        let solution = Solution {};
        let input = solution.read_file("data/day07/test.txt").unwrap();
        let result = solution.solve_b(&input);
        assert_eq!(result, 11387);
    }
}
