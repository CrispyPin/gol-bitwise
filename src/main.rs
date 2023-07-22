use std::env;

use gol_bitwise::{region::Region, rle};

fn main() {
	let mut region = if let Some(file) = env::args().nth(1) {
		Region::from_bools(rle::parse(&file).unwrap())
	} else {
		let mut r = Region::new(1, 1);
		r.randomise();
		r
	};
	region.auto_grow();

	loop {
		print!("\x1B[2J"); // clear screen
		print!("\x1B[u"); // reset cursor
		region.print_all(false);
		region.step();
		region.auto_grow();
		{
			let mut a = String::new();
			std::io::stdin().read_line(&mut a).unwrap();
		}
	}
}
