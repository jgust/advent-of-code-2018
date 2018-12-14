use std::io;

use day_1p1::{prep_input, INPUT};

fn main() -> Result<(), io::Error> {
    let input = prep_input(io::Cursor::new(INPUT))?;

    let result = input.iter().fold(0, |tot, i| tot + i);
    println!("Resulting frequency is: {}", result);
    Ok(())
}
