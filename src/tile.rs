pub type Row = u32;
pub type TileArr = [Row; WIDTH];
pub const EMPTY_TILE: TileArr = [0; WIDTH];

pub const WIDTH: usize = Row::BITS as usize;
const LAST: usize = WIDTH - 1;

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

pub fn step(tile: &mut TileArr, edges: &Edges) {
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

	let mut partial_sums_1 = EMPTY_TILE;
	let mut partial_sums_2 = EMPTY_TILE;

	for (y, row) in tile.iter().enumerate() {
		let left = (row >> 1) | edges.west_bit(y);
		let right = (row << 1) | edges.east_bit(y);
		partial_sums_1[y] = row ^ left ^ right;
		partial_sums_2[y] = (left & right) | ((left ^ right) & row);
	}

	for y in 1..LAST {
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
		let row = edges.n;
		let left = (row >> 1) | edges.nw_bit();
		let right = (row << 1) | edges.ne_bit();
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
		let row = edges.s;
		let left = (row >> 1) | edges.sw_bit();
		let right = (row << 1) | edges.se_bit();
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

impl Edges {
	// pub const EMPTY: Self = Edges {
	// 	n: 0,
	// 	s: 0,
	// 	e: 0,
	// 	w: 0,
	// 	ne: false,
	// 	nw: false,
	// 	se: false,
	// 	sw: false,
	// };

	pub fn full(
		n: &TileArr,
		s: &TileArr,
		e: &TileArr,
		w: &TileArr,
		ne: &TileArr,
		nw: &TileArr,
		se: &TileArr,
		sw: &TileArr,
	) -> Self {
		let mut east = 0;
		let mut west = 0;
		for n in 0..WIDTH {
			east |= (e[n] >> LAST) << n;
			west |= (w[n] & 1) << n;
		}
		Self {
			n: n[LAST],
			s: s[0],
			e: east,
			w: west,
			nw: (nw[LAST] & 1) != 0,
			ne: (ne[LAST] >> LAST) != 0,
			sw: (sw[0] & 1) != 0,
			se: (se[0] >> LAST) != 0,
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
