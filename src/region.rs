use crate::tile::{Edges, Tile, WIDTH};

pub struct Region {
	/// rows of tiles
	tiles: Vec<Vec<Tile>>,
}

impl Region {
	pub fn new(width: usize, height: usize) -> Self {
		let tiles = vec![vec![Tile::new(); width]; height];
		Self { tiles }
	}

	pub fn randomise(&mut self) {
		for row in self.tiles.iter_mut() {
			for tile in row {
				tile.randomise()
			}
		}
	}

	pub fn height(&self) -> usize {
		self.tiles.len()
	}

	pub fn width(&self) -> usize {
		self.tiles[0].len()
	}

	pub fn print_all(&self, tile_grid: bool) {
		for y in 0..self.height() {
			for ch_row in 0..(WIDTH / 2) {
				for x in 0..self.width() {
					self.tiles[y][x].print_row(ch_row);
					if tile_grid {
						print!("|");
					}
				}
				println!()
			}
			if tile_grid {
				println!(
					"{}",
					format!("{:->w$}", "+", w = WIDTH + 1).repeat(self.width())
				);
			}
		}
	}

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
	}

	pub fn auto_grow(&mut self) {
		if self.tiles[0].iter().any(|tile| tile.rows[0] != 0) {
			let row = vec![Tile::new(); self.width()];
			self.tiles.insert(0, row);
		}
		if self.tiles[self.height() - 1]
			.iter()
			.any(|tile| tile.rows[WIDTH - 1] != 0)
		{
			let row = vec![Tile::new(); self.width()];
			self.tiles.push(row);
		}
		if self.tiles.iter().any(|row| row[0].edge_west() != 0) {
			for row in &mut self.tiles {
				row.insert(0, Tile::new());
			}
		}
		if self
			.tiles
			.iter()
			.any(|row| row[row.len() - 1].edge_east() != 0)
		{
			for row in &mut self.tiles {
				row.push(Tile::new());
			}
		}
	}
}
