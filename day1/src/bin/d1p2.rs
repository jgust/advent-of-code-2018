use std::io;

use day1::{find_first_repeated_freq, prep_input, INPUT};

fn main() -> Result<(), io::Error> {
    let input = prep_input(io::Cursor::new(INPUT))?;
    let found = find_first_repeated_freq(0, &input);
    println!("First repeated freq is {}", found);
    Ok(())
}
