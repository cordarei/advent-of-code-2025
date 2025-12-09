use std::io;
use std::io::Read;

use day_01::process_part1;

use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let answer = process_part1(&buf)?;
    println!("Answer for part 1: {answer}");

    Ok(())
}
