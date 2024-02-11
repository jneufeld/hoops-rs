use std::cmp::{min, max};

pub fn max_area(height: Vec<i32>) -> i32 {
	let mut best = i32::MIN;
	let mut i = 0;
	let mut j = height.len() - 1;

	while i < j {
		let x = j - i;
		let y = min(height[i], height[j]);
		let area = x as i32 * y;
		best = std::cmp::max(area, best);

		if y == height[i] {
			i += 1;
		} else {
			j -= 1;
		}
	}

	best
}
