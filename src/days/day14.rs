use std::cell::Cell;
use std::fmt::Write;
use std::hash::Hasher;

use crate::helpers::*;

pub type A1 = impl Display + Debug + Clone;
pub type A2 = impl Display + Debug + Clone;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	grid: Vec<Space>,
	grid_row_len: isize,
	rounds: Vec<usize>,
	inner_cubes: Vec<usize>,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let mut this = Self::from_file(&file);
		this.gen_closest();
		this
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		self.tilt_north();
		self.weight()
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		for _ in 0..PARTIAL {
			self.cycle();
		}

		let mut seen =
			std::collections::HashMap::with_capacity_and_hasher(100, IdentityHasher::default());

		for i in PARTIAL.. {
			self.cycle();
			let hash = self.hash();
			if let Some(last) = seen.insert(hash, i) {
				let cycle_length = i - last;
				let remaining = CYCLES - i - 1;
				let cycle_index = remaining % cycle_length;
				for _ in 0..cycle_index {
					self.cycle();
				}
				return self.weight();
			}
		}
		unreachable!()
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

const ROUND: u8 = b'O';
const CUBE: u8 = b'#';
const VACANT: u8 = b'.';
const CYCLES: usize = 1_000_000_000;
const PARTIAL: usize = 144;

impl Solution {
	fn from_file(file: &[u8]) -> Self {
		let grid_row_len = file.lines().next().unwrap().len() as isize + 2;
		let mut grid = Vec::with_capacity((grid_row_len * grid_row_len) as usize);
		let mut rounds = Vec::with_capacity(grid.capacity() / 5);
		let mut inner_cubes = Vec::with_capacity(rounds.capacity() + grid_row_len as usize * 4);

		// Top border
		grid.extend(
			repeat_iter(Space {
				space_type: Cube,
				closest: Closest::default(),
			})
			.take(grid_row_len as usize),
		);

		for line in file.lines() {
			let space_iter = line.iter().map(|&b| b.try_into().unwrap());
			// West and east borders
			for mut space in [Space::new(Cube)]
				.into_iter()
				.chain(space_iter)
				.chain([Space::new(Cube)])
			{
				match space.space_type {
					Round => {
						rounds.push(grid.len());
						space.space_type = Vacant;
					}
					Cube => inner_cubes.push(grid.len()),
					_ => (),
				}
				grid.push(space);
			}
		}

		// Bottom border
		grid.extend(
			repeat_iter(Space {
				space_type: Cube,
				closest: Closest::default(),
			})
			.take(grid_row_len as usize),
		);

		Self {
			grid,
			grid_row_len,
			rounds,
			inner_cubes,
		}
	}

	fn gen_closest(&mut self) {
		let mut grid = take(&mut self.grid);
		let grid_row_len = self.grid_row_len;

		let grid_cells = Cell::from_mut(grid.as_mut_slice()).as_slice_of_cells();
		for (y, row) in (0..).zip(grid_cells.chunks(grid_row_len as usize)) {
			for (x, space) in (0..).zip(row) {
				let coords = [y, x];
				let space_type = space.get().space_type;
				let mut closest = Closest::default();

				if space_type != Cube {
					for (direction, clo) in [
						([-1, 0], &mut closest.north),
						([0, -1], &mut closest.west),
						([1, 0], &mut closest.south),
						([0, 1], &mut closest.east),
					] {
						let mut coords = coords;
						loop {
							*clo += 1;
							coords = add(coords, direction);
							let Some(index) = self.coords_to_index(coords) else {
								break;
							};
							let Some(space) = grid_cells.get(index) else {
								break;
							};
							if space.get().space_type == Cube {
								break;
							}
						}
					}
				}

				space.set(Space {
					space_type,
					closest,
				});
			}
		}

		self.grid = grid;
	}

	fn cycle(&mut self) {
		self.clear_cube_stacks();

		// self.print_grid();
		self.tilt_north();

		// self.print_grid();
		self.tilt_west();

		// self.print_grid();
		self.tilt_south();

		// self.print_grid();
		self.tilt_east();
	}

	fn clear_cube_stacks(&mut self) {
		for &cube_index in &self.inner_cubes {
			let cube = &mut self.grid[cube_index];
			cube.closest_clear();
		}
		for cube in self.grid.iter_mut().take(self.grid_row_len as usize) {
			cube.closest_clear();
		}
		for cube in self.grid.iter_mut().rev().take(self.grid_row_len as usize) {
			cube.closest_clear();
		}
		for cube in self.grid.chunks_mut(self.grid_row_len as usize) {
			cube.first_mut().unwrap().closest_clear();
			cube.last_mut().unwrap().closest_clear();
		}
	}

