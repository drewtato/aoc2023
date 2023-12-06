#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]

use crate::helpers::*;

pub type A1 = impl Display + Debug + Clone;
pub type A2 = impl Display + Debug + Clone;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	times: Vec<u32>,
	distances: Vec<u32>,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let [l1, l2] = file
			.delimiter('\n')
			.take(2)
			.map(|line| {
				line.split(|c| !c.is_ascii_digit())
					.filter_empty()
					.multi_parse()
					.unwrap()
			})
			.array()
			.unwrap();
		Self {
			times: l1,
			distances: l2,
		}
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		let mut mult = 1;
		for (&time, &distance) in self.times.iter().zip(&self.distances) {
			let mut winners = 0;
			for held in 1..time {
				let remaining = time - held;
				if held * remaining > distance {
					winners += 1;
				}
			}
			mult *= winners;
		}
		mult
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		let time = self.times.iter().fold(0u64, |acc, &t| {
			acc * 10u64.pow(t.length(10) as _) + t as u64
		});

		let distance = self.distances.iter().fold(0u64, |acc, &t| {
			acc * 10u64.pow(t.length(10) as _) + t as u64
		});

		let mut winners = 0;
		for held in 1..time {
			let remaining = time - held;
			if held * remaining > distance {
				winners += 1;
			}
		}
		winners
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
