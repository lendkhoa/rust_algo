// You are given all numbers between 1,2,…,n
//  except one. Your task is to find the missing number.

// Input

// The first input line contains an integer n
// .

// The second line contains n−1
//  numbers. Each number is distinct and between 1
//  and n
//  (inclusive).

// Output

// Print the missing number.

// Constraints
// 2≤n≤2⋅105

// Example

// Input:
// 5
// 2 3 1 5

// Output:
// 4
use std::io;
pub

fn find_missing_number() -> i64 {
	let mut n = String::new();
	io::stdin().read_line(&mut n).expect("Failed");
	let number: i64 = n.trim().parse().expect("Input not an integer");

	let mut m = String::new();
	io::stdin().read_line(&mut m).expect("Failed");
	// the split() method in Rust returns an iterator 
	// of &str slices, and the given_sum variable is of type i32. 
	// Therefore, you cannot directly add a &str slice to an i32 variable.
	let numbers: Vec<_> = m.trim().split(' ').collect();

	// calculate the sum of all numbers from 1 to n
	let total_sum :i64 = (number * (number + 1)) /2;

	// given sum
	let mut given_sum:i64 = 0;
	for n in numbers {
		given_sum += n.parse::<i64>().unwrap();
	}

	total_sum - given_sum
}

#[cfg(test)]
mod test{
	use super::*;

	#[test]
	fn test_1() {

	}
}

