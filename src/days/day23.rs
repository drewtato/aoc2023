use arrayvec::ArrayVec;

use crate::helpers::*;

pub type A1 = impl Display + Debug + Clone;
pub type A2 = impl Display + Debug + Clone;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	map: Grid<u8>,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		Self {
			map: file.grid(identity),
		}
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		let start = [0, 1];
		let end = [self.map[0].len() as i16 - 1, self.map.len() as i16 - 2];
		let mut stack = vec![(start, HashSet::from_iter([start]))];
		let mut best = 0;
		while let Some((mut pos, mut set)) = stack.pop() {
			loop {
				if pos == end {
					best = best.max(set.len());
					break;
				}
				let mut possible = ArrayVec::<[i16; 2], 4>::new();
				for (d, slope) in NEIGHBORS {
					let n = add(pos, d);
					if set.contains(&n) {
						continue;
					}
					match self.get(n) {
						b'.' => possible.push(n),
						b'#' => (),
						s if s == slope => possible.push(n),
						_ => (), // uphill slope
					}
				}
				let last = possible.pop();
				for n in possible {
					let mut set = set.clone();
					set.insert(n);
					stack.push((n, set));
				}
				if let Some(n) = last {
					set.insert(n);
					pos = n;
				} else {
					break;
				}
			}
		}
		best - 1
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		let start = [0, 1];
		let end = [self.map[0].len() as i16 - 1, self.map.len() as i16 - 2];
		let mut stack = vec![(start, HashSet::from_iter([start]))];
		let mut best = 0;
		while let Some((mut pos, mut set)) = stack.pop() {
			loop {
				if pos == end {
					best = best.max(set.len());
					break;
				}
				let mut possible = ArrayVec::<[i16; 2], 4>::new();
				for (d, _) in NEIGHBORS {
					let n = add(pos, d);
					if set.contains(&n) {
						continue;
					}
					match self.get(n) {
						b'.' => possible.push(n),
						b'#' => (),
						_ => possible.push(n),
					}
				}
				let last = possible.pop();
				for n in possible {
					let mut set = set.clone();
					set.insert(n);
					stack.push((n, set));
				}
				if let Some(n) = last {
					set.insert(n);
					pos = n;
				} else {
					break;
				}
			}
		}
		best - 1
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

const NEIGHBORS: [([i16; 2], u8); 4] = [
	([-1, 0], b'^'),
	([1, 0], b'v'),
	([0, -1], b'<'),
	([0, 1], b'>'),
];

impl Solution {
	fn get<I: TryInto<usize>>(&self, pos: [I; 2]) -> u8 {
		self.map.grid_get(pos).copied().unwrap_or(b'#')
	}
}