	fn tilt<A, B>(&mut self, mut closest_cube_coords: A, mut modify_coords: B)
	where
		A: FnMut(Closest, [isize; 2]) -> [isize; 2],
		B: FnMut(&mut Closest, [isize; 2]) -> [isize; 2],
	{
		let mut rounds = take(&mut self.rounds);
		let mut grid = take(&mut self.grid);

		for round in &mut rounds {
			let mut coords = self.index_to_coords(*round);
			// println!("{coords:?}");

			let closest_cube_coords = closest_cube_coords(grid[*round].closest, coords);
			// println!("{closest_cube_coords:?}");
			let closest_cube_index = self.coords_to_index_unchecked(closest_cube_coords);
			// println!("{:?}", grid[closest_cube_index as usize]);

			coords = modify_coords(&mut grid[closest_cube_index].closest, closest_cube_coords);

			*round = self.coords_to_index_unchecked(coords);
		}

		self.rounds = rounds;
		self.grid = grid;
	}

	fn tilt_north(&mut self) {
		self.tilt(
			|closest, [y, x]| {
				let distance = closest.north as isize;
				[y - distance, x]
			},
			|closest, [y, x]| {
				let stacks = &mut closest.south;
				*stacks += 1;
				[y + *stacks as isize, x]
			},
		)
	}

	fn tilt_south(&mut self) {
		self.tilt(
			|closest, [y, x]| {
				let distance = closest.south as isize;
				[y + distance, x]
			},
			|closest, [y, x]| {
				let stacks = &mut closest.north;
				*stacks += 1;
				[y - *stacks as isize, x]
			},
		)
	}

	fn tilt_west(&mut self) {
		self.tilt(
			|closest, [y, x]| {
				let distance = closest.west as isize;
				[y, x - distance]
			},
			|closest, [y, x]| {
				let stacks = &mut closest.east;
				*stacks += 1;
				[y, x + *stacks as isize]
			},
		)
	}

	fn tilt_east(&mut self) {
		self.tilt(
			|closest, [y, x]| {
				let distance = closest.east as isize;
				[y, x + distance]
			},
			|closest, [y, x]| {
				let stacks = &mut closest.west;
				*stacks += 1;
				[y, x - *stacks as isize]
			},
		)
	}

	fn weight(&self) -> u32 {
		self.rounds
			.iter()
			.map(|&round| {
				let [y, _] = self.index_to_coords(round);
				(self.grid_row_len - y - 1) as u32
			})
			.sum()
	}

	fn hash(&mut self) -> u64 {
		self.rounds.sort_unstable();
		let mut hasher = rustc_hash::FxHasher::default();
		for &u in &self.rounds {
			hasher.write_usize(u);
		}
		hasher.finish()
	}

	fn index_to_coords(&self, index: usize) -> [isize; 2] {
		let index = index as isize;
		[index / self.grid_row_len, index % self.grid_row_len]
	}

	fn coords_to_index(&self, coords: [isize; 2]) -> Option<usize> {
		let [y, x] = coords;
		if x > self.grid_row_len || x < 0 {
			return None;
		}
		Some((y * self.grid_row_len + x) as usize)
	}

	#[track_caller]
	fn coords_to_index_unchecked(&self, coords: [isize; 2]) -> usize {
		let [y, x] = coords;
		if cfg!(debug_assertions) && x > self.grid_row_len
			|| x < 0 || y < 0
			|| y > self.grid_row_len
		{
			panic!("coordinates {coords:?} is out of bounds");
		}
		(y * self.grid_row_len + x) as usize
	}

	#[allow(dead_code)]
	fn print_grid(&self) {
		let mut lines = Vec::new();
		for row in self.grid.chunks(self.grid_row_len as usize) {
			let mut line = String::new();
			for space in row {
				write!(line, "{space}").unwrap();
			}
			lines.push(line);
		}
		for &round in &self.rounds {
			let [y, x] = self.index_to_coords(round);
			let [y, x] = [y as usize, x as usize];
			lines[y].replace_range(x..x + 1, "O");
		}
		for line in lines {
			println!("{line}");
		}
		println!("------");
	}

	// fn get(&self, coords: [isize; 2]) -> Option<&Space> {
	// 	let index = self.coords_to_index(coords)?;
	// 	self.grid.get(index)
	// }

	// fn get_mut(&mut self, coords: [isize; 2]) -> Option<&mut Space> {
	// 	let index = self.coords_to_index(coords)?;
	// 	self.grid.get_mut(index)
	// }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum SpaceType {
	Round,
	Cube,
	#[default]
	Vacant,
}
use SpaceType::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Space {
	space_type: SpaceType,
	closest: Closest,
}
impl Space {
	fn new(space_type: SpaceType) -> Self {
		Self {
			space_type,
			closest: Closest::default(),
		}
	}

	fn closest_clear(&mut self) {
		self.closest = Closest::default()
	}
}

/// For round and empty spaces, this is the distance to the nearest cube. For cube spaces, this is
/// the number of rounds stacked upon it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Closest {
	north: u8,
	west: u8,
	south: u8,
	east: u8,
}

impl TryFrom<u8> for Space {
	type Error = ();

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		let space_type = match value {
			VACANT => Vacant,
			ROUND => Round,
			CUBE => Cube,
			_ => return Err(()),
		};

		Ok(Self::new(space_type))
	}
}

impl Display for Space {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		Display::fmt(&self.space_type, f)
	}
}

impl Display for SpaceType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_char(match self {
			Round => ROUND as char,
			Cube => CUBE as char,
			Vacant => VACANT as char,
		})
	}
}
