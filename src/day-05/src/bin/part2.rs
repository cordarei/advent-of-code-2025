use std::io;
use std::io::Read;

use color_eyre::eyre::Result;

use day_05::process_part2;

fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;

    let answer = process_part2(&buf)?;
    println!("Answer for part 2: {answer}");

    Ok(())
}
