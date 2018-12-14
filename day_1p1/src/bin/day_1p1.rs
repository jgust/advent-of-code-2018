use std::io;

use day_1p1::{prep_input, INPUT_DAY1};

fn main() -> Result<(), io::Error> {
    let input = prep_input(io::Cursor::new(INPUT_DAY1))?;

    let result = input.iter().fold(0, |tot, i| tot + i);
    println!("Resulting frequency is: {}", result);
    Ok(())
}
