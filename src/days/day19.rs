use std::ops::{Index, IndexMut, Range};

use crate::helpers::*;

pub type A1 = u32;
pub type A2 = u64;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	workflows: HashMap<Name, Workflow>,
	parts: Vec<Part>,
}
impl Solution {
	fn part_is_accepted(&self, part: Part) -> bool {
		let mut current = START;
		while current != ACCEPTED && current != REJECTED {
			let workflow = &self.workflows[&current];
			current = workflow.process(part);
		}
		current == ACCEPTED
	}
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let mut lines = file.delimiter(b'\n');
		lines.next_back().unwrap();

		let workflows = lines
			.by_ref()
			.take_while(|l| !l.is_empty())
			.map(|line| {
				let (name, mut rest) = line.split_once(is(b'{')).unwrap();
				let name = Name::new(name);
				rest.take_last().unwrap();
				let workflow = Workflow::new(rest);
				(name, workflow)
			})
			.collect();

		let parts = lines
			.map(|mut line| {
				line.take_first().unwrap();
				line.take_last().unwrap();
				Part::new(line)
			})
			.collect();

		Self { workflows, parts }
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		self.parts
			.iter()
			.filter(|&&part| self.part_is_accepted(part))
			.map(|part| part.0.into_iter().sum_self())
			.sum()
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		let all_possible = MegaPart::new(1..4001);
		let mut stack = vec![(START, all_possible)];
		let mut accepted = 0;
		while let Some((name, part)) = stack.pop() {
			stack.extend(part.advance(name, &self.workflows).into_iter().filter(
				|&(name, part)| match name {
					ACCEPTED => {
						accepted += part.count();
						false
					}
					REJECTED => false,
					_ => true,
				},
			));
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

#[derive(Debug, Clone)]
struct Workflow {
	rules: ArrayVec<Rule, 4>,
	fallback: Name,
}
impl Workflow {
	fn new(rules: &[u8]) -> Self {
		let mut rules = rules.delimiter(b',');
		let fallback = Name::new(rules.next_back().unwrap());
		let rules = rules.map(Rule::new).collect();
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
	number: u32,
	destination: Name,
}
impl Rule {
	fn new(mut rule: &[u8]) -> Self {
		let category = Category::new(*rule.take_first().unwrap());
		let operation = Operation::new(*rule.take_first().unwrap());
		let (n, d) = rule.split_once(is(b':')).unwrap();
		let number = n.parse().unwrap();
		let destination = Name::new(d);
		Self {
			category,
			operation,
			number,
			destination,
		}
	}

	fn process(self, part: Part) -> Option<Name> {
		let cat_num = part[self.category];
		self.operation
			.compare(cat_num, self.number)
			.then_some(self.destination)
	}

	fn split(self, mut part: MegaPart) -> (Option<MegaPart>, Option<(Name, MegaPart)>) {
		let mut split_part = part;
		let part_cat = &mut part[self.category];
		let split_part_cat = &mut split_part[self.category];
		match self.operation {
			Less => todo!(),
			Greater => todo!(),
		}
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
	fn new(cat: u8) -> Category {
		match cat {
			b'x' => XtremelyCoolLooking,
			b'm' => Musical,
			b'a' => Aerodynamic,
			b's' => Shiny,
			_ => panic!("invalid category {:?}", DisplayByte(cat)),
		}
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
	fn new(op: u8) -> Operation {
		match op {
			b'<' => Less,
			b'>' => Greater,
			_ => panic!("invalid operation {:?}", DisplayByte(op)),
		}
	}

	fn compare(self, a: u32, b: u32) -> bool {
		match self {
			Less => a < b,
			Greater => a > b,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Name([u8; 3]);

const START: Name = Name(*b"in\0");
const ACCEPTED: Name = Name(*b"A\0\0");
const REJECTED: Name = Name(*b"R\0\0");

impl Name {
	fn new(name: &[u8]) -> Self {
		match *name {
			[a, b, c] => Self([a, b, c]),
			[a, b] => Self([a, b, 0]),
			[a] => Self([a, 0, 0]),
			_ => panic!("invalid name {:?}", DisplaySlice(name)),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Part([u32; 4]);

impl Part {
	fn new(line: &[u8]) -> Self {
		let categories = line
			.delimiter(b',')
			.map(|cat| cat[2..].parse().unwrap())
			.array()
			.unwrap();
		Self(categories)
	}
}

impl Index<Category> for Part {
	type Output = u32;

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
struct MegaPart([(u32, u32); 4]);
impl MegaPart {
	fn new(range: Range<u32>) -> Self {
		Self([(range.start, range.end); 4])
	}

	fn advance(
		mut self,
		name: Name,
		workflows: &HashMap<Name, Workflow>,
	) -> impl IntoIterator<Item = (Name, Self)> {
		let workflow = &workflows[&name];
		let mut new_parts = ArrayVec::<_, 5>::new();

		for &rule in &workflow.rules {
			let (same, diff) = rule.split(self);
			if let Some(same) = same {
				self = same
			} else {
				return new_parts;
			}
			new_parts.extend(diff);
		}

		new_parts.push((workflow.fallback, self));
		new_parts
	}

	fn count(&self) -> u64 {
		todo!()
	}
}

impl Index<Category> for MegaPart {
	type Output = (u32, u32);

	fn index(&self, index: Category) -> &Self::Output {
		match index {
			XtremelyCoolLooking => &self.0[0],
			Musical => &self.0[1],
			Aerodynamic => &self.0[2],
			Shiny => &self.0[3],
		}
	}
}

impl IndexMut<Category> for MegaPart {
	fn index_mut(&mut self, index: Category) -> &mut Self::Output {
		match index {
			XtremelyCoolLooking => &mut self.0[0],
			Musical => &mut self.0[1],
			Aerodynamic => &mut self.0[2],
			Shiny => &mut self.0[3],
		}
	}
}
