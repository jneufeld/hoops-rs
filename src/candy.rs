/// There are n kids with candies. You are given an integer array candies, where
/// each candies[i] represents the number of candies the ith kid has, and an
/// integer extraCandies, denoting the number of extra candies that you have.
///
/// Return a boolean array result of length n, where result[i] is true if, after
/// giving the ith kid all the extraCandies, they will have the greatest number
/// of candies among all the kids, or false otherwise.
///
/// Note that multiple kids can have the greatest number of candies.
fn kids_with_candies(candies: Vec<i32>, extra: i32) -> Vec<bool> {
    let most = candies.iter().max().unwrap();
    candies.iter().map(|n| n + extra >= *most).collect()
}

#[cfg(test)]
mod tests {
    use super::kids_with_candies;

    #[test]
    fn one() {
        let inp = vec![1, 5];
        let exp = vec![false, true];
        let res = kids_with_candies(inp, 1);
        assert_eq!(exp, res);
    }

    #[test]
    fn both() {
        let inp = vec![2, 3];
        let exp = vec![true, true];
        let res = kids_with_candies(inp, 1);
        assert_eq!(exp, res);
    }
}
