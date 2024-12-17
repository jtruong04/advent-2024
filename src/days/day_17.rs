use anyhow::{bail, Result};
use itertools::Itertools;
use regex::Regex;

use crate::days::Problem;

pub struct Solution;

#[derive(Clone,Copy)]
struct Registry {
    a: u64,
    b: u64,
    c: u64,
    i: usize
}

type Program = Vec<u64>;

#[derive(PartialEq, Eq)]
enum Opcode {
    ADV,
    BXL,
    BST,
    JNZ,
    BXC,
    OUT,
    BDV,
    CDV,
}

impl TryFrom<u64> for Opcode {
    type Error = &'static str;
    fn try_from(value: u64) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Opcode::ADV),
            1 => Ok(Opcode::BXL),
            2 => Ok(Opcode::BST),
            3 => Ok(Opcode::JNZ),
            4 => Ok(Opcode::BXC),
            5 => Ok(Opcode::OUT),
            6 => Ok(Opcode::BDV),
            7 => Ok(Opcode::CDV),
            _ => Err("Invalid opcode"),
        }
    }
}

fn combo_operand(operand: u64, registry: &Registry) -> u64 {
    match operand {
        0|1|2|3 => operand,
        4 => registry.a,
        5 => registry.b,
        6 => registry.c,
        7 => panic!("7 is unused in combo operands"),
        _ => panic!("Combo operand out of range!")
    }
}

impl Opcode {
    fn exec(&self, registry: &mut Registry, operand: u64) -> Option<u64> {
        match self {
            Opcode::ADV => {
                registry.a = registry.a >> combo_operand(operand, registry);
            },
            Opcode::BXL => {
                registry.b = registry.b ^ operand;
            },
            Opcode::BST => {
                registry.b = combo_operand(operand, registry) % 8;
            },
            Opcode::JNZ => {
                if registry.a != 0 {
                    registry.i = operand as usize;
                };
            },
            Opcode::BXC => {
                registry.b = registry.b ^ registry.c;
            },
            Opcode::OUT => {
                return Some(combo_operand(operand, registry) % 8);
            },
            Opcode::BDV => {
                registry.b = registry.a >> combo_operand(operand, registry);
            },
            Opcode::CDV => {
                registry.c = registry.a >> combo_operand(operand, registry);
            },
        };
        None
    }
}


fn parse(input: &str) -> Result<(Registry, Program)> {
    let re = Regex::new(r"(Register A: (?<register_a>\d+)).*\n(Register B: (?<register_b>\d+)).*\n(Register C: (?<register_c>\d+)).*\n\n(Program: (?<program>[\d,]+))").unwrap();
    let Some(caps) = re.captures(input) else {
        bail!("Cannot find expected parts!");
    };

    let registry = Registry {
        a: caps["register_a"].parse().unwrap(),
        b: caps["register_b"].parse().unwrap(),
        c: caps["register_c"].parse().unwrap(),
        i: 0
    };
    let program = caps["program"]
        .split(",")
        .map(|c| c.parse().unwrap())
        .collect();

    Ok((registry, program))
}

fn exec(program: &Program, registry: &mut Registry) -> Result<Vec<u64>> {
    let mut output: Vec<u64> = vec![];
    while let (Some(opcode), Some(operand)) = (program.get(registry.i), program.get(registry.i+1)) {
        let opcode = Opcode::try_from(*opcode).unwrap();
        let single_output = opcode.exec(registry, *operand);
        if let Some(single_output) = single_output {
            output.push(single_output);
        }
        if opcode != Opcode::JNZ || registry.a == 0 {
            registry.i += 2;
        }
    }
    Ok(output)
}

fn exec_from_a(program: &Program, a: u64) -> Result<Vec<u64>> {
    let mut registry = Registry {
        a: a,
        b: 0,
        c: 0,
        i: 0
    };
    let mut output: Vec<u64> = vec![];
    while let (Some(opcode), Some(operand)) = (program.get(registry.i), program.get(registry.i+1)) {
        let opcode = Opcode::try_from(*opcode).unwrap();
        let single_output = opcode.exec(&mut registry, *operand);
        if let Some(single_output) = single_output {
            output.push(single_output);
        }
        if opcode != Opcode::JNZ || registry.a == 0 {
            registry.i += 2;
        }
    }
    Ok(output)
}

fn reverse_engineer(program: &Program) -> Option<u64> {
    let mut reverse_program = program.clone();
    reverse_program.reverse();
    let mut idx = 0;
    let mut a:u64 = 0;
    let mut count = 0;
    loop {
        if let Ok(t) = exec_from_a(program, a) {
            count += 1;
            let mut t = t;
            t.reverse();
            if t == reverse_program {
                println!("Answer found in only {} iterations", count);
                return Some(a);
            }
            if t.len() < reverse_program.len() {
                t.extend(vec![0; reverse_program.len() - t.len()]);
            }
            if t[0..idx+1] == reverse_program[0..idx+1] {
                a <<= 3;
                idx += 1;
            } else {
                if a % 8 == 7 {
                    a >>= 3;
                    idx -= 1;
                }
                a += 1;
            }
        }
    }
}


impl Solution {
    fn solve_a(&self, input: &str) -> String {
        let (mut registry, program) = parse(input).unwrap();
        exec(&program, &mut registry).unwrap().iter().map(|i| i.to_string().to_owned()).join(",")
    }

    fn solve_b(&self, input: &str) -> u64 {
        let (_, program) = parse(input).unwrap();
        reverse_engineer(&program).unwrap()

    }
}

impl Problem for Solution {
    fn part_one(&self) -> String {
        let input = self.read_file("data/day17/data.txt").unwrap();
        self.solve_a(&input).to_string()
    }

    fn part_two(&self) -> String {
        let input = self.read_file("data/day17/data.txt").unwrap();
        self.solve_b(&input).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let solution = Solution {};
        let input = solution.read_file("data/day17/test.txt").unwrap();
        let result = solution.solve_a(&input);
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0".to_string());
    }

    #[test]
    fn test_b() {
        let solution = Solution {};
        let input = solution.read_file("data/day17/test2.txt").unwrap();
        let result = solution.solve_b(&input);
        assert_eq!(result, 117440);
    }
}
