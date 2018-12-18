use day2::{calc_checksum, INPUT};

fn main() {
    let words: Vec<&str> = INPUT.lines().map(|line| line.trim()).collect();
    println!("Checksum is: {}", calc_checksum(&words[..]));
}
