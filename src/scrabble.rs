use std::char;

/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let mut score = 0;

    for c in word.chars() {
        if !char::is_ascii_alphabetic(&c) {
            continue;
        }

        match c.to_ascii_lowercase() {
            'd' | 'g' => score += 2,
            'b' | 'c' | 'm' | 'p' => score += 3,
            'f' | 'h' | 'v' | 'w' | 'y' => score += 4,
            'k' => score += 5,
            'j' | 'x' => score += 8,
            'q' | 'z' => score += 10,
            _ => score += 1,
        }
    }

    score
}
