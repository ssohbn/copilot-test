// find a number in the fibbonacci sequence
/// # Examples
/// ```
/// use copilottest::fibonacci;
/// assert_eq!(fibonacci(0), 0);
/// assert_eq!(fibonacci(1), 1);
/// assert_eq!(fibonacci(2), 1);
/// assert_eq!(fibonacci(3), 2);
/// ```
pub fn fibonacci(n: u32) -> u32 {
	match n {
		0 => 0,
		1 => 1,
		_ => fibonacci(n-1) + fibonacci(n-2),
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_fibbonacci() {
		assert_eq!(fibonacci(0), 0);
		assert_eq!(fibonacci(1), 1);
		assert_eq!(fibonacci(2), 1);
		assert_eq!(fibonacci(3), 2);
		assert_eq!(fibonacci(4), 3);
		assert_eq!(fibonacci(5), 5);
		assert_eq!(fibonacci(6), 8);
		assert_eq!(fibonacci(7), 13);
		assert_eq!(fibonacci(8), 21);
		assert_eq!(fibonacci(9), 34);
		assert_eq!(fibonacci(10), 55);
		assert_eq!(fibonacci(11), 89);
	}
}