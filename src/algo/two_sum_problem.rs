use std::collections::HashMap;

pub fn two_sum_example() {
	let nums = vec![1,2,34,56,343];
	let target = 360;
	println!("Looking for sum {:?} within the vector {:?}", target, nums);
	println!("These indices has sum of values = target{:?}", two_sum(&nums, &target));
}

fn two_sum(nums: &Vec<i32>, target: &i32) -> Vec<i32> {
	let mut i = 0;
	// key is num, and value is the position
	let mut seen: HashMap<i32, i32> = HashMap::new();
	
	for i in 0..nums.len() {
			let new_target = target - nums.get(i).unwrap();
			if seen.contains_key(&new_target) {
					return vec![i as i32, *seen.get(&new_target).unwrap()];
			}
			seen.insert(*nums.get(i).unwrap(), i as i32);
	}
	vec![-1,-1]
}

#[cfg(test)]
mod two_sum_tests {
	use super::*;

	#[test]
	fn test_1() {
		let nums = vec![1,2,34,56,343];
		let target = 36;
		assert!(two_sum(&nums, &target).contains(&1));
		assert!(two_sum(&nums, &target).contains(&2));
	}

	#[test]
	fn test_with_negative_numbers() {
		let nums = vec![1,-2,34,-56,343];
		let target = -58;
		assert!(two_sum(&nums, &target).contains(&1));
		assert!(two_sum(&nums, &target).contains(&3));
	}
}