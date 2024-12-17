use anyhow::{bail, Result};
use std::str::FromStr;

pub struct World<T> {
    pub map: Vec<Vec<T>>,
    pub width: usize,
    pub height: usize,
}

impl<T> World<T>
where
    T: FromStr,
{
    pub fn new_from_string(input: &str) -> Result<Self> {
        let mut grid = Vec::new();
        for line in input.lines() {
            let mut row: Vec<T> = Vec::new();
            for c in line.chars() {
                if let Ok(c) = c.to_string().parse::<T>() {
                    row.push(c);
                } else {
                    bail!("Parsing error!");
                }
            }
            grid.push(row);
        }
        Ok(World {
            height: grid.len(),
            width: if grid.len() > 0 { grid[0].len() } else { 0 },
            map: grid,
        })
    }
}

pub trait Tick<T> {
    fn tick(&mut self, dt: T);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parses_ints() {
        let world = World::<u8>::new_from_string("123\n123\n123").unwrap();
        assert_eq!(world.map, vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]]);
    }

    #[test]
    fn test_parses_chars() {
        let world = World::<char>::new_from_string("abc\ndef\nghi").unwrap();
        assert_eq!(
            world.map,
            vec![
                vec!['a', 'b', 'c'],
                vec!['d', 'e', 'f'],
                vec!['g', 'h', 'i']
            ]
        );
    }

    #[test]
    #[should_panic]
    fn test_fails_with_mixed_types() {
        let _grid = World::<u8>::new_from_string("123\nabc\n123").unwrap();
    }
}
