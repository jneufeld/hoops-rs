use std::{cmp::max, collections::HashMap};

fn rob(nums: Vec<i32>) -> i32 {
    let mut memo: HashMap<usize, i32> = HashMap::new();

    for i in (0..nums.len()).rev() {
        let back2 = memo.get(&(i + 2)).unwrap_or(&0);
        let back3 = memo.get(&(i + 3)).unwrap_or(&0);
        let v = nums[i] + max(back2, back3);
        memo.insert(i, v);
    }

    max(*memo.get(&0).unwrap_or(&0), *memo.get(&1).unwrap_or(&0))
}

#[cfg(test)]
mod tests {
    use crate::rob::rob;

    #[test]
    fn mine() {
        let tests = vec![
            (vec![1, 2, 3, 1], 4),
            (vec![2, 7, 9, 3, 1], 12),
            (vec![2, 1, 1, 2], 4),
        ];

        for (inp, exp) in tests {
            let res = rob(inp);
            assert_eq!(res, exp);
        }
    }
}
