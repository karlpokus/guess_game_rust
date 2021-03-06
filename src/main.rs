extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("guess a number");

	let secret_number = rand::thread_rng().gen_range(1, 101);

	//println!("the secret number is: {}", secret_number);
	//println!("input guess");

	loop {
		let mut guess = String::new();

		io::stdin().read_line(&mut guess)
			.expect("failed to read line"); // err handler

		 // convert type and please don't crash if wrong type
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		//println!("you guessed {}", guess);

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("too small"),
			Ordering::Greater => println!("too big"),
			Ordering::Equal => {
				println!("you win");
				break;
			}
		}
	}
}
