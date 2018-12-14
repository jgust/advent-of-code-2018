use std::io;

use day_1p1::{prep_input, INPUT};
use day_1p2::find_first_repeated_freq;

fn main() -> Result<(), io::Error> {
    let input = prep_input(io::Cursor::new(INPUT))?;
    let found = find_first_repeated_freq(0, &input);
    println!("First repeated freq is {}", found);
    Ok(())
}
