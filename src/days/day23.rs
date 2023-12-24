use std::collections::hash_map::Entry;
use std::ops::{Add, AddAssign, Index, IndexMut};

use arrayvec::ArrayVec;

use crate::helpers::*;

pub type A1 = i64;
pub type A2 = i64;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	graph: ArrayVec<Node, 50>,
	end: (usize, i64),
	max_len: i64,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let start = [0, 1];

		let mut graph: ArrayVec<(Node, [Coord; 2]), 50> = ArrayVec::new();
		graph.push(([(BLOCKED, 0); 4], start));
		graph[0].0[Down].0 = 1;

		let mut positions: HashMap<[Coord; 2], usize> = HashMap::with_capacity(50);
		positions.insert(start, 0);

		let row_len = file.len().sqrt();

		let mut stack: ArrayVec<(usize, Direction), 50> = ArrayVec::new();
		stack.push((0, Down));

		let mut end = (0, 0);
		let mut max_len = 0;

		while let Some((mut node_index, mut direction)) = stack.pop() {
			loop {
				let len = graph.len();
				let &mut (ref mut node, pos) = &mut graph[node_index];

				let next_int = find_intersection(&file, row_len, pos, direction);
				let (new_pos, new_dir, length) = match next_int {
					Ok((new_pos, new_dir, length)) => (new_pos, new_dir, length),
					Err(length) => {
						end = (node_index, length);
						break;
					}
				};
				max_len = max_len.max(length);

				let subnode = (node_index, -length);
				let new_node_index = match positions.entry(new_pos) {
					Entry::Occupied(o) => {
						let index = *o.get();
						node[direction] = (index, length);
						graph[index].0[new_dir.invert()] = subnode;
						index
					}
					Entry::Vacant(v) => {
						let index = len;
						v.insert(index);
						node[direction] = (index, length);
						let mut node = [(BLOCKED, 0); 4];
						node[new_dir.invert()] = subnode;
						graph.push((node, new_pos));
						index
					}
				};

				for t in new_dir.turns() {
					match get(&file, row_len, new_pos + t).try_into().unwrap() {
						Slope(d) if d == t => (),
						_ => continue,
					}
					stack.push((new_node_index, t));
				}

				match get(&file, row_len, new_pos + new_dir).try_into().unwrap() {
					Slope(d) if d == new_dir => {
						node_index = new_node_index;
						direction = new_dir;
					}
					_ => break,
				}
			}
		}

		// for item in &graph {
		// 	println!("{item:?}");
		// }
		// println!("End: {end:?}");

		let graph = graph.into_iter().map(|(a, _)| a).collect();

		Self {
			graph,
			end,
			max_len,
		}
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		let mut stack = ArrayVec::<_, 50>::new();
		stack.push((0, 0, Down));

		let mut visited = [false; 50];

		let end_index = self.end.0;
		let max_len = self.max_len;
		let mut nodes_remaining = self.graph.len() as i64 - 2;

		let mut max = 0;
		while let Some((mut node_index, mut distance, mut direction)) = stack.pop() {
			visited[node_index] = false;
			nodes_remaining += 1;
			if direction == Up {
				continue;
			}

			loop {
				let (next_index, next_distance) = self.graph[node_index][direction];
				if (next_index == BLOCKED)
					|| visited[next_index]
					|| next_distance.is_negative()
					|| nodes_remaining * max_len < max - distance - next_distance
				{
					direction.cw();
					if direction == Up {
						break;
					}
					continue;
				}

				if next_index == end_index {
					distance += next_distance;
					max = max.max(distance);
					direction.cw();
					if direction == Up {
						break;
					}
					distance -= next_distance;
					continue;
				}

				direction.cw();
				stack.push((node_index, distance, direction));
				visited[node_index] = true;
				nodes_remaining -= 1;

				node_index = next_index;
				distance += next_distance;
				direction = Up;
			}
		}
		max + self.end.1
	}

	fn part_two(&mut self, d: u8) -> Self::AnswerTwo {
		let mut stack = ArrayVec::<_, 50>::new();
		stack.push((0, 0, Down));

		let mut visited = [false; 50];

		let end_index = self.end.0;
		// let max_len = self.max_len;
		let mut nodes_remaining = self.graph.len() as i64 - 2;

		let mut max_nodes = 0;
		let mut max = 0;
		while let Some((mut node_index, mut distance, mut direction)) = stack.pop() {
			visited[node_index] = false;
			nodes_remaining += 1;
			if direction == Left {
				continue;
			}

			loop {
				let (next_index, next_distance) = self.graph[node_index][direction];
				if (next_index == BLOCKED) || visited[next_index]
				// || nodes_remaining * max_len < max - distance - next_distance
				{
					direction.cw();
					if direction == Left {
						break;
					}
					continue;
				}

				let next_distance = next_distance.abs();

				if next_index == end_index {
					distance += next_distance;
					if d > 0 && distance > max {
						max_nodes = nodes_remaining;
					}
					max = max.max(distance);
					direction.cw();
					if direction == Left {
						break;
					}
					distance -= next_distance;
					continue;
				}

				direction.cw();
				stack.push((node_index, distance, direction));
				visited[node_index] = true;
				nodes_remaining -= 1;

				node_index = next_index;
				distance += next_distance;
				direction = Left;
			}
		}

		if d > 0 {
			println!("{max_nodes}");
		}
		max + self.end.1
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

type Coord = i16;
type Node = [(usize, i64); 4];
const BLOCKED: usize = usize::MAX;

impl Solution {}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
enum Direction {
	Up,
	Right,
	Down,
	Left,
}
use Direction::*;

impl Direction {
	fn offset(self) -> [Coord; 2] {
		match self {
			Up => [-1, 0],
			Right => [0, 1],
			Down => [1, 0],
			Left => [0, -1],
		}
	}

	fn turns(self) -> [Self; 2] {
		match self {
			Up | Down => [Right, Left],
			Right | Left => [Up, Down],
		}
	}

	fn invert(self) -> Self {
		match self {
			Up => Down,
			Right => Left,
			Down => Up,
			Left => Right,
		}
	}

	fn cw(&mut self) {
		*self = match self {
			Up => Right,
			Right => Down,
			Down => Left,
			Left => Up,
		};
	}
}

impl Add<Direction> for [Coord; 2] {
	type Output = Self;

	fn add(self, rhs: Direction) -> Self::Output {
		let other = rhs.offset();
		[self[0] + other[0], self[1] + other[1]]
	}
}

impl AddAssign<Direction> for [Coord; 2] {
	fn add_assign(&mut self, rhs: Direction) {
		*self = *self + rhs;
	}
}

impl<T> Index<Direction> for [T; 4] {
	type Output = T;

	fn index(&self, index: Direction) -> &Self::Output {
		self.get(match index {
			Up => 0,
			Right => 1,
			Down => 2,
			Left => 3,
		})
		.unwrap()
	}
}

impl<T> IndexMut<Direction> for [T; 4] {
	fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
		self.get_mut(match index {
			Up => 0,
			Right => 1,
			Down => 2,
			Left => 3,
		})
		.unwrap()
	}
}

