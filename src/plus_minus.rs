pub fn plus_minus(nums: &[i32]) {
    let mut pos_count = 0;
    let mut neg_count = 0;
    let mut zro_count = 0;

    for n in nums {
        if *n > 0 {
            pos_count += 1;
        } else if *n < 0 {
            neg_count += 1;
        } else {
            zro_count += 1;
        }
    }

    let total = nums.len() as f32;

    let pos_ratio = pos_count as f32 / total;
    let neg_ratio = neg_count as f32 / total;
    let zro_ratio = zro_count as f32 / total;

    println!("{:.06}\n{:.06}\n{:.06}", pos_ratio, neg_ratio, zro_ratio);
}
