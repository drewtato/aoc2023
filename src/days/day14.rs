use std::cell::Cell;

use crate::helpers::*;

pub type A1 = impl Display + Debug + Clone;
pub type A2 = impl Display + Debug + Clone;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	grid: Vec<Vec<Cell<u8>>>,
}

const ROUND: u8 = b'O';
// const CUBE: u8 = b'#';
const EMPTY: u8 = b'.';
const CYCLES: usize = 1_000_000_000;
const PARTIAL: usize = 1_000;

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		Self {
			grid: file.grid(Cell::new),
		}
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		self.tilt_north();

		// print_grid(&self.grid);

		self.sum()
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		let mut cycles_iter = 0..CYCLES;
		for _ in cycles_iter.by_ref().take(PARTIAL) {
			self.cycle();
		}

		let mut weights = vec![self.sum()];
		for _ in cycles_iter {
			self.cycle();

			let sum = self.sum();
			if weights[0] == sum {
				break;
			} else {
				weights.push(sum);
			}
		}
		let cycle_length = weights.len();
		let offset = (CYCLES - PARTIAL) % cycle_length;
		weights[offset]
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
	fn cycle(&self) {
		self.tilt_north();
		self.tilt_west();
		self.tilt_south();
		self.tilt_east();
	}
	fn tilt_north(&self) {
		for (y, row) in self.grid.iter().enumerate() {
			for (x, cell) in row.iter().enumerate() {
				let mut y = y as isize;
				let x = x as isize;
				if cell.get() == ROUND
				// && grid_get(&self.grid, [y - 1, x]).map(|c| c.get()) == Some(EMPTY)
				{
					cell.set(EMPTY);
					y -= 1;
					while grid_get(&self.grid, [y, x]).map(|c| c.get()) == Some(EMPTY) {
						y -= 1;
					}
					y += 1;
					grid_get(&self.grid, [y, x]).unwrap().set(ROUND);
				}
			}
		}
	}

	fn tilt_south(&self) {
		for (y, row) in self.grid.iter().enumerate().rev() {
			for (x, cell) in row.iter().enumerate() {
				let mut y = y as isize;
				let x = x as isize;
				if cell.get() == ROUND
				// && grid_get(&self.grid, [y + 1, x]).map(|c| c.get()) == Some(EMPTY)
				{
					cell.set(EMPTY);
					y += 1;
					while grid_get(&self.grid, [y, x]).map(|c| c.get()) == Some(EMPTY) {
						y += 1;
					}
					y -= 1;
					grid_get(&self.grid, [y, x]).unwrap().set(ROUND);
				}
			}
		}
	}

	fn tilt_west(&self) {
		for (y, row) in self.grid.iter().enumerate() {
			for (x, cell) in row.iter().enumerate() {
				let y = y as isize;
				let mut x = x as isize;
				if cell.get() == ROUND
				// && grid_get(&self.grid, [y, x - 1]).map(|c| c.get()) == Some(EMPTY)
				{
					cell.set(EMPTY);
					x -= 1;
					while grid_get(&self.grid, [y, x]).map(|c| c.get()) == Some(EMPTY) {
						x -= 1;
					}
					x += 1;
					grid_get(&self.grid, [y, x]).unwrap().set(ROUND);
				}
			}
		}
	}

	fn tilt_east(&self) {
		for (y, row) in self.grid.iter().enumerate() {
			for (x, cell) in row.iter().enumerate().rev() {
				let y = y as isize;
				let mut x = x as isize;
				if cell.get() == ROUND
				// && grid_get(&self.grid, [y, x + 1]).map(|c| c.get()) == Some(EMPTY)
				{
					cell.set(EMPTY);
					x += 1;
					while grid_get(&self.grid, [y, x]).map(|c| c.get()) == Some(EMPTY) {
						x += 1;
					}
					x -= 1;
					grid_get(&self.grid, [y, x]).unwrap().set(ROUND);
				}
			}
		}
	}

	fn sum(&mut self) -> usize {
		let height = self.grid.len();

		self.grid
			.iter()
			.enumerate()
			.flat_map(|(y, row)| row.iter().map(move |cell| (y, cell)))
			.map(|(y, cell)| if cell.get() == ROUND { height - y } else { 0 })
			.sum_self()
	}
}

// fn print_grid(grid: &Grid<Cell<u8>>) {
// 	for row in grid {
// 		for cell in row {
// 			print!("{}", DisplayByte(cell.get()));
// 		}
// 		println!();
// 	}
// 	println!("-------");
// }
