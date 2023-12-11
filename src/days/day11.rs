use crate::helpers::*;

pub type A1 = impl Display + Debug + Clone;
pub type A2 = impl Display + Debug + Clone;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	file: Vec<u8>,
}

// const SPACE: u8 = b'.';
const GALAXY: u8 = b'#';
const EOL: u8 = b'\n';

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
	// Remove last newline
	let file = file.split_last().unwrap().1;

	let mut galaxies_per_col = [0; 256];
	let mut galaxies_per_row = [0; 256];
	let mut galaxies_in_current_row = 0;
	let mut total_galaxies = 0;
	let mut y = 0;
	let mut x = 0;

	for &b in file {
		match b {
			EOL => {
				x = 0;
				galaxies_per_row[y] = galaxies_in_current_row;
				y += 1;
				total_galaxies += galaxies_in_current_row;
				galaxies_in_current_row = 0;
			}
			GALAXY => {
				galaxies_in_current_row += 1;
				galaxies_per_col[x] += 1;
				x += 1;
			}
			// SPACE
			_ => x += 1,
		}
	}

	galaxies_per_row[y] = galaxies_in_current_row;
	total_galaxies += galaxies_in_current_row;

	let height = y + 1;
	let width = x;

	let mut total = 0i64;

	let mut passed = 0;
	let mut current = 0;
	for &galaxies_in_current_col in &galaxies_per_col[..width] {
		total += passed * galaxies_in_current_col * current;
		passed += galaxies_in_current_col;
		total -= (total_galaxies - passed) * galaxies_in_current_col * current;
		current += if galaxies_in_current_col == 0 {
			expansion
		} else {
			1
		};
	}

	let mut passed = 0;
	let mut current = 0;
	for &galaxies_in_current_row in &galaxies_per_row[..height] {
		total += passed * galaxies_in_current_row * current;
		passed += galaxies_in_current_row;
		total -= (total_galaxies - passed) * galaxies_in_current_row * current;
		current += if galaxies_in_current_row == 0 {
			expansion
		} else {
			1
		};
	}

	total
}
