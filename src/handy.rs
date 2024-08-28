pub fn contains_all_chars(s: &str, chars: &[char]) -> bool {
	chars.iter().all(|&c| s.contains(c))
}