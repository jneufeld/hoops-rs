fn lonely_int(nums: &[i32]) -> i32 {
    // The problem states input values are 0 <= n <= 100. That's small enough
    // to bucket each number's frequency. The number is the index and the
    // value is the count.
    //
    // O(1) space with constant size.
    const max_val: usize = 101;
    let mut counts = [0; max_val];

    // O(n) time where n = |nums|.
    for num in nums {
        let idx = *num as usize;
        counts[idx] += 1;
    }

    // O(n) time where n = |nums|.
    for (idx, count) in counts.iter().enumerate() {
        if *count == 1 {
            return idx as i32;
        }
    }

    panic!("No value found")
}
