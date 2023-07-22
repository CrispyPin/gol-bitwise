use std::fs;

// one of the shittest RLE parsers in existence :)

pub fn parse(filename: &str) -> Option<Vec<Vec<bool>>> {
	let file = fs::read_to_string(filename).ok()?;
	let mut meta = None;
	let mut rle = String::new();
	for line in file.lines() {
		if line.starts_with('#') {
			continue;
		}
		if meta.is_none() {
			meta = Some(line.to_owned());
			continue;
		}
		rle.push_str(line);
	}
	let meta = meta?;

	let properties = meta.split(", ").collect::<Vec<_>>();
	let width: usize = properties.get(0)?.split('=').nth(1)?.trim().parse().ok()?;
	let height: usize = properties.get(1)?.split('=').nth(1)?.trim().parse().ok()?;

	let mut board = vec![vec![false; width]; height];

	let mut x = 0;
	let mut y = 0;
	let mut run_count = 0;

	for char in rle.chars() {
		match char {
			'b' => {
				run_count = run_count.max(1);
				x += run_count;
				run_count = 0;
			}
			'o' => {
				run_count = run_count.max(1);
				for rx in x..(x + run_count) {
					board[y][rx] = true;
				}
				x += run_count;
				run_count = 0;
			}
			'$' => {
				run_count = run_count.max(1);
				y += run_count;
				run_count = 0;
				x = 0;
			}
			'0'..='9' => {
				run_count *= 10;
				run_count += ((char as u8) - b'0') as usize;
			}
			'!' => break,
			'\n' => (),
			other => println!("Unknown token {other} in RLE"),
		}
	}
	Some(board)
}
