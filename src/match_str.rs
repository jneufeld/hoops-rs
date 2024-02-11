use std::collections::HashMap;

/// Counts occurrences of each `pattern` in `strings` and returns a
/// corresponding list.
fn matching_strings(strings: &[String], patterns: &[String]) -> Vec<i32> {
    // Negligible allocations since keys (from `strings`) are references.
    let mut counts: HashMap<&str, i32> = HashMap::new();

    // O(n) where n = |strings|.
    for s in strings {
        counts.entry(s).and_modify(|c| *c += 1).or_insert(1);
    }

    // O(m) where m = |patterns|.
    patterns
        .iter()
        .map(|p| *counts.get(p.as_str()).unwrap_or(&0))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_inputs_empty() {
        assert_eq!(matching_strings(&[], &[]), vec![]);
    }

    #[test]
    fn empty_strings() {
        let strings = vec![];
        let patterns = vec!["p1".to_string(), "p2".to_string()];
        let expected = vec![0, 0];

        assert_eq!(matching_strings(&strings, &patterns), expected);
    }

    #[test]
    fn empty_patterns() {
        let strings = vec!["first".to_string(), "second".to_string()];
        let patterns = vec![];
        let expected = vec![];

        assert_eq!(matching_strings(&strings, &patterns), expected);
    }

    #[test]
    fn no_matches() {
        let strings = vec!["first".to_string(), "second".to_string()];
        let patterns = vec!["p1".to_string(), "p2".to_string()];
        let expected = vec![0, 0];

        assert_eq!(matching_strings(&strings, &patterns), expected);
    }

    #[test]
    fn multiple_matches() {
        let strings = vec![
            "fst".to_string(), // 1
            "snd".to_string(), // 1
            "fst".to_string(), // 2
            "snd".to_string(), // 2
            "xyx".to_string(), // 1
            "fst".to_string(), // 3
        ];

        let patterns = vec!["fst".to_string(), "snd".to_string(), "dummy".to_string()];
        let expected = vec![3, 2, 0];

        assert_eq!(matching_strings(&strings, &patterns), expected);
    }
}
