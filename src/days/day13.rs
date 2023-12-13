use crate::helpers::*;

pub type A1 = u32;
pub type A2 = A1;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	file: Vec<u8>,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		Self { file }
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		self.solve(|grid| {
			for split in 1..grid.len() {
				let (before, after) = grid.split_at(split);
				let condition = before.iter().rev().zip(after).all(|(b, a)| b == a);
				if condition {
					return split as u32;
				}
			}
			0
		})
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		self.solve(|grid| {
			for split in 1..grid.len() {
				let (before, after) = grid.split_at(split);
				let mut total = 0;
				for (b, a) in before.iter().rev().zip(after) {
					total += (b ^ a).count_ones();
					if total > 1 {
						break;
					}
				}
				if total == 1 {
					return split as u32;
				}
			}
			0
		})
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
	fn solve<F>(&self, mut f: F) -> A1
	where
		F: FnMut(&[u32]) -> A1,
	{
		let mut bytes = self.file.as_slice();
		let mut total = 0;

		loop {
			let mut grid_horizontal = [0; 32];
			let mut grid_vertical = [0; 32];

			let first_horiz = &mut grid_horizontal[0];
			let mut horiz_len = 0;

			for (&byte, vert) in bytes.iter().zip(&mut grid_vertical) {
				if byte == EOL {
					break;
				}
				let bit = (byte & 1) as u32;
				*first_horiz <<= 1;
				*first_horiz |= bit;

				// This is always the first bit so shifting is unnecessary
				*vert |= bit;

				horiz_len += 1;
			}

			bytes.take(..horiz_len + 1).unwrap();

			let mut vert_len = 1;
			for (mut chunk, horiz) in bytes.chunks(horiz_len + 1).zip(&mut grid_horizontal[1..]) {
				if chunk[0] == EOL {
					break;
				}
				bytes.take(..chunk.len()).unwrap();
				chunk.take_last();
				for (&byte, vert) in chunk.iter().zip(&mut grid_vertical) {
					let bit = (byte & 1) as u32;

					*vert <<= 1;
					*vert |= bit;

					*horiz <<= 1;
					*horiz |= bit;
				}

				vert_len += 1;
			}

			let horiz_ans = f(&grid_horizontal[..vert_len]);
			total += if horiz_ans == 0 {
				f(&grid_vertical[..horiz_len])
			} else {
				horiz_ans * 100
			};

			if bytes.take_first().is_none() {
				break;
			}
		}

		total
	}
}

// const ASH: u8 = b'.';
// const ROCK: u8 = b'#';
const EOL: u8 = b'\n';