fn find_intersection(
	file: &[u8],
	row_len: usize,
	mut pos: [Coord; 2],
	mut direction: Direction,
) -> Result<([Coord; 2], Direction, i64), i64> {
	let mut length = 0;
	'l: loop {
		length += 1;
		pos += direction;
		let [t1, t2] = direction.turns();
		let result = [direction, t1, t2]
			.into_iter()
			.map::<(Trail, _), _>(|d| (get(file, row_len, pos + d).try_into().unwrap(), d));

		for (trail, dir) in result {
			match trail {
				Path => {
					direction = dir;
					continue 'l;
				}
				Forest => continue,
				Slope(_) => {
					pos += dir;
					pos += dir;
					length += 2;
					return Ok((pos, dir, length));
				}
			}
		}
		return Err(length);
	}
}

fn get(file: &[u8], row_len: usize, pos: [Coord; 2]) -> u8 {
	*file
		.get(pos[0] as usize * (row_len + 1) + pos[1] as usize)
		.unwrap_or(&b'#')
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Hash)]
enum Trail {
	Path,
	Forest,
	Slope(Direction),
}
use Trail::*;

impl TryFrom<u8> for Trail {
	type Error = ();

	fn try_from(value: u8) -> Result<Self, Self::Error> {
		Ok(match value {
			b'.' => Path,
			b'#' => Forest,
			b'^' => Slope(Up),
			b'>' => Slope(Right),
			b'v' => Slope(Down),
			b'<' => Slope(Left),
			_ => return Err(()),
		})
	}
}
