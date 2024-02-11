fn mini_max_sum(nums: &[i32]) {
    let mut sum: i128 = 0;

    for n in nums {
        sum += *n as i128;
    }

    // The input is always five numbers. Using the largest and smallest single
    // values, we can find the largest and smallest sums.
    let min = *nums.iter().min().expect("interview style");
    let max = *nums.iter().max().expect("interview style");

    let max_sum = sum - min as i128;
    let min_sum = sum - max as i128;

    println!("{min_sum} {max_sum}");
}
