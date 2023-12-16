use crate::helpers::*;

pub type A1 = u64;
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
		let mut total = 0;
		let mut current_hash = 0;
		for &b in &self.file[..self.file.len() - 1] {
			if b == b',' {
				total += current_hash;
				current_hash = 0;
			} else {
				hash_one(&mut current_hash, b);
			}
		}
		total += current_hash;
		total
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		let mut boxes = std::array::from_fn(|_| LensBox::default());
		let mut input = self.file.trim_ascii_end();
		while !input.is_empty() {
			lens_operation(&mut input, &mut boxes);
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

type LensBox = arrayvec::ArrayVec<Lens, 8>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Lens {
	label: [u8; 8],
	focal: u8,
}

impl Lens {
	fn new(label: [u8; 8], focal: u8) -> Self {
		Self { label, focal }
	}
}

fn lens_operation(step_and_rest: &mut &[u8], boxes: &mut [LensBox; 256]) {
	let mut label = [0; 8];
	let mut operation = 0;
	let mut hash = 0;
	for c in &mut label {
		let byte = *step_and_rest.take_first().unwrap();
		if byte & 0b01000000 != 0 {
			*c = byte;
			hash_one(&mut hash, byte);
		} else {
			operation = byte;
			break;
		}
	}

	if operation == b'=' {
		let focal_length = *step_and_rest.take_first().unwrap() - b'0';

		let b = &mut boxes[hash as usize];
		let mut replaced = false;
		for lens in b.iter_mut() {
			if lens.label == label {
				lens.focal = focal_length;
				replaced = true;
				break;
			}
		}
		if !replaced {
			b.push(Lens::new(label, focal_length));
		}
	} else {
		let b = &mut boxes[hash as usize];

		for (i, lens) in b.iter_mut().enumerate() {
			if lens.label == label {
				b.remove(i);
				break;
			}
		}
	}
}

fn focusing_power(boxes: &[LensBox]) -> usize {
	// println!("{}", boxes.iter().map(|b| b.len()).max().unwrap());
	boxes
		.iter()
		.enumerate()
		.flat_map(|(i, b)| {
			b.iter()
				.enumerate()
				.map(move |(j, &lens)| (i + 1) * (j + 1) * lens.focal as usize)
		})
		.sum_self()
}

fn hash_one(value: &mut u64, b: u8) {
	*value += b as u64;
	*value *= 17;
	*value %= 256;
}
