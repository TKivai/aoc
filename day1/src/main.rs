// pub mod custom_error;

use std::fmt::Error;

pub mod part1;
pub mod part2;

use part1::process_part1;
use part2::process_part2;

fn main() -> Result<(), Error> {
    let input_file = "src/input.txt";

    let input = std::fs::read_to_string(input_file).unwrap();

    println!("Result for part 1: {:?}", process_part1(&input));
    println!("Result for part 2: {:?}", process_part2(&input));

    Ok(())
}
