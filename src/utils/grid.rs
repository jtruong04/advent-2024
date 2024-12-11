use anyhow::{bail, Result};
use std::str::FromStr;

pub fn parse_into_grid<T>(input: &str) -> Result<Vec<Vec<T>>>
where
    T: FromStr,
{
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
    Ok(grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parses_ints() {
        let grid = parse_into_grid::<u8>("123\n123\n123").unwrap();
        assert_eq!(grid, vec![vec![1, 2, 3], vec![1, 2, 3], vec![1, 2, 3]]);
    }

    #[test]
    fn test_parses_chars() {
        let grid = parse_into_grid::<char>("abc\ndef\nghi").unwrap();
        assert_eq!(
            grid,
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
        let _grid = parse_into_grid::<u8>("123\nabc\n123").unwrap();
    }
}
