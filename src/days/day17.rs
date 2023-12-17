use crate::helpers::*;

pub type A1 = impl Display + Debug + Clone;
pub type A2 = impl Display + Debug + Clone;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	map: Vec<u8>,
	row_len: usize,
	col_len: usize,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let row_len = file.iter().position(is(b'\n')).unwrap();
		let col_len = file.len() / (row_len + 1);
		Self {
			map: file,
			row_len,
			col_len,
		}
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		let start = [0, 0];
		let mut queue = MinHeap::from_iter([Position {
			heat: 0,
			coords: start,
			direction: [1, 0],
			straights: 2,
		}]);
		let end = [self.col_len as isize - 1, self.row_len as isize - 1];

		let mut visited = HashMap::new();
		// let mut path = HashMap::new();

		while let Some(position) = queue.pop() {
			// println!("{:?}", position);

			let old_heat = visited
				.entry((position.coords, position.straights, position.direction))
				.or_insert(u64::MAX);
			if *old_heat <= position.heat {
				continue;
			} else {
				*old_heat = position.heat;
				// path.insert(position.coords, position.direction);
			}

			if position.coords == end {
				return position.heat;
			}

			queue.extend(self.forward(position));
			queue.extend(self.left(position));
			queue.extend(self.right(position));
		}

		panic!("no path found p1")
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		let start = [0, 0];
		let mut queue = MinHeap::from_iter([
			Position {
				heat: 0,
				coords: start,
				direction: [1, 0],
				straights: 10,
			},
			Position {
				heat: 0,
				coords: start,
				direction: [0, 1],
				straights: 10,
			},
		]);
		let end = [self.col_len as isize - 1, self.row_len as isize - 1];

		let mut visited = HashMap::new();
		// let mut path = HashMap::new();

		while let Some(position) = queue.pop() {
			// println!("{:?}", position);
			// if position.coords == [59, 5] {
			// 	println!("{position:?}");
			// }

			let old_heat = visited
				.entry((position.coords, position.straights, position.direction))
				.or_insert(u64::MAX);
			if *old_heat <= position.heat {
				continue;
			} else {
				*old_heat = position.heat;
				// path.insert(position.coords, position.direction);
			}

			if position.coords == end {
				if position.straights > 6 {
					continue;
				}
				// std::mem::take(self).print_path(&visited, position);
				return position.heat;
			}

			queue.extend(self.forward2(position));
			queue.extend(self.left2(position));
			queue.extend(self.right2(position));
		}

		panic!("no path found p2")
		// 1386 too low
		// 1402 too high
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

type Coord = isize;

impl Solution {
	fn get(&self, position: [Coord; 2]) -> Option<u64> {
		let [y, x] = position;
		let [y, x] = [y as usize, x as usize];
		if x >= self.row_len || y >= self.col_len {
			None
		} else {
			self.map
				.get(y * (self.row_len + 1) + x)
				.map(|&heat| (heat - b'0') as u64)
		}
	}

	fn get_mut(&mut self, position: [Coord; 2]) -> Option<&mut u8> {
		let [y, x] = position;
		let [y, x] = [y as usize, x as usize];
		if x >= self.row_len || y >= self.col_len {
			None
		} else {
			self.map.get_mut(y * (self.row_len + 1) + x)
		}
	}

	fn forward(&self, position: Position) -> Option<Position> {
		let Position {
			heat,
			coords,
			direction,
			straights,
		} = position;

		let Some(new_straights) = straights.checked_sub(1) else {
			return None;
		};

		let new_coords = add(coords, direction);
		let add_heat = self.get(new_coords)?;

		Some(Position {
			heat: heat + add_heat,
			coords: new_coords,
			direction,
			straights: new_straights,
		})
	}

