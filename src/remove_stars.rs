pub fn remove_stars(s: String) -> String {
	let mut result = String::new();
	for c in s.chars() {
		if c == '*' { result.pop(); }
		else { result.push(c); }
	}
	result
}
