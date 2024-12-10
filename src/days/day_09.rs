use crate::days::Problem;

pub struct Solution;

fn parse(input: &str) -> (Vec<u64>, Vec<u64>) {
    let mut numbers: Vec<u64> = Vec::new();
    let mut spaces: Vec<u64> = Vec::new();
    for (idx, c) in input.chars().enumerate() {
        if let Some(x) = c.to_digit(10) {
            if idx % 2 == 0 {
                numbers.push(x.into());
            } else {
                spaces.push(x.into());
            }
        };
    }
    (numbers, spaces)
}

#[derive(Debug)]
struct File{
    id: u64,
    len: usize,
    start: usize,
}

#[derive(Debug, Copy, Clone)]
struct Space{
    len: usize,
    start: usize,
}

impl File {
    fn value(&self) -> u64 {
        self.id * self.len as u64 * (self.len as u64 + 2*self.start as u64 -1 ) / 2
    }

    fn fits_in(&self, space: &Space) -> bool {
        self.len <= space.len
    }
}

fn parse_into_files(input: &str) -> (Vec<File>, Vec<Space>) {
    let mut files = Vec::new();
    let mut spaces = Vec::new();
    let mut pos: usize = 0;
    for (idx, c) in input.chars().enumerate() {
        if let Some(x) = c.to_digit(10) {
            if idx%2 == 0 {
                files.push(File{id: idx as u64/2, len: x as usize, start: pos});
            } else {
                spaces.push(Space{len: x as usize, start: pos});
            }
            pos += x as usize;
        }
    }
    (files, spaces)
}

fn defrag(files: &mut Vec<File>, spaces: &mut Vec<Space>) {
    for file in files.iter_mut().rev() {
        spaces.sort_by(|a,b| a.start.cmp(&b.start));

        for space in spaces.iter_mut() {
            if file.fits_in(space) && file.start > space.start {
                file.start = space.start;
                space.start = space.start + file.len;
                space.len = space.len - file.len;
                break;
            }
        }
    }
}


impl Solution {
    fn solve_a(&self, input: &str) -> u64 {
        let (mut numbers, mut spaces) = parse(input);

        let mut sum: u64 = 0;
        let mut pos: u64 = 0;
        let (mut i, mut j, mut k): (usize, usize, usize) = (0, numbers.len() - 1, 0);

        while i!=j || numbers[i] != 0 {
            if numbers[i] != 0 {
                sum += pos*i as u64;
                pos += 1;
                numbers[i] -= 1;
            } else if spaces[k] != 0 && numbers[j] != 0 {
                sum += pos*j as u64;
                pos += 1;
                spaces[k] -= 1;
                numbers[j] -= 1;
            } else if spaces[k] != 0 && numbers[j] == 0  {
                j -= 1;
            } else {
                i += 1;
                k += 1;
            }
        }
        sum
    }

    fn solve_b(&self, input: &str) -> u64 {
        let (mut files, mut spaces) = parse_into_files(input);
        defrag(&mut files, &mut spaces);
        files.iter().map(|file| file.value()).sum()
    }
}

impl Problem for Solution {
    fn part_one(&self) -> String {
        let input = self.read_file("data/day09/data.txt").unwrap();
        self.solve_a(&input).to_string()
    }

    fn part_two(&self) -> String {
        let input = self.read_file("data/day09/data.txt").unwrap();
        self.solve_b(&input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let solution = Solution {};
        let result = solution.solve_a("12345");
        assert_eq!(result, 60);
    }

    #[test]
    fn test_a() {
        let solution = Solution {};
        let result = solution.solve_a("2333133121414131402");
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_b() {
        let solution = Solution {};
        let result = solution.solve_b("2333133121414131402");
        assert_eq!(result, 2858);
    }
}
