use std::collections::VecDeque;

/// Returns number of solutions. A solution is a contiguous run of `span`
// elements from `nums` that sum to `target`.
fn birthday(nums: &[i32], target: i32, span: i32) -> i32 {
    let span = span as usize;

    let mut result = 0;
    let mut run: VecDeque<i32> = VecDeque::new();

    for n in nums {
        run.push_back(*n);

        // Fill sliding window before summing
        if run.len() < span {
            continue;
        }

        let sum: i32 = run.iter().sum();

        if sum == target {
            result += 1;
        }

        // Remove oldest element from window
        run.pop_front();
    }

    result
}
