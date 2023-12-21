use std::convert::identity;

use crate::helpers::*;

pub type A1 = usize;
pub type A2 = usize;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	file: Vec<u8>,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		Self { file }
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		let mut grid = self.file.grid(identity);
		let size = grid.len() as isize;
		let start = [size / 2, size / 2];
		*grid.grid_get_mut(start).unwrap() = b'.';
		run(start, &grid, 64, None)[0].len()
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		let mut grid = self.file.grid(identity);

		let middle = (grid.len() as isize - 1) / 2;
		// assert_eq!(*grid.grid_get([middle, middle]).unwrap(), b'S');
		let start = [middle, middle];
		*grid.grid_get_mut(start).unwrap() = b'.';

		let middle_to_edge = (grid.len() - 1) / 2;
		let edge_to_edge = grid.len();

		// let full_plots_even = grid.iter().flatten().step_by(2).filter(is(&b'.')).count();
		// let full_plots_odd = grid
		// 	.iter()
		// 	.flatten()
		// 	.skip(1)
		// 	.step_by(2)
		// 	.filter(is(&b'.'))
		// 	.count();

		let sets = run(start, &grid, middle_to_edge + edge_to_edge, None);
		let once = sets[0].len();
		let sets = run(start, &grid, edge_to_edge, Some(sets));
		let twice = sets[0].len();
		let sets = run(start, &grid, edge_to_edge, Some(sets));
		let thrice = sets[0].len();
		// let sets = run(start, &grid, edge_to_edge, Some(sets));
		// let fourth = sets[0].len();

		// dbg!(thrice);
		let target = (STEPS - middle_to_edge) / edge_to_edge;
		let change_one = twice - once;
		let change_two = thrice - twice;
		let change_change = change_two - change_one;

		let mut total_plots = once;
		let mut change = change_one;
		for _ in 1..target {
			total_plots += change;
			change += change_change;
			// dbg!(total_plots);
		}

		total_plots
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

fn run(
	start: [isize; 2],
	grid: &Grid<u8>,
	steps: usize,
	init: Option<[HashSet<[isize; 2]>; 2]>,
) -> [HashSet<[isize; 2]>; 2] {
	let [mut stack, mut next_stack] =
		init.unwrap_or_else(|| [HashSet::from_iter([start]), HashSet::default()]);
	// println!("{start:?}, {steps}");

	for _ in 0..steps {
		// println!("{:?}", stack);
		next_stack.clear();
		for &pos in &stack {
			let y = pos[0];
			let x = pos[1];
			for [dy, dx] in [[-1, 0], [1, 0], [0, -1], [0, 1]] {
				let ny = y + dy;
				let nx = x + dx;

				let Some(&c) = grid.grid_get([
					ny.rem_euclid(grid.len() as isize),
					nx.rem_euclid(grid.len() as isize),
				]) else {
					continue;
				};
				if c == b'.' {
					next_stack.insert([ny, nx]);
				}
			}
		}
		swap(&mut stack, &mut next_stack);
	}
	// println!("{:?}", stack.iter().sorted().collect_vec());
	// println!("{}", stack.len());
	[stack, next_stack]
}

const STEPS: usize = 26_501_365;
