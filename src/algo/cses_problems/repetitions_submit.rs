use std::io;

fn main() -> io::Result<()> {
	let mut sequence = String::new();
	io::stdin().read_line(&mut sequence).expect("Failed");

	let mut counter = 1;
	let mut max_counter = 0;
	let dna_chars: Vec<char> = sequence.chars().collect();

	for i in 1..dna_chars.len() {
		if dna_chars[i] == dna_chars[i-1] {
				counter += 1;
		} else {
				counter = 1;
		}
		if counter > max_counter {
				max_counter = counter;
		}
	}

	println!("{}", max_counter);
		
	Ok(())
}