	fn left(&self, position: Position) -> Option<Position> {
		let Position {
			heat,
			coords,
			direction,
			..
		} = position;

		let new_direction = match direction {
			[0, 1] => [-1, 0],
			[-1, 0] => [0, -1],
			[0, -1] => [1, 0],
			[1, 0] => [0, 1],
			_ => unreachable!(),
		};

		let new_coords = add(coords, new_direction);
		let add_heat = self.get(new_coords)?;

		Some(Position {
			heat: heat + add_heat,
			coords: new_coords,
			direction: new_direction,
			straights: 2,
		})
	}

	fn right(&self, position: Position) -> Option<Position> {
		let Position {
			heat,
			coords,
			direction,
			..
		} = position;

		let new_direction = match direction {
			[-1, 0] => [0, 1],
			[0, -1] => [-1, 0],
			[1, 0] => [0, -1],
			[0, 1] => [1, 0],
			_ => unreachable!(),
		};
		let new_coords = add(coords, new_direction);
		let add_heat = self.get(new_coords)?;

		Some(Position {
			heat: heat + add_heat,
			coords: new_coords,
			direction: new_direction,
			straights: 2,
		})
	}

	fn forward2(&self, mut position: Position) -> Option<Position> {
		let Position {
			heat,
			coords,
			direction,
			straights,
		} = &mut position;

		if *straights == 0 {
			return None;
		} else if *straights > 6 {
			for _ in 6..*straights {
				*straights -= 1;
				*coords = add(*coords, *direction);
				*heat += self.get(*coords)?;
			}
		} else {
			*straights -= 1;
			*coords = add(*coords, *direction);
			*heat += self.get(*coords)?;
		}

		Some(position)
	}

	fn left2(&self, position: Position) -> Option<Position> {
		let Position {
			heat,
			coords,
			direction,
			straights,
		} = position;

		if straights > 6 {
			return None;
		}

		let new_direction = match direction {
			[0, 1] => [-1, 0],
			[-1, 0] => [0, -1],
			[0, -1] => [1, 0],
			[1, 0] => [0, 1],
			_ => unreachable!(),
		};

		let new_coords = add(coords, new_direction);
		let add_heat = self.get(new_coords)?;

		Some(Position {
			heat: heat + add_heat,
			coords: new_coords,
			direction: new_direction,
			straights: 9,
		})
	}

	fn right2(&self, position: Position) -> Option<Position> {
		let Position {
			heat,
			coords,
			direction,
			straights,
		} = position;

		if straights > 6 {
			return None;
		}

		let new_direction = match direction {
			[-1, 0] => [0, 1],
			[0, -1] => [-1, 0],
			[1, 0] => [0, -1],
			[0, 1] => [1, 0],
			_ => unreachable!(),
		};
		let new_coords = add(coords, new_direction);
		let add_heat = self.get(new_coords)?;

		Some(Position {
			heat: heat + add_heat,
			coords: new_coords,
			direction: new_direction,
			straights: 9,
		})
	}

	#[allow(dead_code)]
	fn print_path(
		mut self,
		visited: &HashMap<([Coord; 2], u8, [Coord; 2]), u64>,
		mut current: Position,
	) {
		let start = [0, 0];
		while current.coords != start {
			let block = self.get_mut(current.coords).unwrap();
			current.heat -= (*block - b'0') as u64;
			current.coords = sub(current.coords, current.direction);
			*block = match current.direction {
				[-1, 0] => b'^',
				[0, -1] => b'<',
				[1, 0] => b'v',
				[0, 1] => b'>',
				_ => unreachable!(),
			};
			if current.straights == 9 {
				'a: for s in 0..10 {
					for dir in [[-1, 0], [0, -1], [1, 0], [0, 1]] {
						if let Some(&old) = visited.get(&(current.coords, s, dir)) {
							if old == current.heat {
								current.direction = dir;
								current.straights = s;
								break 'a;
							}
						}
					}
				}
			} else {
				current.straights += 1;
			}
		}
		assert_eq!(current.heat, 0);

		println!("{}", DisplaySlice(&self.map))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
	heat: u64,
	coords: [Coord; 2],
	direction: [Coord; 2],
	straights: u8,
}
