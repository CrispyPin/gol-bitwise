use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gol_bitwise::{
	region::Region,
	tile::{self, Tile},
};

fn large_grid(c: &mut Criterion) {
	const SIZE: usize = 2048 / tile::WIDTH; // same number of cells regardless of tile size
	let mut region = black_box(Region::new(SIZE, SIZE));
	region.randomise();

	c.bench_function("large grid", |b| {
		b.iter(|| {
			region.step();
		})
	});
}

fn growing(c: &mut Criterion) {
	c.bench_function("growing", |b| {
		b.iter(|| {
			let mut region = black_box(Region::new(1, 1));
			region.set_tile(Tile::gliders(), 0, 0);
			for _ in 0..1024 {
				region.step();
				region.auto_grow();
			}
		})
	});
}

criterion_group!(benches, large_grid, growing);
criterion_main!(benches);
