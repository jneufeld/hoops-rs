use std::str;

pub fn encode(source: &str) -> String {
    let mut output = String::new();
    let mut run_len = 0;
    let mut prev = None;

    for char in source.chars() {
        // Increment the run length when the current and previous characters are
        // the same.
        if Some(char) == prev {
            run_len += 1;
        } else if let Some(char) = prev {
            if run_len > 0 {
                let run = (run_len + 1).to_string();
                output.push_str(&run);
            }

            output.push(char);

            run_len = 0;
        }

        prev = Some(char);
    }

    // CPed from above.
    if let Some(char) = prev {
        if run_len > 0 {
            let run = (run_len + 1).to_string();
            output.push_str(&run);
        }

        output.push(char);
    }

    output
}

pub fn decode(source: &str) -> String {
    let mut output = String::new();
    let mut digits = String::new();

    for ch in source.chars() {
        match ch {
            '0'..='9' => digits.push(ch),
            _ => {
                if let Ok(run_len) = str::parse::<usize>(&digits) {
                    let repeats = str::repeat(&ch.to_string(), run_len);
                    output.push_str(&repeats);
                } else {
                    output.push(ch);
                }

                digits.clear();
            }
        }
    }

    output
}
