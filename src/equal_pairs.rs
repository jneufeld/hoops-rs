pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
	let mut result = 0;

	let mut rows = HashMap::new();
	for r in grid.iter() {
		rows
			.entry(r)
			.and_modify(|count| *count += 1)
			.or_insert(1);
	}

	let n = grid.len();
	for c in 0..n {
		let mut col = Vec::new();
		for r in 0..n {
			col.push(grid[r][c]);
		}
		if let Some(count) = rows.get(&col) {
			result += count;
		}
	}

	result
}
