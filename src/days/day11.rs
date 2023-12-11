use std::convert::identity;

use crate::helpers::*;

pub type A1 = impl Display + Debug + Clone;
pub type A2 = impl Display + Debug + Clone;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	galaxies: Vec<(usize, usize)>,
	empty_rows: Vec<RowOrCol>,
	empty_cols: Vec<RowOrCol>,
}

const SPACE: u8 = b'.';
const GALAXY: u8 = b'#';

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum RowOrCol {
	Empty,
	NotEmpty,
}
use RowOrCol::*;

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let image = file.grid(identity);

		let mut empty_cols = Vec::new();

		for x in 0..image[0].len() {
			if image.iter().all(|row| row[x] == SPACE) {
				empty_cols.push(Empty);
			} else {
				empty_cols.push(NotEmpty);
			}
		}

		let mut empty_rows = Vec::new();

		for row in &image {
			if row.iter().all(|&c| c == SPACE) {
				empty_rows.push(Empty);
			} else {
				empty_rows.push(NotEmpty);
			}
		}

		let galaxies =
			image
				.iter()
				.enumerate()
				.flat_map(|(y, row)| {
					row.iter().enumerate().filter_map(move |(x, &cell)| {
						if cell == GALAXY {
							Some((y, x))
						} else {
							None
						}
					})
				})
				.collect();

		Self {
			galaxies,
			empty_rows,
			empty_cols,
		}
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		self.calcuate_with_expansion(2)
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		self.calcuate_with_expansion(1_000_000)
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
	fn calcuate_with_expansion(&self, expansion: usize) -> usize {
		let mut total = 0;
		for (ia, &(gay, gax)) in self.galaxies.iter().enumerate() {
			for &(gby, gbx) in self.galaxies[ia + 1..].iter() {
				let (ytop, ybottom) = if gay < gby { (gay, gby) } else { (gby, gay) };

				for y in ytop..ybottom {
					if self.empty_rows[y] == Empty {
						total += expansion;
					} else {
						total += 1;
					}
				}

				let (xtop, xbottom) = if gax < gbx { (gax, gbx) } else { (gbx, gax) };

				for x in xtop..xbottom {
					if self.empty_cols[x] == Empty {
						total += expansion;
					} else {
						total += 1;
					}
				}
			}
		}
		total
	}
}
