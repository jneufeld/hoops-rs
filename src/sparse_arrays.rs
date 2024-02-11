use std::collections::HashMap;

fn matching_strings(inputs: &[String], patterns: &[String]) -> Vec<i32> {
    // Negligible allocations since keys (from `strings`) are references.
    let mut counts: HashMap<&str, i32> = HashMap::new();

    // O(n) where n = |strings|.
    for s in inputs {
        counts.entry(s).and_modify(|c| *c += 1).or_insert(1);
    }

    // O(m) where m = |patterns|.
    patterns
        .iter()
        .map(|p| *counts.get(p.as_str()).unwrap_or(&0))
        .collect()
}
