use anyhow::{bail, Result};
use std::{collections::HashMap, fs};

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;

pub fn load() -> Registry {
    let mut registry = Registry::new();

    registry.register(1, Box::new(day_01::Solution {}));
    registry.register(2, Box::new(day_02::Solution {}));
    registry.register(3, Box::new(day_03::Solution {}));
    registry.register(4, Box::new(day_04::Solution {}));
    registry.register(5, Box::new(day_05::Solution {}));
    registry.register(6, Box::new(day_06::Solution {}));
    registry.register(7, Box::new(day_07::Solution {}));
    registry.register(8, Box::new(day_08::Solution {}));
    registry.register(9, Box::new(day_09::Solution {}));
    registry.register(10, Box::new(day_10::Solution {}));
    registry.register(11, Box::new(day_11::Solution {}));
    registry.register(12, Box::new(day_12::Solution {}));
    registry.register(13, Box::new(day_13::Solution {}));
    registry.register(14, Box::new(day_14::Solution {}));
    registry.register(15, Box::new(day_15::Solution {}));

    registry
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

    pub fn run(&self, day: u8) -> Result<(String, String)> {
        let solver = self.problems.get(&day);
        match solver {
            Some(solution) => Ok((solution.part_one(), solution.part_two())),
            None => bail!("No solution found!"),
        }
    }

    pub fn run_all(&self) -> Result<Vec<(u8, (String, String))>> {
        let mut keys: Vec<&u8> = self.problems.keys().collect();
        keys.sort();
        Ok(keys
            .iter()
            .map(|day| (**day, self.run(**day).unwrap()))
            .collect())
    }
}

pub trait Problem {
    fn part_one(&self) -> String;
    fn part_two(&self) -> String;
    fn read_file(&self, file_path: &str) -> Result<String, std::io::Error> {
        fs::read_to_string(file_path)
    }
}
