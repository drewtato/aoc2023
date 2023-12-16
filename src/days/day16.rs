use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::helpers::*;

pub type A1 = impl Display + Debug + Clone;
pub type A2 = impl Display + Debug + Clone;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	map: Vec<u8>,
	row_len: usize,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let row_len = file.iter().position(|&b| b == b'\n').unwrap();
		Self { map: file, row_len }
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		let start = ([0, 0], [0, 1]);
		let (mut tiles, mut beams) = self.initialize_allocs();
		self.propogate_laser(start, &mut tiles, &mut beams)
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		let len = self.row_len as Coord;

		(0..len)
			.into_par_iter()
			.map(|x| ([0, x], [1, 0]))
			.chain((0..len).into_par_iter().map(|x| ([len - 1, x], [-1, 0])))
			.chain((0..len).into_par_iter().map(|y| ([y, 0], [0, 1])))
			.chain((0..len).into_par_iter().map(|y| ([y, len - 1], [0, -1])))
			.map_init(
				|| self.initialize_allocs(),
				|(tiles, beams), start| self.propogate_laser(start, tiles, beams),
			)
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

type Coord = i8;
type Beam = ([Coord; 2], [Coord; 2]);

const EMPTY: u8 = b'.';
const MIRROR_UP: u8 = b'/';
const MIRROR_DOWN: u8 = b'\\';
const SPLITTER_V: u8 = b'|';
const SPLITTER_H: u8 = b'-';

impl Solution {
	fn propogate_laser(
		&self,
		start: ([Coord; 2], [Coord; 2]),
		tiles: &mut [Coord],
		beams: &mut Vec<Beam>,
	) -> usize {
		let mut set_tiles = 0;
		beams.push(start);

		while let Some((mut position, mut direction)) = beams.pop() {
			let Some(&grid_cell) = self.get(position) else {
				continue;
			};

			let dir_0_bit = direction[0].abs() * (direction[0] + 3);
			let dir_1_bit = direction[1].abs() * (direction[1] + 3);
			let dir_bit = dir_0_bit << 1 | dir_1_bit >> 1;
			// println!("{dir_bit:04b}");
			let count = &mut tiles[position[0] as usize * self.row_len + position[1] as usize];
			if *count == 0 {
				set_tiles += 1;
				*count |= dir_bit;
			} else if *count & dir_bit == 0 {
				*count |= dir_bit;
			} else {
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
					if direction[0] == 0 {
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
		// dbg!(tiles.capacity());
		// dbg!(beams.capacity());
		set_tiles
	}

	fn initialize_allocs(&self) -> (Vec<Coord>, Vec<Beam>) {
		let tiles = vec![0; self.row_len.pow(2)];
		let beams = Vec::with_capacity(128);
		(tiles, beams)
	}

	fn get(&self, position: [i8; 2]) -> Option<&u8> {
		let [y, x] = position;
		let [y, x] = [y as u8 as usize, x as u8 as usize];
		if x >= self.row_len {
			None
		} else {
			self.map.get(y * (self.row_len + 1) + x)
		}
	}
}
