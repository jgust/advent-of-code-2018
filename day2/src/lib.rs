use std::cmp;
use std::collections::HashMap;

pub static INPUT: &'static str = include_str!("input.txt");

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    StringLengthMismatch,
}

type Result<T> = std::result::Result<T, Error>;

pub fn count_occurences(word: &str) -> (u32, u32) {
    let mut histogram = HashMap::with_capacity(word.len());
    let mut counts = vec![0; word.len()];

    for c in word.chars() {
        histogram
            .entry(c)
            .and_modify(|e| {
                counts[*e] -= 1;
                *e += 1;
                counts[*e] += 1;
            })
            .or_insert_with(|| {
                counts[1] += 1;
                1
            });
    }

    match counts.len() {
        0...2 => (0, 0),
        3 => (cmp::min(counts[2], 1), 0),
        _ => (cmp::min(counts[2], 1), cmp::min(counts[3], 1)),
    }
}

#[test]
fn test_count_occurrences() {
    assert_eq!((0, 0), count_occurences(""));
    assert_eq!((0, 0), count_occurences("abc"));
    assert_eq!((0, 0), count_occurences("abcdef"));
    assert_eq!((1, 1), count_occurences("bababc"));
    assert_eq!((1, 0), count_occurences("abbcde"));
    assert_eq!((0, 1), count_occurences("abcccd"));
    assert_eq!((1, 0), count_occurences("aabcdd"));
    assert_eq!((0, 1), count_occurences("ababab"));
    assert_eq!((0, 1), count_occurences("aaaabaccvccccbab"));
}

pub fn calc_checksum(words: &[&str]) -> u32 {
    let (no_twos, no_threes) = words
        .iter()
        .map(|w| count_occurences(w))
        .fold((0, 0), |tot, n| (tot.0 + n.0, tot.1 + n.1));
    no_twos * no_threes
}

pub fn hamming_distance(s1: &str, s2: &str) -> Result<u32> {
    if s1.len() != s2.len() {
        return Err(Error::StringLengthMismatch);
    }

    let distance = s1
        .chars()
        .zip(s2.chars())
        .map(|(c1, c2)| if c1 != c2 { 1 } else { 0 })
        .sum();

    Ok(distance)
}

#[test]
fn test_hamming_distance() {
    assert_eq!(Ok(2), hamming_distance("abcde", "axcye"));
    assert_eq!(Ok(1), hamming_distance("fghij", "fguij"));
    assert_eq!(Ok(0), hamming_distance("abcde", "abcde"));
}
