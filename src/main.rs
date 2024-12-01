use std::collections::HashMap;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    
    #[arg(short, long, default_value_t = 1)]
    day: u8
}


fn main() {
    let args = Args::parse();
    let mut registry = Registry::new();

    let d1_solutions = Box::new(Day01{});
    registry.register(1, d1_solutions);


    let solution = registry.run(args.day, "");

    match solution {
        Ok(answer) => {
            println!("Day {}, solution {}", args.day, answer);
        },
        Err(err) => {
            println!("{}", err);
        }
    }
}

struct Registry {
    problems: HashMap<u8, Box<dyn Problem>>,
}

impl Registry {
    fn new() -> Self {
        Registry {
            problems: HashMap::new()
        }
    }

    fn register(&mut self, day: u8, solver: Box<dyn Problem>) {
        self.problems.insert(day, solver);
    }

    fn run(&self, day: u8, input: &str) -> Result<String, String> {
        let solver = self.problems.get(&day);
        match solver {
            Some(solution) => {
                Ok(solution.part_one(input))
            },
            None => {
                Err("No solution found!".to_string())
            }
        }
    }
}

pub trait Problem {
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;
}

struct Day01;

impl Problem for Day01 {
    fn part_one(&self, _input: &str) -> String {
        "Day 01a solution".to_string()
    }
    
    fn part_two(&self, _input: &str) -> String {
        "Day 02a solution".to_string()
    }
}