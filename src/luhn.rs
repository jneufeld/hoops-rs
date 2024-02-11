/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let Some(digits) = get_digits(code) else {
        return false;
    };

    let mut sum = 0;
    let mut double = false;

    for num in digits.into_iter().rev() {
        sum += match double {
            false => num,
            true => {
                // TODO ???
                let tmp = num * 2;

                if tmp > 9 {
                    tmp - 9
                } else {
                    tmp
                }
            }
        };

        double = !double;
    }

    sum % 10 == 0
}

/// Returns digits needed to check valid Luhn checksum or None if input is
/// invalid.
fn get_digits(code: &str) -> Option<Vec<u32>> {
    if !is_numeric(code) {
        return None;
    }

    let digits: Vec<u32> = code.chars().filter_map(|c| c.to_digit(10)).collect();

    if digits.len() < 2 {
        return None;
    }

    Some(digits)
}

/// Returns true if every character is a digit or whitespace.
fn is_numeric(code: &str) -> bool {
    for c in code.chars() {
        if c != ' ' && !c.is_ascii_digit() {
            return false;
        }
    }

    true
}
