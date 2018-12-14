use std::io::{self, BufRead, BufReader};

pub static INPUT_DAY1: &'static str = include_str!("input.txt");

pub fn prep_input<R: io::Read>(input: R) -> io::Result<Vec<i64>> {
    let contents = BufReader::new(input);

    contents
        .lines()
        .map(|line| {
            line.and_then(|v| {
                v.parse()
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            })
        })
        .collect()
}
