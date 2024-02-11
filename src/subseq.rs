/// Given two strings s and t, return true if s is a subsequence of t, or false
/// otherwise.
///
/// A subsequence of a string is a new string that is formed from the original
/// string by deleting 0+ characters without disturbing the relative positions
/// of the remaining characters.
///
/// E.g., "ace" is a subsequence of "abcde" while "aec" is not).
pub fn is_subsequence(s: String, t: String) -> bool {
    if s.is_empty() {
        return true;
    }

    let mut marker = s.chars().peekable();

    for c in t.chars() {
        if marker.peek() == Some(&c) {
            let _ = marker.next();
            if marker.peek().is_none() {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::is_subsequence;

    #[test]
    fn mine() {
        let tests = vec![
            ("", "", true),
            ("", "a", true),
            ("a", "", false),
            ("01", "01", true),
            ("01", "001", true),
            ("01", "101", true),
            ("01", "111", false),
            ("axc", "a123c", false),
        ];

        for (s, t, exp) in tests {
            let res = is_subsequence(s.to_string(), t.to_string());
            assert_eq!(exp, res);
        }
    }
}
