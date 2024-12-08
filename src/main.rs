mod days;
mod utils;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: Option<u8>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let registry = days::load();
    let solutions = match args.day {
        Some(day) => {
            let mut solution = Vec::new();
            solution.push((day, registry.run(day).unwrap()));
            solution
        }
        None => registry.run_all().unwrap(),
    };

    for (day, (p1, p2)) in solutions {
        println!("Day {:0>2}\nPart 1: {:>10}\nPart 2: {:>10}\n", day, p1, p2);
    }

    Ok(())
}
