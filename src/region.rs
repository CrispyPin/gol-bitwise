use crate::tile::{Edges, Tile, WIDTH};

pub struct Region {
	/// rows of tiles
	tiles: Vec<Vec<Tile>>,
	auto_expand: bool,
}

impl Region {
	pub fn new() -> Self {
		let width = 2;
		let height = 2;
		let mut tiles = Vec::new();
		for _ in 0..height {
			let mut row = Vec::new();
			for _ in 0..width {
				row.push(Tile::random());
			}
			tiles.push(row);
		}

		Self {
			tiles,
			auto_expand: false,
		}
	}

	pub fn height(&self) -> usize {
		self.tiles.len()
	}

	pub fn width(&self) -> usize {
		self.tiles[0].len()
	}

	pub fn print_all(&self) {
		for y in 0..self.height() {
			for ch_row in 0..(WIDTH / 2) {
				for x in 0..self.width() {
					self.tiles[y][x].print_row(ch_row);
					// print!("|");
				}
				println!()
			}
			// println!("------");
		}
	}

	// pub fn set_cell(&mut self, x: isize, y: isize, state: bool) {
	// 	//
	// }

	fn get_tile_relative(&self, x: usize, y: usize, relx: isize, rely: isize) -> Option<&Tile> {
		self.tiles
			.get(y.checked_add_signed(rely)?)
			.and_then(|row| row.get(x.checked_add_signed(relx)?))
	}

	pub fn step(&mut self) {
		// store edges
		let mut edges = Vec::with_capacity(self.height());
		for y in 0..self.height() {
			let mut row = Vec::with_capacity(self.width());
			for x in 0..self.width() {
				let n = self.get_tile_relative(x, y, 0, -1).unwrap_or(&Tile::EMPTY);
				let s = self.get_tile_relative(x, y, 0, 1).unwrap_or(&Tile::EMPTY);
				let e = self.get_tile_relative(x, y, 1, 0).unwrap_or(&Tile::EMPTY);
				let w = self.get_tile_relative(x, y, -1, 0).unwrap_or(&Tile::EMPTY);
				let ne = self.get_tile_relative(x, y, 1, -1).unwrap_or(&Tile::EMPTY);
				let nw = self.get_tile_relative(x, y, -1, -1).unwrap_or(&Tile::EMPTY);
				let se = self.get_tile_relative(x, y, 1, 1).unwrap_or(&Tile::EMPTY);
				let sw = self.get_tile_relative(x, y, -1, 1).unwrap_or(&Tile::EMPTY);

				let edge = Edges::new(n, s, e, w, ne, nw, se, sw);
				row.push(edge);
			}
			edges.push(row);
		}

		for y in 0..self.height() {
			for x in 0..self.width() {
				self.tiles[y][x].step(&edges[y][x]);
			}
		}
		if self.auto_expand {
			// todo
		}
	}
}
