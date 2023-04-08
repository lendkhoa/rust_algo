use std::convert::TryInto;
use std::io;

struct PermutationGenerator {
	n: i32,
	permutations :Vec<i32>,
	chosen: Vec<bool>,
	results: Vec<Vec<i32>>,
}
impl PermutationGenerator {
	fn new(n: i32) -> PermutationGenerator {
		let mut permutations :Vec<i32> = Vec::new();
		let mut chosen = vec![false; (n + 1) as usize];
		let mut results: Vec<Vec<i32>> = Vec::new();
		PermutationGenerator { n, permutations, chosen, results}
	}

	fn search(&mut self) {
		// println!(">> HELLO 4");
		if self.permutations.len() as i32 == self.n {
			self.gather_data();
		} else {
			for i in 1 .. self.n + 1 {
				if self.chosen[i as usize] {
					continue;
				}
				self.chosen[i as usize] = true;
				self.permutations.push(i);
				self.search();
				self.chosen[i as usize] = false;
				self.permutations.pop();
			}
		}
		// self.print_results();
	}

	fn gather_data(&mut self) {
		let mut temp: Vec<i32> = Vec::new();
		for value in &self.permutations {
			temp.push(*value)
		}
		&self.results.push(temp.to_vec());
	}

	fn print_vector(&self) {
		println!("\n");
		for value in &self.permutations{
			print!("{}, ", value);
		}
	}

	fn print_results(&self) {
		for vector in &self.results {
			println!("\n");
			for value in vector {
				print!("{}, ", value);
			}
		}
	}
}

pub
fn generate_permutation(n: i32) -> Vec<Vec<i32>> {
	let mut generator = PermutationGenerator::new(n);	
	generator.search();
	println!("{:?}", generator.results);
	generator.results
}

pub
fn beautiful_permutation(n: i32) -> io::Result<()> {
	let mut generator = PermutationGenerator::new(n);	
	generator.search();
	// check if there is beautiful permutation
	let mut found_beautiful = false;
	for vector in generator.results {
		if beautiful(&vector) {
			for val in vector {
				print!("{} ", val)
			}			
			found_beautiful = true;
			break; // found the 1st beautiful permutation
			print!("\n")
		}
	}
	if !found_beautiful {
		print!("NO SOLUTION");
	}
	Ok(())
}

fn beautiful(vector: &Vec<i32>) -> bool {
	let mut i = 1;
	while i < vector.len() {
		// println!("Comparing {}th: {}, with {}th: {}. Diff #{}", i, vector[i], i - 1, vector[i-1], (vector[i]-vector[i-1]).abs());
		if (vector[i] - vector[i-1]).abs() == 1 {
			return false;
		}
		i += 1;
	}
	true
}

// FOR CSES SUBMISSION
// fn main() -> io::Result<()> {
// 	let mut n = String::new();
// 	io::stdin().read_line(&mut n).expect("Failed");
// 	let number: i32 = n.trim().parse().expect("Input not an integer");

// 	beautiful_permutation(number);
		
// 	Ok(())
// }

#[cfg(test)]
mod generate_permutation_test {
	use super::*;

	#[test]
	fn test_1() {
		let mut expected: Vec<Vec<i32>> = vec![
			vec![1, 2, 3],
			vec![1, 3, 2],
			vec![2, 1, 3],
			vec![2, 3, 1],
			vec![3, 1, 2],
			vec![3, 2, 1],
		];

		assert_eq!(expected, generate_permutation(3));
	}

}
