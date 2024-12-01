mod days;
use std::{collections::HashMap, fs};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    day: u8,
}

fn main() {
    let args = Args::parse();
    let mut registry = Registry::new();

    (days::day_01::Solution {}).add_to_registry(&mut registry);
    (days::day_02::Solution {}).add_to_registry(&mut registry);

    let solution = registry.run(args.day);
    match solution {
        Ok(answer) => {
            println!(
"Day {}
=====
Example P1: {}
Full Data P1: {}
Example P2: {}
Full Data P2: {}",
             args.day, answer.0, answer.1, answer.2, answer.3);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}

pub struct Registry {
    problems: HashMap<u8, Box<dyn Problem>>,
}

impl Registry {
    fn new() -> Self {
        Registry {
            problems: HashMap::new(),
        }
    }

    fn register(&mut self, day: u8, solver: Box<dyn Problem>) {
        self.problems.insert(day, solver);
    }

    fn run(&self, day: u8) -> Result<(String, String, String, String), String> {
        let solver = self.problems.get(&day);
        match solver {
            Some(solution) => Ok((solution.part_one(true), solution.part_one(false), solution.part_two(true), solution.part_two(false))),
            None => Err("No solution found!".to_string()),
        }
    }
}

pub trait Problem {
    fn part_one(&self, test: bool) -> String;
    fn part_two(&self, test: bool) -> String;
    fn add_to_registry(self, registry: &mut crate::Registry);
    fn read_file(&self, file_path: &str) -> Result<String, std::io::Error> {
        fs::read_to_string(file_path)
    }
}
