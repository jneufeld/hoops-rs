const RADIX: u32 = 10;

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    // TODO multi-purpose variable
    let mut remaining = 10;
    let mut sum = 0;

    for ch in isbn.chars() {
        // Encountered too many characters
        if remaining == 0 {
            return false;
        }

        if let Some(digit) = ch.to_digit(RADIX) {
            sum += digit * remaining;
            remaining -= 1;
        } else if ch == 'X' && remaining == 1 {
            sum += 10 * remaining;
            remaining -= 1;
        }
    }

    // Must be the right sum and have encountered the right number of digits
    remaining == 0 && sum % 11 == 0
}
