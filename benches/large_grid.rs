use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gol_bitwise::region::Region;

fn large_grid(c: &mut Criterion) {
	const SIZE: usize = 2048 / gol_bitwise::tile::WIDTH; // same number of cells regardless of tile size
	let mut region = black_box(Region::new(SIZE, SIZE));
	region.randomise();

	c.bench_function("large grid", |b| {
		b.iter(|| {
			region.step();
		})
	});
}

criterion_group!(benches, large_grid);
criterion_main!(benches);
