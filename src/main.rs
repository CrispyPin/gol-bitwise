mod region;
mod tile;
use region::Region;

fn main() {
	let mut region = Region::new();

	loop {
		println!("####################");
		region.print_all();
		region.step();
		{
			let mut a = String::new();
			std::io::stdin().read_line(&mut a).unwrap();
		}
	}
}
