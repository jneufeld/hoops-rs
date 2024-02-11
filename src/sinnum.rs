/// Given a non-empty array of integers nums, every element appears twice except
/// for one. Find that single one.
///
/// You must implement a solution with a linear runtime complexity and use only
/// constant extra space.
fn single_number(nums: Vec<i32>) -> i32 {
    let mut nums = nums.into_iter();
    let mut res = nums.next().expect("lc err");

    if nums.len() == 1 {
        return res;
    }

    for n in nums {
        res ^= n;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::single_number;

    #[test]
    fn mine() {
        let tests = vec![(vec![1], 1), (vec![2, 2, 1], 1), (vec![2, 3, 5, 2, 5], 3)];
        for (inp, exp) in tests {
            let res = single_number(inp);
            assert_eq!(res, exp);
        }
    }
}
