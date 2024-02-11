fn diag_diff(arr: &[Vec<i32>]) -> i32 {
    // Left-to-right sum: M[0][0], M[1][1], ..., M[i][i].
    let mut left_sum = 0;

    for i in 0..arr.len() {
        left_sum += arr[i][i];
    }

    // Right-to-left sum where n is M's len: M[0][n], M[1][n-1], ...,
    // M[i][n-i].
    let mut right_sum = 0;

    for i in 0..arr.len() {
        right_sum += arr[i][arr.len() - 1 - i];
    }

    (left_sum - right_sum).abs()
}
