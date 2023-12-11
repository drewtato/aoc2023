use crate::helpers::*;

pub type A1 = impl Display + Debug + Clone;
pub type A2 = impl Display + Debug + Clone;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	file: Vec<u8>,
}

// const SPACE: u8 = b'.';
const GALAXY: u8 = b'#';
// const EOL: u8 = b'\n';

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		Self { file }
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		self.solve_with_expansion(2)
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		self.solve_with_expansion(1_000_000)
	}

	fn run_any<W: std::fmt::Write>(
		&mut self,
		part: u32,
		_writer: W,
		_: u8,
	) -> Res<std::time::Duration> {
		#[allow(clippy::match_single_binding)]
		match part {
			_ => Err(AocError::PartNotFound),
		}
	}
}

impl Solution {
	fn solve_with_expansion(&self, expansion: i64) -> i64 {
		solve(&self.file, expansion)
	}
}

fn solve(file: &[u8], expansion: i64) -> i64 {
	// Assume square grid, which means the file is x by (x + 1)
	// Use the quadratic formula
	let size = ((file.len() * 4 + 1).sqrt() - 1) / 2;
	if size == 140 {
		return solve_opt(file, expansion);
	}

	let mut scratch_space = vec![0; size * 2];
	// let mut scratch_space = [0; 140 * 2];
	let (galaxies_per_col, galaxies_per_row) = scratch_space.split_at_mut(size);
	let mut total_galaxies = 0;
	let chunks = file.chunks(size + 1);

	for (gpr, chunk) in galaxies_per_row.iter_mut().zip(chunks) {
		for (x, &b) in chunk[..size].iter().enumerate() {
			if b == GALAXY {
				*gpr += 1;
				galaxies_per_col[x] += 1;
			}
		}
		total_galaxies += *gpr;
	}

	let mut total = 0i64;

	for galaxies in [galaxies_per_col, galaxies_per_row] {
		let mut passed = 0;
		let mut current = 0;
		for &mut galaxies_in_current in galaxies {
			total += passed * galaxies_in_current * current;
			passed += galaxies_in_current;
			total -= (total_galaxies - passed) * galaxies_in_current * current;
			current += if galaxies_in_current == 0 {
				expansion
			} else {
				1
			};
		}
	}

	total
}

fn solve_opt(file: &[u8], expansion: i64) -> i64 {
	// Assume square grid, which means the file is x by (x + 1)
	// Use the quadratic formula
	// let size = ((file.len() * 4 + 1).sqrt() - 1) / 2;
	let size = 140;

	let mut scratch_space = [0; 140 * 2];
	// let mut scratch_space = [0; 140 * 2];
	let (galaxies_per_col, galaxies_per_row) = scratch_space.split_at_mut(size);
	let mut total_galaxies = 0;
	let chunks = file.chunks(size + 1);

	for (gpr, chunk) in galaxies_per_row.iter_mut().zip(chunks) {
		for (x, &b) in chunk[..size].iter().enumerate() {
			if b == GALAXY {
				*gpr += 1;
				galaxies_per_col[x] += 1;
			}
		}
		total_galaxies += *gpr;
	}

	let mut total = 0i64;

	for galaxies in [galaxies_per_col, galaxies_per_row] {
		let mut passed = 0;
		let mut current = 0;
		for &mut galaxies_in_current in galaxies {
			total += passed * galaxies_in_current * current;
			passed += galaxies_in_current;
			total -= (total_galaxies - passed) * galaxies_in_current * current;
			current += if galaxies_in_current == 0 {
				expansion
			} else {
				1
			};
		}
	}

	total
}
