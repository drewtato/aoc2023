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
		PlotExplorer::new(start, &grid).run(64)
	}

	fn part_two(&mut self, d: u8) -> Self::AnswerTwo {
		let mut grid = self.file.grid(identity);

		let middle = (grid.len() as isize - 1) / 2;
		let start = [middle, middle];
		// assert_eq!(*grid.grid_get(start).unwrap(), b'S');
		*grid.grid_get_mut(start).unwrap() = b'.';

		let middle_to_edge = (grid.len() - 1) / 2;
		let edge_to_edge = grid.len();

		let mut plot_explorer = PlotExplorer::new(start, &grid);
		let zero = plot_explorer.run(middle_to_edge);
		let once = plot_explorer.run(edge_to_edge);
		let twice = plot_explorer.run(edge_to_edge);
		let thrice = plot_explorer.run(edge_to_edge);
		// let sets = run(start, &grid, edge_to_edge, Some(sets));
		// let fourth = sets[0].len();

		if d > 0 {
			println!("zero: {zero}");
			println!("once: {once}");
			println!("twice: {twice}");
			println!("thrice: {thrice}");
		}

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

const STEPS: usize = 26_501_365;
const NEIGHBORS: [[i16; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];

#[derive(Debug, Clone)]
struct PlotExplorer<'a> {
	live: Vec<[i16; 2]>,
	next_live: Vec<[i16; 2]>,
	seen: HashSet<[i16; 2]>,
	grid: &'a Grid<u8>,
	steps: usize,
	next_steps: usize,
}

impl<'a> PlotExplorer<'a> {
	fn new(start: [isize; 2], grid: &'a Grid<u8>) -> Self {
		let start = start.map(|n| n as i16);
		let mut live = Vec::with_capacity(1_000);
		live.push(start);
		let next_live = Vec::with_capacity(1_000);
		let mut seen = HashSet::with_capacity(100_000);
		seen.insert(start);
		Self {
			live,
			next_live,
			seen,
			grid,
			steps: 1,
			next_steps: 0,
		}
	}

	fn run(&mut self, steps: usize) -> usize {
		for _i in 0..steps {
			// dbg!(i, self.live.len(), self.next_live.len(), self.seen.len());
			self.next_live.clear();
			for &[y, x] in &self.live {
				for [dy, dx] in NEIGHBORS {
					let new_pos = [y + dy, x + dx];
					let grid_pos = new_pos.map(|p| p.rem_euclid(self.grid.len() as i16));
					if *self.grid.grid_get(grid_pos).unwrap() == b'#' || !self.seen.insert(new_pos)
					{
						continue;
					}
					self.next_live.push(new_pos);
				}
			}
			self.next_steps += self.next_live.len();
			swap(&mut self.steps, &mut self.next_steps);
			swap(&mut self.live, &mut self.next_live);
		}
		self.steps
	}
}
