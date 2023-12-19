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
		Step::area(self.file.lines().map(|line| Step::try_from(line).unwrap()))
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		Step::area(
			self.file
				.lines()
				.map(|line| Step::alt_try_from(line).unwrap()),
		)
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

type Coord = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Step {
	direction: Direction,
	distance: Coord,
}

impl Step {
	fn alt_try_from(value: &[u8]) -> Result<Step, ()> {
		let (&[dist_hex @ .., f, _], _) = value.split_last_chunk::<7>().unwrap();
		let direction = Direction::from_number(f - b'0').unwrap();

		let mut distance = 0;
		for h in dist_hex {
			distance *= 16;
			distance += hex_to_number(h).unwrap() as i64;
		}

		Ok(Self {
			direction,
			distance,
		})
	}

	fn area(steps: impl Iterator<Item = Step>) -> Coord {
		let mut pos = [0, 0];
		let mut area = 0;
		let mut step_count = 0;

		for step in steps {
			step_count += step.distance;
			let offset = step.direction.to_offset();
			let offset = mul(offset, [step.distance as Coord, step.distance as Coord]);
			let new_pos = add(pos, offset);
			area += pos[0] * new_pos[1] - pos[1] * new_pos[0];
			pos = new_pos;
		}

		area.abs() / 2 + step_count / 2 + 1
	}
}

impl TryFrom<&[u8]> for Step {
	type Error = ();

	fn try_from(value: &[u8]) -> Result<Step, ()> {
		let direction = value[0].try_into()?;
		let (d, _c) = value[2..].split_once(is(b' ')).ok_or(())?;
		let distance = d.parse().ok_or(())?;
		// let color = c[2..][..6].try_into()?;
		Ok(Self {
			direction,
			distance,
		})
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
	Up,
	Left,
	Down,
	Right,
}
use Direction::*;

impl Direction {
	fn to_offset(self) -> [Coord; 2] {
		match self {
			Up => [-1, 0],
			Left => [0, 1],
			Down => [1, 0],
			Right => [0, -1],
		}
	}

	fn from_number(number: u8) -> Result<Self, ()> {
		Ok(match number {
			0 => Right,
			1 => Down,
			2 => Left,
			3 => Up,
			_ => return Err(()),
		})
	}
}

impl TryFrom<u8> for Direction {
	type Error = ();

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		let d = match value {
			b'U' => Up,
			b'L' => Left,
			b'D' => Down,
			b'R' => Right,
			_ => return Err(()),
		};
		Ok(d)
	}
}

// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
// struct Color([u8; 3]);

// impl TryFrom<&[u8]> for Color {
// 	type Error = ();

// 	fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
// 		let arr: [u8; 6] = value.try_into().map_err(|_| ())?;
// 		let [a, b, c, d, e, f] = arr.try_map(hex_to_number)?;
// 		let r = a * 16 + b;
// 		let g = c * 16 + d;
// 		let b = e * 16 + f;
// 		Ok(Color([r, g, b]))
// 	}
// }

fn hex_to_number(n: u8) -> Result<u8, ()> {
	Ok(match n {
		b'0'..=b'9' => n - b'0',
		b'a'..=b'f' => n - b'a' + 10,
		_ => return Err(()),
	})
}
