#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]

use crate::helpers::*;

pub type A1 = impl Display + Debug + Clone;
pub type A2 = impl Display + Debug + Clone;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	file: Vec<u8>,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(mut file: Vec<u8>, _: u8) -> Self {
		Self { file }
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		let mut total = 0;
		for step in self.file.trim_ascii_end().delimiter(',') {
			let value = hash_algorithm(step);
			total += value as u32;
		}
		total
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		let mut boxes: [Vec<(&[u8], u8)>; 256] = from_fn_array(|_| Vec::new());
		for step in self.file.trim_ascii_end().delimiter(',') {
			if let Some((label, number)) = step.split_once(is(&b'=')) {
				let hash = hash_algorithm(label);
				let focal_length: u8 = number.parse().unwrap();

				let b = &mut boxes[hash as usize];
				let mut replaced = false;
				for (i, lens) in b.iter_mut().enumerate() {
					if lens.0 == label {
						lens.1 = focal_length;
						replaced = true;
						break;
					}
				}
				if !replaced {
					b.push((label, focal_length));
				}
			} else {
				let label = &step[..step.len() - 1];
				let hash = hash_algorithm(label);
				let b = &mut boxes[hash as usize];

				for (i, lens) in b.iter_mut().enumerate() {
					if lens.0 == label {
						b.remove(i);
						break;
					}
				}
			}
		}
		focusing_power(&boxes)
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

fn focusing_power(boxes: &[Vec<(&[u8], u8)>]) -> usize {
	boxes
		.iter()
		.enumerate()
		.flat_map(|(i, b)| {
			b.iter()
				.enumerate()
				.map(move |(j, &(_, focal_length))| (i + 1) * (j + 1) * focal_length as usize)
		})
		.sum_self()
}

fn hash_algorithm<'a>(step: impl IntoIterator<Item = &'a u8>) -> u8 {
	let mut value = 0;
	for &b in step {
		value += b as u32;
		value *= 17;
		value %= 256;
	}
	value as u8
}
