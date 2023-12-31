pub type Row = u64;

pub const WIDTH: usize = Row::BITS as usize;
const LAST: usize = WIDTH - 1;

#[derive(Clone, Debug)]
pub struct Edges {
	n: Row,
	s: Row,
	e: Row,
	w: Row,
	nw: bool,
	ne: bool,
	sw: bool,
	se: bool,
}

#[derive(Clone, Debug)]
pub struct Tile {
	pub rows: [Row; WIDTH],
	pub edges: Edges,
}

impl Tile {
	pub const EMPTY: Tile = Tile {
		rows: [0; WIDTH],
		edges: Edges::EMPTY,
	};

	pub fn new() -> Self {
		Self::EMPTY
	}

	pub fn gliders() -> Self {
		let mut t = Self::EMPTY;
		t.rows[1] = 0b1110000;
		t.rows[2] = 0b1000000;
		t.rows[3] = 0b0100000;
		t.rows[WIDTH - 4] = 0b0100;
		t.rows[WIDTH - 3] = 0b0010;
		t.rows[WIDTH - 2] = 0b1110;
		t
	}

	pub fn edge_east(&self) -> Row {
		let mut edge = 0;
		for n in 0..WIDTH {
			edge |= (self.rows[n] & 1) << n;
		}
		edge
	}

	pub fn edge_west(&self) -> Row {
		let mut edge = 0;
		for n in 0..WIDTH {
			edge |= (self.rows[n] >> LAST) << n;
		}
		edge
	}

	pub fn randomise(&mut self) {
		for r in 0..WIDTH {
			self.rows[r] = rand::random();
		}
	}

	pub fn print_row(&self, ch_row: usize) {
		let mut row = String::with_capacity(WIDTH * 2 + 1);
		let top = self.rows[ch_row * 2];
		let bottom = self.rows[ch_row * 2 + 1];
		for bit in (0..WIDTH).rev() {
			let states = ((top >> bit) & 1, (bottom >> bit) & 1);
			let ch = match states {
				(0, 0) => ' ',
				(1, 0) => '▀',
				(0, 1) => '▄',
				(1, 1) => '█',
				_ => unreachable!(),
			};
			row.push(ch);
		}
		print!("{row}");
	}

	pub fn step(&mut self) {
		fn step_row(state: &mut Row, a0: Row, a1: Row, b0: Row, b1: Row, c0: Row, c1: Row) {
			// simulates addition of [WIDTH] groups of 3 2-bit numbers using bitwise operations

			// partial sum (first and second number/row)
			let t0 = a0 ^ b0;
			let t1 = (a0 & b0) ^ (a1 ^ b1);
			let t2 = (a0 & b0 & (a1 ^ b1)) | (a1 & b1);

			// total neighbor count (incl. center cell)
			let n0 = t0 ^ c0;
			let n1 = (t0 & c0) ^ (t1 ^ c1);
			let n2 = t2 ^ ((t0 & c0 & (t1 ^ c1)) | (t1 & c1));

			// count == 3 || (old_state && count == 4)
			*state = (!n2 & n1 & n0) | (*state & n2 & !(n0 | n1));
		}

		let mut partial_sums_1 = [0; WIDTH];
		let mut partial_sums_2 = [0; WIDTH];

		let tile = &mut self.rows;

		for (y, row) in tile.iter().enumerate() {
			let left = (row >> 1) | self.edges.west_bit(y);
			let right = (row << 1) | self.edges.east_bit(y);
			partial_sums_1[y] = row ^ left ^ right;
			partial_sums_2[y] = (left & right) | ((left ^ right) & row);
		}

		for y in 1..(WIDTH - 1) {
			step_row(
				&mut tile[y],
				partial_sums_1[y - 1],
				partial_sums_2[y - 1],
				partial_sums_1[y],
				partial_sums_2[y],
				partial_sums_1[y + 1],
				partial_sums_2[y + 1],
			);
		}

		// top and bottom cases
		let (partial_north_1, partial_north_2) = {
			let row = self.edges.n;
			let left = (row >> 1) | self.edges.nw_bit();
			let right = (row << 1) | self.edges.ne_bit();
			(row ^ left ^ right, (left & right) | ((left ^ right) & row))
		};
		step_row(
			&mut tile[0],
			partial_north_1,
			partial_north_2,
			partial_sums_1[0],
			partial_sums_2[0],
			partial_sums_1[1],
			partial_sums_2[1],
		);
		let (partial_south_1, partial_south_2) = {
			let row = self.edges.s;
			let left = (row >> 1) | self.edges.sw_bit();
			let right = (row << 1) | self.edges.se_bit();
			(row ^ left ^ right, (left & right) | ((left ^ right) & row))
		};
		step_row(
			&mut tile[LAST],
			partial_sums_1[WIDTH - 2],
			partial_sums_2[WIDTH - 2],
			partial_sums_1[LAST],
			partial_sums_2[LAST],
			partial_south_1,
			partial_south_2,
		);
	}
}

impl Edges {
	pub const EMPTY: Self = Self {
		n: 0,
		s: 0,
		e: 0,
		w: 0,
		nw: false,
		ne: false,
		sw: false,
		se: false,
	};

	pub fn new(
		n: &Tile,
		s: &Tile,
		e: &Tile,
		w: &Tile,
		ne: &Tile,
		nw: &Tile,
		se: &Tile,
		sw: &Tile,
	) -> Self {
		Self {
			n: n.rows[LAST],
			s: s.rows[0],
			e: e.edge_west(),
			w: w.edge_east(),
			nw: (nw.rows[LAST] & 1) != 0,
			ne: (ne.rows[LAST] >> LAST) != 0,
			sw: (sw.rows[0] & 1) != 0,
			se: (se.rows[0] >> LAST) != 0,
		}
	}

	fn west_bit(&self, y: usize) -> Row {
		((self.w >> y) & 1) << LAST
	}

	fn east_bit(&self, y: usize) -> Row {
		(self.e >> y) & 1
	}

	fn nw_bit(&self) -> Row {
		if self.nw {
			1 << LAST
		} else {
			0
		}
	}

	fn sw_bit(&self) -> Row {
		if self.sw {
			1 << LAST
		} else {
			0
		}
	}

	fn ne_bit(&self) -> Row {
		self.ne as Row
	}

	fn se_bit(&self) -> Row {
		self.se as Row
	}
}
