fn bucket_sort(arr: &[i32]) -> Vec<i32> {
    const INPUT_MAX: usize = 100;

    let mut bucket = [0; INPUT_MAX];

    for num in arr {
        let idx = *num as usize;
        bucket[idx] += 1;
    }

    bucket.to_vec()
}
