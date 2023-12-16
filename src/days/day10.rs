use std::convert::identity;

use crate::helpers::*;

pub type A1 = impl Display + Debug + Clone;
pub type A2 = impl Display + Debug + Clone;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	map: Vec<Vec<u8>>,
	start: [usize; 2],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
	Right,
	Down,
	Left,
	Up,
}
use Direction::*;

impl Direction {
	fn add(self, [y, x]: [usize; 2]) -> [usize; 2] {
		match self {
			Right => [y, x + 1],
			Down => [y + 1, x],
			Left => [y, x - 1],
			Up => [y - 1, x],
		}
	}
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let map = file.trim_ascii_end().grid(identity);

		let mut start = None;
		'l: for (y, row) in map.iter().enumerate() {
			for (x, &cell) in row.iter().enumerate() {
				if cell == b'S' {
					start = Some([y, x]);
					break 'l;
				}
			}
		}

		Self {
			map,
			start: start.unwrap(),
		}
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		let mut one = self.start;
		let mut two = self.start;

		let neighbors = self.map.neighbors(self.start[0], self.start[1]);

		let up = neighbors[0][1].copied();
		let right = neighbors[1][2].copied();
		let down = neighbors[2][1].copied();
		let left = neighbors[1][0].copied();

		let (mut one_dir, mut two_dir) = 'start: {
			let mut first = None;
			match up {
				Some(b'|' | b'7' | b'F') => {
					if let Some(first) = first {
						break 'start (first, Up);
					} else {
						first = Some(Up);
					}
				}
				Some(_) | None => (),
			}
			match right {
				Some(b'-' | b'7' | b'J') => {
					if let Some(first) = first {
						break 'start (first, Right);
					} else {
						first = Some(Right);
					}
				}
				Some(_) | None => (),
			}
			match down {
				Some(b'|' | b'L' | b'J') => {
					if let Some(first) = first {
						break 'start (first, Down);
					} else {
						first = Some(Down);
					}
				}
				Some(_) | None => (),
			}
			match left {
				Some(b'-' | b'L' | b'F') =>
				{
					#[allow(unused_assignments)]
					if let Some(first) = first {
						break 'start (first, Left);
					} else {
						first = Some(Left);
					}
				}
				Some(_) | None => (),
			}
			unreachable!()
		};

		*self.map.grid_get_mut(self.start).unwrap() = match (one_dir, two_dir) {
			(Right, Down) | (Down, Right) => b'f',
			(Right, Left) | (Left, Right) => b'_',
			(Right, Up) | (Up, Right) => b'l',
			(Down, Left) | (Left, Down) => b's',
			(Down, Up) | (Up, Down) => b'i',
			(Left, Up) | (Up, Left) => b'j',
			_ => unreachable!(),
		};

		for steps in 0.. {
			for (current, current_dir) in [(&mut one, &mut one_dir), (&mut two, &mut two_dir)] {
				*current = current_dir.add(*current);
				let next_cell = self.map.grid_get_mut(*current).unwrap();

				*current_dir = match (*current_dir, *next_cell) {
					(Right, b'-') => Right,
					(Right, b'J') => Up,
					(Right, b'7') => Down,

					(Down, b'|') => Down,
					(Down, b'L') => Right,
					(Down, b'J') => Left,

					(Left, b'-') => Left,
					(Left, b'F') => Down,
					(Left, b'L') => Up,

					(Up, b'|') => Up,
					(Up, b'7') => Left,
					(Up, b'F') => Right,

					(_, b'_' | b'i' | b's' | b'j' | b'l' | b'f') => return steps,

					_ => panic!("Reached end of pipe at {current:?}"),
				};

				*next_cell = match *next_cell {
					b'-' => b'_',
					b'|' => b'i',
					b'7' => b's',
					b'J' => b'j',
					b'L' => b'l',
					b'F' => b'f',
					_ => unreachable!(),
				};
			}
			// This is already caught above (hopefully)
			// if one == two {
			// 	break steps;
			// }
		}

		unreachable!()
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		let mut total = 0;
		for row in &self.map {
			let mut inside_loop = false;
			let mut above_is_inside = false;
			for &cell in row {
				match cell {
					b'_' => (),
					b'i' => inside_loop = !inside_loop,
					b's' => inside_loop = above_is_inside,
					b'j' => inside_loop = !above_is_inside,
					b'l' => above_is_inside = !inside_loop,
					b'f' => above_is_inside = inside_loop,
					_ => {
						if inside_loop {
							total += 1;
						}
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
