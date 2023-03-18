use std::io;

fn main() -> io::Result<()> {
	let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Failed to read line");
    let mut number: i64 = input_line.trim().parse().expect("Input not an integer");

	// let mut result: Vec<i64> = Vec::new();
	loop {
		// result.push(number);
		print!("{} ", number);
		if number == 1 {
			break;
		}
		if number % 2 == 0 {
			number /= 2;
		} else {
			number = number * 3 + 1;
		}
		
	}
    Ok(())
}