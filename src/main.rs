mod region;
mod tile;
use region::Region;

fn main() {
	let mut region = Region::new(1, 1);
	region.randomise();

	loop {
		println!("####################");
		region.print_all(true);
		region.step();
		region.auto_grow();
		{
			let mut a = String::new();
			std::io::stdin().read_line(&mut a).unwrap();
		}
	}
}
