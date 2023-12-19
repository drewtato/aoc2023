use std::collections::hash_map::Entry;
use std::ops::{Index, IndexMut, Range};

use crate::helpers::*;

pub type A1 = u64;
pub type A2 = u64;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	workflows: Vec<Workflow>,
	start: Name,
	accept: Name,
	reject: Name,
	file: Vec<u8>,
	offset: usize,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let mut lines = file.delimiter(b'\n');
		lines.next_back().unwrap();
		let mut workflows = Vec::with_capacity(1000);
		for _ in 0..3 {
			workflows.push(Default::default());
		}

		let start = Name::new(0);
		let accept = Name::new(1);
		let reject = Name::new(2);
		let mut name_map: HashMap<&[u8], Name> = HashMap::with_capacity(1000);
		name_map.extend([(START, start), (ACCEPTED, accept), (REJECTED, reject)]);

		let mut lines = file.as_slice();
		loop {
			if lines[0] == b'\n' {
				break;
			}
			let name_end = lines.iter().position(|&b| b == b'{').unwrap();

			let name = lines.take(..name_end).unwrap();
			lines.take_first().unwrap();

			let workflow = Workflow::new(&mut lines, &mut name_map, &mut workflows);

			let name = Name::add_to_name_map(name, &mut name_map, &mut workflows);
			workflows[name.0] = workflow;
		}

		lines.take(..2).unwrap();

		let offset = file.len() - lines.len() - 1;

		Self {
			workflows,
			start,
			accept,
			reject,
			file,
			offset,
		}
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		parse_parts(&self.file[self.offset..])
			.filter(|&part| self.part_is_accepted(part))
			.map(|part| part.0.into_iter().map(|d| d as u64).sum::<u64>())
			.sum()
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		let all_possible = PartRange::new(1..4001);
		let mut stack = vec![(self.start, all_possible)];
		let mut accepted = 0;
		while let Some((name, part)) = stack.pop() {
			stack.extend(
				part.advance(name, self)
					.into_iter()
					.filter(|&(name, part)| match name {
						n if n == self.accept => {
							accepted += part.count();
							false
						}
						n if n == self.reject => false,
						_ => true,
					}),
			);
		}
		accepted
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

fn parse_parts(mut lines: &[u8]) -> impl Iterator<Item = Part> + '_ {
	lines.take_last().unwrap();
	std::iter::from_coroutine(move || {
		let start = lines.take(..3).unwrap();
		debug_assert_eq!(DisplaySlice(start), DisplaySlice(b"{x=".as_slice()));
		loop {
			let mut x = 0;
			loop {
				let &d = lines.take_first().unwrap();
				if d == b',' {
					break;
				}
				x *= 10;
				x += (d - b'0') as Number;
			}

			lines.take(..2).unwrap();
			let mut m = 0;
			loop {
				let &d = lines.take_first().unwrap();
				if d == b',' {
					break;
				}
				m *= 10;
				m += (d - b'0') as Number;
			}

			lines.take(..2).unwrap();
			let mut a = 0;
			loop {
				let &d = lines.take_first().unwrap();
				if d == b',' {
					break;
				}
				a *= 10;
				a += (d - b'0') as Number;
			}

			lines.take(..2).unwrap();
			let mut s = 0;
			loop {
				let &d = lines.take_first().unwrap();
				if d == b'}' {
					break;
				}
				s *= 10;
				s += (d - b'0') as Number;
			}

			yield Part::new(x, m, a, s);
			if lines.take(..4).is_none() {
				break;
			}
		}
	})
}

type Number = u16;

impl Solution {
	fn part_is_accepted(&self, part: Part) -> bool {
		let mut current = self.start;
		while current != self.accept && current != self.reject {
			let workflow = &self[current];
			current = workflow.process(part);
		}
		current == self.accept
	}
}

impl Index<Name> for Solution {
	type Output = Workflow;

	fn index(&self, index: Name) -> &Self::Output {
		&self.workflows[index.0]
	}
}

impl IndexMut<Name> for Solution {
	fn index_mut(&mut self, index: Name) -> &mut Self::Output {
		&mut self.workflows[index.0]
	}
}

#[derive(Debug, Clone, Default)]
struct Workflow {
	rules: ArrayVec<Rule, 4>,
	fallback: Name,
}
impl Workflow {
	fn new<'a>(
		rest: &mut &'a [u8],
		name_map: &mut HashMap<&'a [u8], Name>,
		workflows: &mut Vec<Workflow>,
	) -> Workflow {
		let mut rules = ArrayVec::new();

		loop {
			let Some(rule) = Rule::new(rest, name_map, workflows) else {
				break;
			};
			rules.push(rule);
		}

		let raw_fallback_pos = rest.iter().position(is(b'}')).unwrap();
		let raw_fallback = rest.take(..raw_fallback_pos).unwrap();
		rest.take(..2).unwrap();

		let fallback = Name::add_to_name_map(raw_fallback, name_map, workflows);
		Self { rules, fallback }
	}

	fn process(&self, part: Part) -> Name {
		for &rule in &self.rules {
			if let Some(dest) = rule.process(part) {
				return dest;
			}
		}
		self.fallback
	}
}

