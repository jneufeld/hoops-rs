pub fn unique_occurrences(arr: Vec<i32>) -> bool {
	let mut counts = HashMap::new();
	for n in &arr {
		counts.entry(n).and_modify(|n| *n += 1).or_insert(1);
	}

	let mut uniq = HashSet::new();
	for (n, c) in &counts {
		uniq.insert(c);
	}

	uniq.len() == counts.len()
}
