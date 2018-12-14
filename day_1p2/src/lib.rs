use std::collections::HashSet;

pub fn find_first_repeated_freq(initial: i64, freq_deltas: &[i64]) -> i64 {
    struct FoundRepeat(i64);

    let mut seen_set = HashSet::new();
    seen_set.insert(initial);

    let mut start_val = initial;
    loop {
        let res = freq_deltas.iter().try_fold(start_val, |tot, &i| {
            if seen_set.insert(tot + i) {
                Ok(tot + i)
            } else {
                Err(FoundRepeat(tot + i))
            }
        });

        match res {
            Ok(fin_val) => start_val = fin_val,
            Err(FoundRepeat(i)) => break i,
        }
    }
}

#[test]
fn test_find_repeated_freq() {
    assert_eq!(0, find_first_repeated_freq(0, &[1, -1]));
    assert_eq!(10, find_first_repeated_freq(0, &[3, 3, 4, -2, -4]));
    assert_eq!(5, find_first_repeated_freq(0, &[-6, 3, 8, 5, -6]));
    assert_eq!(14, find_first_repeated_freq(0, &[7, 7, -2, -7, -4]));
}
