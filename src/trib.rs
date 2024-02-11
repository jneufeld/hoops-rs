const K: usize = 38;

fn tribonacci(n: i32) -> i32 {
    let mut memo = [0; K];
    memo[1] = 1;
    memo[2] = 1;

    for j in 3..n + 1 {
        let j = j as usize;
        memo[j] = memo[j - 1] + memo[j - 2] + memo[j - 3]
    }

    memo[n as usize]
}

#[cfg(test)]
mod tests {
    use crate::trib::tribonacci;

    #[test]
    fn mine() {
        let tests = vec![
            (0, 0),
            (1, 1),
            (2, 1),
            (4, 4),
            (25, 1389537),
            (37, 2082876103),
        ];

        for (arr, exp) in tests {
            let res = tribonacci(arr);
            assert_eq!(res, exp);
        }
    }
}
