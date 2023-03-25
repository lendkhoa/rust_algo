use std::io;

pub

fn main() -> io::Result<()> {
	let mut n = String::new();
	io::stdin().read_line(&mut n).expect("failed to read n");
	let number: i128 = n.trim().parse().expect("Input not an integer");
	// read vector
	let mut v = String::new();
	io::stdin().read_line(&mut v).expect("failed to read n");
	let numbers: Vec<_> = v.trim().split(" ").collect();

	println!("{}", increasing_array(number, numbers));
	Ok(())
}

fn increasing_array(n: i128, mut numbers: Vec<&str>) -> i128 {
	let mut min_change:i128= 0;
	for i in 1..numbers.len() {
		if numbers[i].parse::<i128>().unwrap() < numbers[i-1].parse::<i128>().unwrap() {
			min_change += numbers[i-1].parse::<i128>().unwrap() - numbers[i].parse::<i128>().unwrap();
			numbers[i] = numbers[i-1];
		}
	}
	min_change
}

#[cfg(test)]
mod test_increasing_array {
	use super::*;

	#[test]
	fn test_1() {
		assert_eq!(5, increasing_array(5, vec!["3","2","5","1","7"]));
	}

	#[test]
	fn test_2() {
		assert_eq!(12, increasing_array(6, vec!["3","2","5","1","7","0"]));
	}

	#[test]
	fn test_3() {
		assert_eq!(31, increasing_array(31, vec!["6", "10", "4", "10", "2", "8", "9", "2", "7", "7"]));
	}
}