use crate::tile::{Edges, Tile, WIDTH};

pub struct Region {
	/// rows of tiles
	tiles: Vec<Vec<Tile>>,
	size: (usize, usize),
	offset: (isize, isize),
	auto_expand: bool,
}

impl Region {
	pub fn new() -> Self {
		let tiles = vec![vec![Tile::glider(); 2]; 2];
		Self {
			tiles,
			size: (2, 2),
			offset: (-1, -1),
			auto_expand: false,
		}
	}

	pub fn print_all(&self) {
		for y in 0..self.size.1 {
			for ch_row in 0..(WIDTH / 2) {
				for x in 0..self.size.1 {
					self.tiles[y][x].print_row(ch_row);
					// print!("|");
				}
				println!()
			}
			// println!("------");
		}
	}

	pub fn set_cell(&mut self, x: isize, y: isize, state: bool) {
		//
	}

	fn get_tile_relative(&self, x: usize, y: usize, relx: isize, rely: isize) -> Option<&Tile> {
		self.tiles
			.get(y.checked_add_signed(rely)?)
			.and_then(|row| row.get(x.checked_add_signed(relx)?))
	}

	pub fn step(&mut self) {
		// store edges
		let mut edges = Vec::with_capacity(self.size.1);
		for y in 0..self.size.1 {
			let mut row = Vec::with_capacity(self.size.0);
			for x in 0..self.size.0 {
				let n = self.get_tile_relative(x, y, 0, -1).unwrap_or(&Tile::EMPTY);
				let s = self.get_tile_relative(x, y, 0, 1).unwrap_or(&Tile::EMPTY);
				let e = self.get_tile_relative(x, y, 1, 0).unwrap_or(&Tile::EMPTY);
				let w = self.get_tile_relative(x, y, -1, 0).unwrap_or(&Tile::EMPTY);
				let ne = self.get_tile_relative(x, y, 1, -1).unwrap_or(&Tile::EMPTY);
				let nw = self.get_tile_relative(x, y, -1, -1).unwrap_or(&Tile::EMPTY);
				let se = self.get_tile_relative(x, y, 1, 1).unwrap_or(&Tile::EMPTY);
				let sw = self.get_tile_relative(x, y, -1, 1).unwrap_or(&Tile::EMPTY);

				let edge = Edges::full(n, s, e, w, ne, nw, se, sw);
				// dbg!(&edge);
				row.push(edge);
			}
			edges.push(row);
		}
		// dbg!(&edges);
		for y in 0..self.size.1 {
			for x in 0..self.size.0 {
				// dbg!(x, y, &edges[y][x]);
				self.tiles[y][x].step(&edges[y][x]);
			}
		}
		if self.auto_expand {
			// todo
		}
	}
}
