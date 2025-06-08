use anyhow::Result;
use day12::{parse_input, solve_part1, solve_part2};

fn main() -> Result<()> {
    // Read the input file that matches the Cargo Package name
    let input_path = format!("../inputs/{}.txt", env!("CARGO_PKG_NAME"));

    let string = parse_input(input_path)?;

    println!("Part 1: {}", solve_part1(&string));
    println!("Part 2: {}", solve_part2(&string));
    Ok(())
}

