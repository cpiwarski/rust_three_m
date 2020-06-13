use std::io;
use std::collections;

fn main() {

	let mut v: Vec<i32> = Vec::new();

	// Collect information
	loop {
		println!("Enter in an integer, or type \"quit\" to begin processing");

		let mut input = String::new();

		io::stdin()
			.read_line(&mut input)
			.expect("Failed to read line");
		
		println!("You entered in {}", input);

		if input.trim() == String::from("quit") {
			break;
		};

		let i: i32 = match input.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		v.push(i);
	
	}

	println!("You supplied the data: {:?}", v);

	let size = v.len() as i32;
	let mut map = collections::HashMap::new();
	let mut avg = 0;

	// Find median
	let median: i32 = v[(size / 2) as usize];

	// Calculate mean
	for x in v.iter() {
		let count = map.entry(x).or_insert(0);
		*count += 1;
		avg += x;
	}
	avg = avg / size;

	// Calculate mode
	let mut mode = 0;
	let mut mode_count = 0;
	for (number, count) in map.iter() {
		if count > &mode_count {
			mode = **number;
			mode_count = *count;
		}
	}

	// Display results
	println!("The median is {}", median);
	println!("The mean is {}", avg);
	println!("The mode is {}", mode);
}
