use std::{cmp::max, collections::HashMap};

pub fn rob(nums: Vec<i32>) -> i32 {
    let mut memo: HashMap<usize, i32> = HashMap::new();

    for i in (0..nums.len()).rev() {
        let back2 = memo.get(&(i + 2)).unwrap_or(&0);
        let back3 = memo.get(&(i + 3)).unwrap_or(&0);

        let v = nums[i] + max(back2, back3);

        memo.insert(i, v);
    }

    max(*memo.get(&0).unwrap_or(&0), *memo.get(&1).unwrap_or(&0))
}
