use std::io::{self, BufRead, BufReader};

fn prep_input<R: io::Read>(input: R) -> io::Result<Vec<i64>> {
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

fn main() -> Result<(), io::Error> {
    let input = include_str!("input.txt");
    let input = prep_input(io::Cursor::new(input))?;

    let result = input.iter().fold(0, |tot, i| tot + i);
    println!("Resulting frequency is: {}", result);
    Ok(())
}
