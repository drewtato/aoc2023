use crate::helpers::*;

pub type A1 = impl Display + Debug + Clone;
pub type A2 = impl Display + Debug + Clone;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	file: Vec<usize>,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let file = file
			.trim_ascii()
			.lines()
			.map(|line| {
				let numbers = line.delimiter(':').nth(1).unwrap();
				let [winning, have] = numbers
					.delimiter('|')
					.map(|list| {
						list.delimiter(' ')
							.filter(|s| !s.is_empty())
							.map(|s| s.parse().unwrap())
							.fold([false; 100], |mut map, number: usize| {
								map[number] = true;
								map
							})
					})
					.array()
					.unwrap();
				winning
					.into_iter()
					.zip(have)
					.filter(|&(a, b)| a && b)
					.count()
			})
			.collect();
		Self { file }
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		self.file
			.iter()
			.map(|&count| 2u32.pow(count as _) / 2)
			.sum_self()
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		let mut copies = [1u64; 10];
		self.file
			.iter()
			.map(|&count| {
				let &this_copies = copies.first().unwrap();
				copies.rotate_left(1);
				*copies.last_mut().unwrap() = 1;
				for c in &mut copies[..count] {
					*c += this_copies
				}
				// dbg_small!(this_copies);
				this_copies
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
