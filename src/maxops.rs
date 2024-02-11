pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
	let mut ns: Vec<_> = nums.clone().into_iter().filter(|n| n < &k).collect();
	ns.sort();

	if ns.is_empty() { return 0; }

	let mut result = 0;

	let mut lower = 0;
	let mut upper = ns.len() - 1;

	while lower < upper {
		let s = ns[lower] + ns[upper];

		if s == k {
			result += 1;
			lower += 1;
			upper -= 1;
		} else if s > k {
			upper -= 1;
		} else {
			lower += 1;
		}
	}

	result
}
