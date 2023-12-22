use crate::helpers::*;

pub type A1 = usize;
pub type A2 = usize;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	bricks: Vec<Brick>,
	map: Vec<[[usize; FLOOR_SIZE]; FLOOR_SIZE]>,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let mut bricks = Vec::default();
		for line in file.lines() {
			let (a, b) = line.split_once(is(b'~')).unwrap();
			let a = a
				.delimiter(',')
				.map(|n| n.parse().unwrap())
				.array()
				.unwrap();
			let [x, y, z] = a;
			let start = Point { z, y, x };
			let b = b
				.delimiter(',')
				.map(|n| n.parse().unwrap())
				.array()
				.unwrap();
			let [x, y, z] = b;
			let end = Point { z, y, x };
			let mut both = [start, end];
			both.sort_unstable();
			let [start, end] = both;
			bricks.push(Brick { start, end });
		}
		bricks.sort_unstable();
		let mut map: Vec<[[usize; FLOOR_SIZE]; FLOOR_SIZE]> =
			vec![[[FLOOR; FLOOR_SIZE]; FLOOR_SIZE]];
		for (i, b) in bricks.iter().enumerate() {
			for p in b.cubes() {
				let slice = if let Some(slice) = map.get_mut(p.z) {
					slice
				} else {
					map.resize(p.z + 1, [[EMPTY; FLOOR_SIZE]; FLOOR_SIZE]);
					&mut map[p.z]
				};
				slice[p.y][p.x] = i;
			}
		}
		// print_map(&map);
		let mut this = Self { bricks, map };

		this.gravity();

		this
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		let mut safe = HashSet::from_iter(0..self.bricks.len());

		for brick in &self.bricks {
			if brick.start.z == 1 {
				continue;
			}
			// dbg!(brick);
			if brick.start.z == brick.end.z {
				let mut solid = false;
				let mut one = None;
				for mut p in brick.cubes() {
					p.z -= 1;
					let resting_on = p.index_map(&self.map).unwrap();
					if resting_on == EMPTY {
						continue;
					}
					if let Some(existing) = one {
						if existing != resting_on {
							solid = true;
							break;
						}
					} else {
						one = Some(resting_on);
					}
				}
				if !solid {
					// dbg!(one);
					safe.remove(&one.unwrap());
				}
			} else {
				let mut p = brick.start;
				p.z -= 1;
				let resting = p.index_map(&self.map).unwrap();
				// dbg!(resting);
				safe.remove(&resting);
			}
		}

		safe.len()
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		(0..self.bricks.len())
			.map(|i| {
				let mut other = self.clone();
				other.disintegrate(i);
				other.gravity()
			})
			.sum()
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
	fn gravity(&mut self) -> usize {
		let mut fallen = 0;
		for (i, brick) in self.bricks.iter_mut().enumerate() {
			if brick.start.z <= 1 {
				continue;
			}

			let fall_distance = if brick.start.z == brick.end.z {
				(1..)
					.find(|fall_distance| {
						brick
							.cubes()
							.any(|p| self.map[p.z - fall_distance][p.y][p.x] != EMPTY)
					})
					.unwrap()
			} else {
				(1..)
					.find(|fall_distance| {
						let mut p = brick.start;
						p.z -= fall_distance;
						p.index_map(&self.map).unwrap() != EMPTY
					})
					.unwrap()
			} - 1;

			if fall_distance > 0 {
				fallen += 1;
				for cube in brick.cubes() {
					*cube.index_map_mut(&mut self.map).unwrap() = EMPTY;
				}
				brick.start.z -= fall_distance;
				brick.end.z -= fall_distance;
				for cube in brick.cubes() {
					*cube.index_map_mut(&mut self.map).unwrap() = i;
				}
			}
		}
		fallen
	}

	fn disintegrate(&mut self, i: usize) {
		let b = &mut self.bricks[i];
		for p in b.cubes() {
			*p.index_map_mut(&mut self.map).unwrap() = EMPTY
		}
		b.start.z = 0;
		b.end.z = 0;
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Brick {
	start: Point,
	end: Point,
}

impl Brick {
	fn cubes(self) -> impl Iterator<Item = Point> {
		gen_iter(move || {
			if self.start.z != self.end.z {
				for z in self.start.z..self.end.z + 1 {
					yield Point {
						z,
						y: self.start.y,
						x: self.start.x,
					}
				}
			} else if self.start.y != self.end.y {
				for y in self.start.y..self.end.y + 1 {
					yield Point {
						z: self.start.z,
						y,
						x: self.start.x,
					}
				}
			} else if self.start.x != self.end.x {
				for x in self.start.x..self.end.x + 1 {
					yield Point {
						z: self.start.z,
						y: self.start.y,
						x,
					}
				}
			} else {
				// One cube brick
				yield Point {
					z: self.start.z,
					y: self.start.y,
					x: self.start.x,
				}
			}
		})
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
	z: usize,
	y: usize,
	x: usize,
}

impl Point {
	fn index_map(self, map: &[[[usize; FLOOR_SIZE]; FLOOR_SIZE]]) -> Option<usize> {
		map.get(self.z).map(|slice| slice[self.y][self.x])
	}
	fn index_map_mut(self, map: &mut [[[usize; FLOOR_SIZE]; FLOOR_SIZE]]) -> Option<&mut usize> {
		map.get_mut(self.z).map(|slice| &mut slice[self.y][self.x])
	}
}

const FLOOR: usize = usize::MAX - 1;
const EMPTY: usize = usize::MAX;
const FLOOR_SIZE: usize = 10;

fn print_map(map: &[[[usize; FLOOR_SIZE]; FLOOR_SIZE]]) {
	for slice in map.iter().rev() {
		for row in slice {
			print!("[");
			for &c in row.iter().skip(1) {
				match c {
					EMPTY => print!("_ "),
					FLOOR => print!("=="),
					_ => print!("{c} "),
				}
			}
			print!("]");
		}
		println!();
	}
}
