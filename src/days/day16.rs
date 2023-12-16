#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::convert::identity;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::helpers::*;

pub type A1 = impl Display + Debug + Clone;
pub type A2 = impl Display + Debug + Clone;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	file: Vec<Vec<u8>>,
}

const EMPTY: u8 = b'.';
const MIRROR_UP: u8 = b'/';
const MIRROR_DOWN: u8 = b'\\';
const SPLITTER_V: u8 = b'|';
const SPLITTER_H: u8 = b'-';

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		Self {
			file: file.grid(identity),
		}
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		let mut tiles = HashSet::new();
		let mut beams = vec![([0, 0], [0, 1])];
		while let Some((mut position, mut direction)) = beams.pop() {
			let Some(&grid_cell) = grid_get(&self.file, position) else {
				continue;
			};
			if !tiles.insert((position, direction)) {
				continue;
			}
			match grid_cell {
				EMPTY => {}
				MIRROR_UP => {
					if direction[0] != 0 {
						direction = [0, -direction[0]];
					} else {
						direction = [-direction[1], 0];
					}
				}
				MIRROR_DOWN => {
					if direction[0] != 0 {
						direction = [0, direction[0]];
					} else {
						direction = [direction[1], 0];
					}
				}
				SPLITTER_V => {
					if direction[0] != 0 {
					} else {
						direction = [1, 0];
						beams.push((position, [-1, 0]));
					}
				}
				SPLITTER_H => {
					if direction[0] != 0 {
						direction = [0, 1];
						beams.push((position, [0, -1]));
					}
				}
				_ => {
					unreachable!()
				}
			}
			position = add(position, direction);
			beams.push((position, direction));
		}
		tiles
			.into_iter()
			.map(|(p, _)| p)
			.collect::<HashSet<_>>()
			.len()
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		let len = self.file.len() as i32;

		(0..len)
			.into_par_iter()
			.map(|x| ([0, x], [1, 0]))
			.chain((0..len).into_par_iter().map(|x| ([len - 1, x], [-1, 0])))
			.chain((0..len).into_par_iter().map(|y| ([y, 0], [0, 1])))
			.chain((0..len).into_par_iter().map(|y| ([y, len - 1], [0, -1])))
			.map(|(pos, dir)| {
				let mut tiles = HashSet::new();
				let mut beams = vec![(pos, dir)];
				while let Some((mut position, mut direction)) = beams.pop() {
					let Some(&grid_cell) = grid_get(&self.file, position) else {
						continue;
					};
					if !tiles.insert((position, direction)) {
						continue;
					}
					match grid_cell {
						EMPTY => {}
						MIRROR_UP => {
							if direction[0] != 0 {
								direction = [0, -direction[0]];
							} else {
								direction = [-direction[1], 0];
							}
						}
						MIRROR_DOWN => {
							if direction[0] != 0 {
								direction = [0, direction[0]];
							} else {
								direction = [direction[1], 0];
							}
						}
						SPLITTER_V => {
							if direction[0] != 0 {
							} else {
								direction = [1, 0];
								beams.push((position, [-1, 0]));
							}
						}
						SPLITTER_H => {
							if direction[0] != 0 {
								direction = [0, 1];
								beams.push((position, [0, -1]));
							}
						}
						_ => {
							unreachable!()
						}
					}
					position = add(position, direction);
					beams.push((position, direction));
				}
				tiles
					.into_iter()
					.map(|(p, _)| p)
					.collect::<HashSet<_>>()
					.len()
			})
			.max()
			.unwrap()
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
