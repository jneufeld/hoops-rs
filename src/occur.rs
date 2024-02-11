use std::collections::{HashMap, HashSet};

/// Given an array of integers arr, return true if the number of occurrences of
/// each value in the array is unique or false otherwise.
fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut counts = HashMap::new();
    for n in &arr {
        counts.entry(n).and_modify(|n| *n += 1).or_insert(1);
    }

    let mut uniq = HashSet::new();
    for (_, c) in &counts {
        uniq.insert(c);
    }

    uniq.len() == counts.len()
}

#[cfg(test)]
mod tests {
    use super::unique_occurrences;

    #[test]
    fn mine() {
        let tests = vec![
            (vec![1], true),
            (vec![1, 1], true),
            (vec![1, 2], false),
            (vec![1, 2, 1], true),
        ];

        for (arr, exp) in tests {
            let res = unique_occurrences(arr);
            assert_eq!(res, exp);
        }
    }
}
