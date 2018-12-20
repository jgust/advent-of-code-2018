use day2::{hamming_distance, INPUT};
use std::collections::HashSet;

fn main() {
    let words: Vec<&str> = INPUT.lines().map(|line| line.trim()).collect();
    let mut candidates = HashSet::new();

    for w1 in &words {
        let found: HashSet<&str> = words
            .iter()
            .filter(|w2| hamming_distance(w1, w2) == Ok(1))
            .map(|&w| w)
            .collect();
        candidates.extend(found);
    }

    if candidates.len() > 1 {
        // Here we rely on the assumption that all words found with a hamming distance of 1
        // in the step above belong to the same series. Thus all are differing on the same letter.
        // Therefore it is enough to just compare the first two entries in the set.
        let mut cand_iter = candidates.iter();
        let first = cand_iter.next().unwrap();
        let second = cand_iter.next().unwrap();

        let common: String = first
            .chars()
            .zip(second.chars())
            .filter(|(c1, c2)| c1 == c2)
            .map(|(c1, _)| c1)
            .collect();

        println!(
            "Found {} candidates. Common letters are: {}",
            candidates.len(),
            common
        );
    } else {
        println!("No candidates found!");
    }
}
