use crate::helpers::*;

pub type A1 = impl Display + Debug + Clone;
pub type A2 = impl Display + Debug + Clone;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	histories: Vec<Vec<i64>>,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let histories = file
			.trim_ascii_end()
			.lines()
			.map(|line| line.delimiter(' ').multi_parse().unwrap())
			.collect();
		Self { histories }
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		self.histories
			.iter()
			.map(|history| {
				let mut intermediates = vec![history.clone()];
				loop {
					let mut all_zero = true;
					let new = intermediates
						.last()
						.unwrap()
						.array_windows()
						.map(|&[a, b]| {
							let diff = b - a;
							if diff != 0 {
								all_zero = false;
							}
							diff
						})
						.collect_vec();

					if all_zero {
						break;
					}

					intermediates.push(new);
				}

				let mut generated = 0;
				for intermediate in intermediates.iter().rev() {
					let &last = intermediate.last().unwrap();
					generated += last;
				}
				generated
			})
			.sum_self()
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		self.histories
			.iter()
			.map(|history| {
				let mut intermediates = vec![history.clone()];
				loop {
					let mut all_zero = true;
					let new = intermediates
						.last()
						.unwrap()
						.array_windows()
						.map(|&[a, b]| {
							let diff = b - a;
							if diff != 0 {
								all_zero = false;
							}
							diff
						})
						.collect_vec();

					if all_zero {
						break;
					}

					intermediates.push(new);
				}

				let mut generated = 0;
				for intermediate in intermediates.iter().rev() {
					let &first = intermediate.first().unwrap();
					generated = first - generated;
				}
				generated
			})
			.sum_self()
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
