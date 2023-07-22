use crate::tile::{Edges, Row, Tile, WIDTH};

pub struct Region {
	/// rows of tiles
	tiles: Vec<Vec<Tile>>,
}

impl Region {
	pub fn new(width: usize, height: usize) -> Self {
		let tiles = vec![vec![Tile::new(); width]; height];
		Self { tiles }
	}

	pub fn from_bools(board: Vec<Vec<bool>>) -> Self {
		let height = board.len() / WIDTH + 1;
		let width = board[0].len() / WIDTH + 1;
		let mut tiles = vec![vec![Tile::new(); width]; height];

		for tile_y in 0..height {
			for tile_x in 0..width {
				let tile = &mut tiles[tile_y][tile_x];
				for y in 0..WIDTH {
					let by = tile_y * WIDTH + y;
					if by >= board.len() {
						break;
					}
					let mut row = 0;
					for x in 0..WIDTH {
						let bx = tile_x * WIDTH + x;
						if bx >= board[by].len() {
							break;
						}
						row |= (board[by][bx] as Row) << (WIDTH - x - 1) as Row;
					}
					tile.rows[y] = row;
				}
			}
		}

		Self { tiles }
	}

	pub fn randomise(&mut self) {
		for row in self.tiles.iter_mut() {
			for tile in row {
				tile.randomise()
			}
		}
	}

	pub fn set_tile(&mut self, tile: Tile, x: usize, y: usize) {
		self.tiles[y][x] = tile;
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

	fn get_tile_rel(&self, x: usize, y: usize, relx: isize, rely: isize) -> &Tile {
		let (Some(x), Some(y)) = (x.checked_add_signed(relx), y.checked_add_signed(rely))
			else { return &Tile::EMPTY; };
		self.tiles
			.get(y)
			.and_then(|row| row.get(x))
			.unwrap_or(&Tile::EMPTY)
	}

	pub fn step(&mut self) {
		for y in 0..self.height() {
			for x in 0..self.width() {
				let n = self.get_tile_rel(x, y, 0, -1);
				let s = self.get_tile_rel(x, y, 0, 1);
				let e = self.get_tile_rel(x, y, 1, 0);
				let w = self.get_tile_rel(x, y, -1, 0);
				let ne = self.get_tile_rel(x, y, 1, -1);
				let nw = self.get_tile_rel(x, y, -1, -1);
				let se = self.get_tile_rel(x, y, 1, 1);
				let sw = self.get_tile_rel(x, y, -1, 1);
				self.tiles[y][x].edges = Edges::new(n, s, e, w, ne, nw, se, sw);
			}
		}

		for y in 0..self.height() {
			for x in 0..self.width() {
				self.tiles[y][x].step();
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
