use std::cmp::min;

pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
	let mut memo = [0; 1002];

	for j in (0..cost.len()).rev() {
		memo[j] = cost[j] + min(memo[j + 1], memo[j + 2]);
	}

	min(memo[0], memo[1])
}
