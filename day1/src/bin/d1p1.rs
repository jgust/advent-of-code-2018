use std::io;

use day1::{prep_input, INPUT};

fn main() -> Result<(), io::Error> {
    let input = prep_input(io::Cursor::new(INPUT))?;

    let result: i64 = input.iter().sum();
    println!("Resulting frequency is: {}", result);
    Ok(())
}
