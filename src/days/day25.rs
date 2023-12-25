use std::collections::hash_map::Entry;

use arrayvec::ArrayVec;

use crate::helpers::*;

pub type A1 = usize;
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
		let mut components: HashMap<&[u8], HashSet<&[u8]>> = HashMap::new();
		for line in self.file.lines() {
			let (name, rest) = line.split_once(is(b':')).unwrap();
			let rest: ArrayVec<_, 10> = rest.trim_ascii().delimiter(' ').collect();
			components.entry(name).or_default().extend(rest.clone());

			for r in rest {
				components.entry(r).or_default().insert(name);
			}
		}

		let mut path = Vec::new();
		let &node = components.keys().next().unwrap();
		for _ in 0..3 {
			path.extend(find_longest_path(&components, node));
			remove_edges(&mut components, &path);
			path.clear();
		}

		let mut one_side = HashSet::new();
		let mut stack = vec![*components.keys().next().unwrap()];

		while let Some(node) = stack.pop() {
			one_side.insert(node);
			let Some(list) = components.remove(node) else {
				continue;
			};
			stack.extend(list);
		}

		one_side.len() * components.len()
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		"It's snowing ❄️"
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

fn find_longest_path<'a>(
	graph: &HashMap<&'a [u8], HashSet<&'a [u8]>>,
	start: &'a [u8],
) -> impl Iterator<Item = (&'a [u8], &'a [u8])> + 'a {
	let mut visited = HashMap::new();
	let mut queue = VecDeque::from_iter([start]);
	let mut end = start;
	while let Some(node) = queue.pop_front() {
		end = node;
		for &connection in &graph[&node] {
			let Entry::Vacant(v) = visited.entry(connection) else {
				continue;
			};
			v.insert(node);
			queue.push_back(connection);
		}
	}
	gen_iter(move || loop {
		let next = visited[&end];
		yield (end, next);
		end = next;
		if end == start {
			break;
		}
	})
}

fn remove_edges<'a>(
	graph: &mut HashMap<&'a [u8], HashSet<&'a [u8]>>,
	edges: &[(&'a [u8], &'a [u8])],
) {
	for &(a, b) in edges {
		graph.get_mut(a).unwrap().remove(b);
		graph.get_mut(b).unwrap().remove(a);
	}
}
