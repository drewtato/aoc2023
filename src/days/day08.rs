use crate::helpers::*;

pub type A1 = impl Display + Debug + Clone;
pub type A2 = impl Display + Debug + Clone;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	turns: Vec<Turn>,
	map: HashMap<Node, [Node; 2]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Turn {
	Left,
	Right,
}
use Turn::*;

impl TryFrom<u8> for Turn {
	type Error = ();

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		Ok(match value {
			b'L' => Left,
			b'R' => Right,
			_ => return Err(()),
		})
	}
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node(DisplaySlice<[u8; 3]>);

impl Node {
	fn new(node: [u8; 3]) -> Self {
		Self(DisplaySlice(node))
	}

	fn is_start(self) -> bool {
		self.0 .0[2] == b'A'
	}

	fn is_end(self) -> bool {
		self.0 .0[2] == b'Z'
	}
}

impl Debug for Node {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.0)
	}
}

impl<'a> TryFrom<&'a [u8]> for Node {
	type Error = ();

	fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
		Ok(match value {
			&[a, b, c] => Self::new([a, b, c]),
			_ => return Err(()),
		})
	}
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let (turns_line, map_lines) = file.split_once(is(&b'\n')).unwrap();
		let turns = turns_line
			.iter()
			.map(|&b| b.try_into().unwrap())
			.collect_vec();
		let map = map_lines
			.trim_ascii()
			.lines()
			.map(|line| {
				let node = line[..3].try_into().unwrap();
				let left = line[7..][..3].try_into().unwrap();
				let right = line[12..][..3].try_into().unwrap();
				(node, [left, right])
			})
			.collect();
		Self { turns, map }
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		let start = Node::new(*b"AAA");
		let end = Node::new(*b"ZZZ");

		self.distance_to_end(start, |node| node == end)
			.map_or(-1, |(n, _)| n as isize)
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		self.map
			.keys()
			.filter(|node| node.is_start())
			.map(|&node| {
				let (offset, end) = self.distance_to_end(node, Node::is_end).unwrap();
				let cycle = self.distance_to_end(end, Node::is_end).unwrap().0;
				(offset, cycle)
			})
			// .inspect(|(offset, cycle)| {
			// 	dbg!(offset, cycle);
			// })
			.reduce(|(offset_a, cycle_a), (offset_b, cycle_b)| {
				let mut offset_a = offset_a as isize;
				let mut offset_b = offset_b as isize;
				let cycle_a = cycle_a as isize;
				let cycle_b = cycle_b as isize;
				loop {
					match offset_a - offset_b {
						negative @ ..=-1 => {
							let multiples = ((-negative - 1) / cycle_a) + 1;
							offset_a += multiples * cycle_a;
						}
						0 => break (offset_a as usize, lcm(offset_a, offset_b) as usize),
						positive @ 1.. => {
							let multiples = ((positive - 1) / cycle_b) + 1;
							offset_b += multiples * cycle_b;
						}
					}
				}
			})
			.unwrap()
			.0
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

impl Solution {
	fn distance_to_end(
		&self,
		start: Node,
		mut end_condition: impl FnMut(Node) -> bool,
	) -> Option<(usize, Node)> {
		if !self.map.contains_key(&start) {
			return None;
		}

		let mut current = start;

		for (i, &turn) in self.turns.iter().cycle().enumerate() {
			current = match turn {
				Left => self.map[&current][0],
				Right => self.map[&current][1],
			};

			if end_condition(current) {
				return Some((i + 1, current));
			}
		}
		None
	}
}