#[derive(Debug, Clone, Copy)]
struct Rule {
	category: Category,
	operation: Operation,
	number: Number,
	destination: Name,
}
impl Rule {
	fn new<'a>(
		rule: &mut &'a [u8],
		name_map: &mut HashMap<&'a [u8], Name>,
		workflows: &mut Vec<Workflow>,
	) -> Option<Rule> {
		let Some(category) = Category::new(rule[0]) else {
			return None;
		};
		let Some(operation) = Operation::new(rule[1]) else {
			return None;
		};
		rule.take(..2).unwrap();

		let mut number = 0;
		loop {
			let &d = rule.take_first().unwrap();
			if d == b':' {
				break;
			}
			number *= 10;
			number += (d - b'0') as Number;
		}

		let d_pos = rule.iter().position(is(b',')).unwrap();
		let d = rule.take(..d_pos).unwrap();
		rule.take_first().unwrap();

		let destination = Name::add_to_name_map(d, name_map, workflows);
		Some(Self {
			category,
			operation,
			number,
			destination,
		})
	}

	fn process(self, part: Part) -> Option<Name> {
		let cat_num = part[self.category];
		self.operation
			.compare(cat_num, self.number)
			.then_some(self.destination)
	}

	fn split(self, mut part: PartRange) -> (Option<PartRange>, Option<(Name, PartRange)>) {
		let mut split_part = part;
		let part_cat = &mut part[self.category];
		let split_part_cat = &mut split_part[self.category];
		match self.operation {
			Less => {
				part_cat.0 = part_cat.0.max(self.number);
				split_part_cat.1 = split_part_cat.1.min(self.number);
			}
			Greater => {
				split_part_cat.0 = split_part_cat.0.max(self.number + 1);
				part_cat.1 = part_cat.1.min(self.number + 1);
			}
		}
		let part = (part_cat.0 < part_cat.1).then_some(part);
		let split_part =
			(split_part_cat.0 < split_part_cat.1).then_some((self.destination, split_part));
		(part, split_part)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Category {
	XtremelyCoolLooking,
	Musical,
	Aerodynamic,
	Shiny,
}
use Category::*;

impl Category {
	fn new(cat: u8) -> Option<Category> {
		Some(match cat {
			b'x' => XtremelyCoolLooking,
			b'm' => Musical,
			b'a' => Aerodynamic,
			b's' => Shiny,
			_ => return None,
		})
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Operation {
	Less,
	Greater,
}
use arrayvec::ArrayVec;
use Operation::*;

impl Operation {
	fn new(op: u8) -> Option<Operation> {
		Some(match op {
			b'<' => Less,
			b'>' => Greater,
			_ => return None,
		})
	}

	fn compare(self, a: Number, b: Number) -> bool {
		match self {
			Less => a < b,
			Greater => a > b,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Name(usize);

const START: &[u8] = b"in";
const ACCEPTED: &[u8] = b"A";
const REJECTED: &[u8] = b"R";

impl Name {
	fn new(name: usize) -> Self {
		Self(name)
	}

	fn add_to_name_map<'a>(
		raw_name: &'a [u8],
		name_map: &mut HashMap<&'a [u8], Name>,
		workflows: &mut Vec<Workflow>,
	) -> Self {
		match name_map.entry(raw_name) {
			Entry::Occupied(o) => *o.get(),
			Entry::Vacant(v) => {
				let i = *v.insert(Name::new(workflows.len()));
				workflows.push(Default::default());
				i
			}
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Part([Number; 4]);

impl Part {
	fn new(x: Number, m: Number, a: Number, s: Number) -> Self {
		Self([x, m, a, s])
	}
}

impl Index<Category> for Part {
	type Output = Number;

	fn index(&self, index: Category) -> &Self::Output {
		match index {
			XtremelyCoolLooking => &self.0[0],
			Musical => &self.0[1],
			Aerodynamic => &self.0[2],
			Shiny => &self.0[3],
		}
	}
}

impl IndexMut<Category> for Part {
	fn index_mut(&mut self, index: Category) -> &mut Self::Output {
		match index {
			XtremelyCoolLooking => &mut self.0[0],
			Musical => &mut self.0[1],
			Aerodynamic => &mut self.0[2],
			Shiny => &mut self.0[3],
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PartRange([(Number, Number); 4]);
impl PartRange {
	fn new(range: Range<Number>) -> Self {
		Self([(range.start, range.end); 4])
	}

	fn advance(mut self, name: Name, sol: &Solution) -> impl IntoIterator<Item = (Name, Self)> {
		let workflow = &sol[name];
		let mut new_parts = ArrayVec::<_, 5>::new();

		for &rule in &workflow.rules {
			let (same, diff) = rule.split(self);
			let Some(same) = same else {
				new_parts.extend(diff);
				return new_parts;
			};
			self = same;
			new_parts.extend(diff);
		}

		new_parts.push((workflow.fallback, self));
		new_parts
	}

	fn count(&self) -> u64 {
		self.0
			.into_iter()
			.map(|(a, b)| a.abs_diff(b) as u64)
			.product()
	}
}

impl Index<Category> for PartRange {
	type Output = (Number, Number);

	fn index(&self, index: Category) -> &Self::Output {
		match index {
			XtremelyCoolLooking => &self.0[0],
			Musical => &self.0[1],
			Aerodynamic => &self.0[2],
			Shiny => &self.0[3],
		}
	}
}

impl IndexMut<Category> for PartRange {
	fn index_mut(&mut self, index: Category) -> &mut Self::Output {
		match index {
			XtremelyCoolLooking => &mut self.0[0],
			Musical => &mut self.0[1],
			Aerodynamic => &mut self.0[2],
			Shiny => &mut self.0[3],
		}
	}
}
