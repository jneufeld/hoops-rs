use std::collections::VecDeque;

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = VecDeque::new();

    for c in string.chars() {
        match c {
            '(' | '[' | '{' => stack.push_back(c),
            ')' => {
                if !matches('(', stack.pop_back()) {
                    return false;
                }
            }
            ']' => {
                if !matches('[', stack.pop_back()) {
                    return false;
                }
            }
            '}' => {
                if !matches('{', stack.pop_back()) {
                    return false;
                }
            }
            _ => (),
        }
    }

    stack.is_empty()
}

fn matches(expected: char, actual: Option<char>) -> bool {
    if let Some(c) = actual {
        if c == expected {
            return true;
        }
    }

    false
}
