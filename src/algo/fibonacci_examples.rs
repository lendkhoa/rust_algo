pub fn finbonacci_examples() {
	fibonacci(6);
}

fn fibonacci(term: i32) {
	println!("Find the {} Fibonacci number: {}", term, nth_fibonacci(term));
}

fn nth_fibonacci(term: i32) -> i32 {
	match term {
		0 => 0,
		1 => 1,
		_ => nth_fibonacci(term - 1) + nth_fibonacci(term - 2),
	}
}

#[cfg(test)]
mod fibonacci_tests {
	use super::*;

	#[test]
	fn test_1() {
		assert_eq!(0, nth_fibonacci(0));
		assert_eq!(1, nth_fibonacci(1));
	}

	#[test]
	fn test_2() {
		assert_eq!(8, nth_fibonacci(6));
		assert_eq!(13, nth_fibonacci(7));
	}
}