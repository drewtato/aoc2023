use std::cell::Cell;

use crate::helpers::*;

pub type A1 = impl Display + Debug + Clone;
pub type A2 = impl Display + Debug + Clone;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	file: Grid<(u8, Cell<(u8, u32)>)>,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let file = file.grid(|c| (c, (0, 1).into()));
		Self { file }
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		let mut total = 0;
		for (y, row) in self.file.iter().enumerate() {
			for (x, &(item, _)) in row.iter().enumerate() {
				if item.is_ascii_digit()
					&& !self.file[y]
						.get(x.wrapping_sub(1))
						.is_some_and(|(c, _)| c.is_ascii_digit())
				{
					let mut number = 0;
					let mut near_symbol = false;
					for ox in 0.. {
						let nx = ox + x;
						if let Some(&(c, _)) = self.file[y].get(nx) {
							if c.is_ascii_digit() {
								number *= 10;
								number += (c as char).to_digit(10).unwrap();
								if self
									.file
									.neighbors_iter(y, nx)
									.any(|&(c, _)| !c.is_ascii_digit() && c != b'.')
								{
									near_symbol = true;
								}
							} else {
								break;
							}
						} else {
							break;
						}
					}
					if near_symbol {
						total += number;
					}
				}
			}
		}
		total
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		let mut total = 0;
		for (y, row) in self.file.iter().enumerate() {
			for (x, &(item, _)) in row.iter().enumerate() {
				if item.is_ascii_digit()
					&& !self.file[y]
						.get(x.wrapping_sub(1))
						.is_some_and(|(c, _)| c.is_ascii_digit())
				{
					let mut number = 0;
					let mut near_symbol = None;
					for ox in 0.. {
						let nx = ox + x;
						if let Some(&(c, _)) = self.file[y].get(nx) {
							if c.is_ascii_digit() {
								number *= 10;
								number += (c as char).to_digit(10).unwrap();
								for (oy, row) in self.file.neighbors(y, nx).iter().enumerate() {
									for (ox, &&(c, _)) in row.iter().flatten().enumerate() {
										if c == b'*' {
											near_symbol = Some((y + oy - 1, nx + ox - 1));
										}
									}
								}
							} else {
								break;
							}
						} else {
							break;
						}
					}

					if let Some(near_symbol) = near_symbol {
						let cell = &self.file[near_symbol.0][near_symbol.1].1;
						let (mut count, mut ratio) = cell.get();
						match count {
							// The first number, needs a second one to count
							0 => ratio = number,
							// The second number, we add it tentatively to the total
							1 => {
								ratio *= number;
								total += ratio;
							}
							// There are three numbers, we remove the tetative number from the total
							2 => total -= ratio,
							// After three, nothing matters
							_ => (),
						}
						// This will only ever be six (not even eight since adjacent digits are part
						// of the same number)
						count += 1;
						cell.set((count, ratio));
					}
				}
			}
		}
		total
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
