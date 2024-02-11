pub fn max_vowels(s: String, k: i32) -> i32 {
	let k = k as usize;
	let mut result = 0;
	let s = s.as_bytes();

	for i in 0..k {
		let c = s[i as usize] as char;
		if c=='a' || c=='e' || c=='i' || c=='o' || c=='u' {
			result += 1;
		}
	}

	let mut count = result;
	let mut low = 0;

	for high in k..s.len() {
		let c = s[low];
		if c==b'a' || c==b'e' || c==b'i' || c==b'o' || c==b'u' {
			count -= 1;
		}

		let c = s[high];
		if c==b'a' || c==b'e' || c==b'i' || c==b'o' || c==b'u' {
			count += 1;
		}

		if count > result { result = count; }
		low+=1;
	}

	result
}
