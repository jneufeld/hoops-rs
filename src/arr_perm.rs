fn has_min_sum_perm(min: i32, a: &mut [i32], b: &mut [i32]) -> String {
    // Sort: O(nlogn)
    a.sort_unstable();
    b.sort_unstable();

    // Scan in opposite directions
    let mut i = 0;
    let mut j = b.len() - 1;

    // Scan: O(n)
    while i < a.len() && j >= 0 {
        let sum = a[i] + b[j];

        if sum < min {
            return "NO".to_string();
        }

        i += 1;
        j -= 1;
    }

    "YES".to_string()
}
