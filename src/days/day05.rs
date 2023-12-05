use crate::helpers::*;

pub type A1 = impl Display + Debug + Clone;
pub type A2 = impl Display + Debug + Clone;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	seeds: Vec<u64>,
	maps: Vec<Vec<[u64; 3]>>,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let mut fiter = file.trim_ascii().delimiter("\n\n");
		let seeds = fiter
			.next()
			.unwrap()
			.delimiter([' ', '\n'])
			.filter(|s| !s.is_empty())
			.filter(|c| c[0].is_ascii_digit())
			.multi_parse()
			.unwrap();

		let maps = fiter
			.map(|group| {
				group
					.delimiter([' ', '\n'])
					.filter(|s| !s.is_empty())
					.filter(|c| c[0].is_ascii_digit())
					.map(|item| item.parse().unwrap())
					.array_chunks()
					.sorted_by_key(|[_, b, _]| *b)
					.collect_vec()
			})
			.collect_vec();

		Self { seeds, maps }
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		self.seeds
			.iter()
			.map(|&seed| {
				let mut current = seed;
				for map in &self.maps {
					let next = match map.binary_search_by_key(&current, |[_, b, _]| *b) {
						Ok(index) => {
							if map[index][2] > 0 {
								current - map[index][1] + map[index][0]
							} else {
								current
							}
						}
						Err(index) => {
							if index == 0 {
								current
							} else {
								let index = index - 1;
								if map[index][1] + map[index][2] > current {
									current - map[index][1] + map[index][0]
								} else {
									current
								}
							}
						}
					};
					current = next;
				}
				current
			})
			.min()
			.unwrap()
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		use rayon::prelude::*;
		self.seeds
			// .chunks(2)
			.par_chunks(2)
			.flat_map(|chunk| {
				let &[a, b] = chunk else { unreachable!() };
				a..(a + b)
			})
			.map(|seed| {
				let mut current = seed;
				for map in &self.maps {
					let next = match map.binary_search_by_key(&current, |[_, b, _]| *b) {
						Ok(index) => {
							if map[index][2] > 0 {
								current - map[index][1] + map[index][0]
							} else {
								current
							}
						}
						Err(index) => {
							if index == 0 {
								current
							} else {
								let index = index - 1;
								if map[index][1] + map[index][2] > current {
									current - map[index][1] + map[index][0]
								} else {
									current
								}
							}
						}
					};
					current = next;
				}
				current
			})
			.min()
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
