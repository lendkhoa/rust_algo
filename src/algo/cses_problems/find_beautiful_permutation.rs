use std::io::{self, Read};

pub
fn search(n: i32) {
	if n == 1 {
		print!("1");
	}
	else if n < 4 {
		print!("NO SOLUTION");
	} else if n == 4 {
		print!("2 4 1 3");
	} else {
		for i in (1..=n).step_by(2) {
			print!("{} ", i);
		}
		for i in (2..=n).step_by(2) {
			print!("{}", i);
			if i + 2 <= n {
				print!(" ");
			}
		}
	}
}

fn main() -> io::Result<()> {
	let mut n = String::new();
	io::stdin().read_line(&mut n).expect("Failed to read");
	let number:i32 = n.trim().parse().expect("Input not an integer");
	search(number);
	Ok(())
}