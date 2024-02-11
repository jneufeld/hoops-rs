use std::collections::HashSet;

fn pangrams(s: &str) -> String {
    let lowercase = s.to_ascii_lowercase();
    let alphabet = 'a'..='z';

    let mut encountered: HashSet<char> = HashSet::new();

    for c in lowercase.chars() {
        if alphabet.contains(&c) {
            encountered.insert(c);
        }
    }

    let is_pangram = encountered.len() == 26;
    let answer = if is_pangram { "pangram" } else { "not pangram" };

    answer.to_string()
}
