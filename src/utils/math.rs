use std::str::FromStr;

use anyhow::{bail, Result};

pub fn gcd(x: i32, y: i32) -> i32 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

pub fn count_digits<T>(x: T) -> usize
where
    T: ToString,
{
    x.to_string().len()
}

pub fn split_number<T>(x: T, idx: usize) -> Result<(T, T)>
where
    T: ToString + FromStr + Copy,
{
    if idx > count_digits(x) {
        bail!("Can't split! Index out of range.");
    }
    let mut x = x.to_string();
    let y = x.split_off(idx);

    let x = x.parse::<T>();
    let y = y.parse::<T>();

    match (x,y) {
        (Ok(x), Ok(y)) => {
            return Ok((x,y));
        },
        _ => {
            bail!("Error parsing resulting string back to integer.");
        }
    }
}


 
#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(1234, 4)]
    #[case(34, 2)]
    #[case(00213, 3)]
    fn test_count_digits(#[case] input: u32, #[case] expected: usize) {
        assert_eq!(count_digits(input), expected);
    }


    #[rstest]
    #[case((1234, 2), (12, 34))]
    #[case((32, 1), (3,2))]
    #[case((1000, 2), (10,0))]
    fn test_split_number(#[case] input: (u32, usize), #[case] expected: (u32, u32)) {
        assert_eq!(split_number(input.0, input.1).unwrap(), expected);
    }

}
