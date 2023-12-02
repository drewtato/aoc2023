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

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		Self { file }
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		let mut total = 0;
		let mut first = 0;
		let mut last = 0;
		for &byte in &self.file {
			if byte == b'\n' {
				total += first * 10 + last;
				first = 0;
				last = 0;
			}
			for &(number, value) in NUMBERS {
				if byte == number {
					if first == 0 {
						first = value;
					}
					last = value;
				}
			}
		}
		total
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		let mut total = 0;
		let mut first = 0;
		let mut last = 0;
		for offset in 0..self.file.len() {
			let slice = &self.file[offset..];
			match slice.first() {
				None => break,
				Some(b'\n') => {
					total += first * 10 + last;
					first = 0;
					last = 0;
				}
				Some(_) => {
					for &(number, value) in WORDS {
						if slice.starts_with(number) {
							if first == 0 {
								first = value;
							}
							last = value;
						}
					}
				}
			}
		}
		total
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

const NUMBERS: &[(u8, u64)] = &[
	(b'1', 1),
	(b'2', 2),
	(b'3', 3),
	(b'4', 4),
	(b'5', 5),
	(b'6', 6),
	(b'7', 7),
	(b'8', 8),
	(b'9', 9),
];

const WORDS: &[(&[u8], u64)] = &[
	(b"1", 1),
	(b"2", 2),
	(b"3", 3),
	(b"4", 4),
	(b"5", 5),
	(b"6", 6),
	(b"7", 7),
	(b"8", 8),
	(b"9", 9),
	(b"one", 1),
	(b"two", 2),
	(b"three", 3),
	(b"four", 4),
	(b"five", 5),
	(b"six", 6),
	(b"seven", 7),
	(b"eight", 8),
	(b"nine", 9),
];
