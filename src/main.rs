mod tile;
use tile::*;

fn main() {
	let mut tile = EMPTY_TILE;
	let mut tile2 = EMPTY_TILE;
	let mut tile3 = EMPTY_TILE;
	let mut tile4 = EMPTY_TILE;
	// tile[WIDTH - 8] = (0b_1111_1111_1100_0000 as Row).reverse_bits();
	tile[WIDTH - 8] = 0b_00100000;
	tile[WIDTH - 7] = 0b_00010000;
	tile[WIDTH - 6] = 0b_01110000;

	tile[WIDTH - 3] = 0b_0010;
	tile[WIDTH - 2] = 0b_0001;
	tile[WIDTH - 1] = 0b_0111;
	loop {
		let edges = Edges::full(
			&EMPTY_TILE,
			&tile3,
			&tile2,
			&EMPTY_TILE,
			&EMPTY_TILE,
			&EMPTY_TILE,
			&tile4,
			&EMPTY_TILE,
		);
		let edges2 = Edges::full(
			&EMPTY_TILE,
			&tile4,
			&EMPTY_TILE,
			&tile,
			&EMPTY_TILE,
			&EMPTY_TILE,
			&EMPTY_TILE,
			&tile3,
		);
		let edges3 = Edges::full(
			&tile,
			&EMPTY_TILE,
			&tile4,
			&EMPTY_TILE,
			&tile2,
			&EMPTY_TILE,
			&EMPTY_TILE,
			&EMPTY_TILE,
		);
		let edges4 = Edges::full(
			&tile2,
			&EMPTY_TILE,
			&EMPTY_TILE,
			&tile3,
			&EMPTY_TILE,
			&tile,
			&EMPTY_TILE,
			&EMPTY_TILE,
		);
		println!();
		println!("\n{:#<w$}", "", w = WIDTH * 2 + 1);
		print_tiles(&tile, &tile2);
		// print_tile(&tile3);
		println!("{:-<w$}", "", w = WIDTH * 2 + 1);
		print_tiles(&tile3, &tile4);
		// print_tile(&tile2);
		// print_tile(&tile4);

		step(&mut tile, &edges);
		step(&mut tile2, &edges2);
		step(&mut tile3, &edges3);
		step(&mut tile4, &edges4);

		{
			let mut a = String::new();
			std::io::stdin().read_line(&mut a).unwrap();
		}
	}
}

// fn print_tile(tile: &Tile) {
// 	for y in 0..(WIDTH / 2) {
// 		let top = tile[y * 2];
// 		let bot = tile[y * 2 + 1];
// 		let mut row = String::with_capacity(WIDTH);
// 		for bit in (0..WIDTH).rev() {
// 			let states = ((top >> bit) & 1, (bot >> bit) & 1);
// 			row.push(match states {
// 				(0, 0) => ' ',
// 				(1, 0) => '▀',
// 				(0, 1) => '▄',
// 				(1, 1) => '█',
// 				_ => unreachable!(),
// 			});
// 		}
// 		println!("{row}");
// 	}
// }

fn blocks(top: Row, bottom: Row, bit: usize) -> char {
	let states = ((top >> bit) & 1, (bottom >> bit) & 1);
	match states {
		(0, 0) => ' ',
		(1, 0) => '▀',
		(0, 1) => '▄',
		(1, 1) => '█',
		_ => unreachable!(),
	}
}

fn print_tiles(left: &TileArr, right: &TileArr) {
	for y in 0..(WIDTH / 2) {
		let a = left[y * 2];
		let b = left[y * 2 + 1];
		let mut row = String::with_capacity(WIDTH * 2 + 1);
		for bit in (0..WIDTH).rev() {
			row.push(blocks(a, b, bit));
		}
		row.push('|');
		let a = right[y * 2];
		let b = right[y * 2 + 1];
		for bit in (0..WIDTH).rev() {
			row.push(blocks(a, b, bit));
		}
		println!("{row}");
	}
}
