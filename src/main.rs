use gol_bitwise::region::Region;

fn main() {
	let mut region = Region::new(1, 1);
	region.randomise();
	print!("\x1B[2J"); // clear screen

	loop {
		print!("\x1B[u"); // reset cursor
		region.print_all(true);
		region.step();
		region.auto_grow();
		{
			let mut a = String::new();
			std::io::stdin().read_line(&mut a).unwrap();
		}
	}
}
