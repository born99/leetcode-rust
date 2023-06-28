pub fn reverse_string(s: &mut Vec<char>) {
	let length = s.len();

	for i in 0..length - 1 {
		let temp = s[i];
		s[i] = s[length - 1 - i];
		s[length - 1 - i] = temp;

		if i >= (length - 1) / 2 {
			break;
		}
	}
}
