use crate::helpers::*;

pub type A1 = u64;
pub type A2 = u64;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	records: Vec<Record>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Record {
	row: Vec<Spring>,
	groups: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Spring {
	Intact,
	Broken,
	Unknown,
}
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use Spring::*;

impl From<u8> for Spring {
	fn from(value: u8) -> Self {
		match value {
			b'.' => Intact,
			b'#' => Broken,
			b'?' => Unknown,
			_ => panic!("Bad spring character {}", DisplayByte(value)),
		}
	}
}

// impl Display for Spring {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		match self {
// 			Intact => f.write_str("."),
// 			Broken => f.write_str("#"),
// 			Unknown => f.write_str("?"),
// 		}
// 	}
// }

// struct DisplaySpringSlice<'a>(&'a [Spring]);

// impl Display for DisplaySpringSlice<'_> {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		for spring in self.0 {
// 			write!(f, "{}", spring)?;
// 		}
// 		Ok(())
// 	}
// }

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let records = file
			.lines()
			.map(|line| {
				let (row, groups) = line.split_once(is(b' ')).unwrap();
				let row = row.iter().cloned().map(Spring::from).collect();
				let groups = groups.delimiter(',').multi_parse().unwrap();
				Record { row, groups }
			})
			.collect();
		Self { records }
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		let mut memo = Memo::new();
		self.records
			.iter()
			.map(|Record { row, groups }| arrangements(row, groups, &mut memo))
			.sum()
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		self.records
			.par_iter()
			.map_with(Memo::new(), |memo, Record { row, groups }| {
				let mut new_row = Vec::with_capacity(row.len() * 5 + 4);
				new_row.extend_from_slice(row);
				for _ in 1..5 {
					new_row.push(Unknown);
					new_row.extend_from_slice(row);
				}

				let mut new_groups = Vec::with_capacity(groups.len() * 5);
				for _ in 0..5 {
					new_groups.extend_from_slice(groups);
				}

				arrangements(&new_row, &new_groups, memo)
			})
			.sum()
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
type Memo = Vec<Option<u64>>;
fn arrangements(row: &[Spring], groups: &[u8], memo: &mut Memo) -> u64 {
	memo.clear();
	let memo_row = groups.len() + 1;
	let needed_len = (row.len() + 1) * memo_row;
	memo.resize(needed_len, None);
	set_memo(memo, 0, 0, memo_row, 1);
	arrangements_inner(row, groups, memo, memo_row)
}

fn get_memo(memo: &Memo, y: usize, x: usize, memo_row: usize) -> Option<u64> {
	// println!("{y} {x}");
	memo[y * memo_row + x]
}

fn set_memo(memo: &mut Memo, y: usize, x: usize, memo_row: usize, value: u64) {
	// println!("set {y} {x} = {value}");
	memo[y * memo_row + x] = Some(value)
}

fn arrangements_inner(
	mut row: &[Spring],
	mut groups: &[u8],
	memo: &mut Memo,
	memo_row: usize,
) -> u64 {
	let memo_key = (row.len(), groups.len());
	if let Some(already_found) = get_memo(memo, memo_key.0, memo_key.1, memo_row) {
		// println!("Memo invoked at {:?}: {}", memo_key, already_found);
		return already_found;
	}

	let mut calculate = || {
		// let (orig_row, orig_groups) = (row, groups);

		let Some(&first) = row.take_first() else {
			// There's definitely more groups, so this fails
			return 0;
		};

		let mut possibles = 0;

		if let Intact | Unknown = first {
			possibles += arrangements_inner(row, groups, memo, memo_row)
		}

		if let Broken | Unknown = first {
			let mut broken = || {
				let Some(&group) = groups.take_first() else {
					// There are no more groups to match this broken spring.
					return 0;
				};
				for _ in 1..group {
					let Some(&next) = row.take_first() else {
						// There are no more springs to fulfill this group.
						return 0;
					};
					match next {
						// This isn't broken, so the group is not fulfilled.
						Intact => return 0,
						// An unknown must be broken to fulfill the group.
						Broken | Unknown => (),
					}
				}
				// If this isn't the final, there needs to be one intact to break the group.
				if let Some(&next) = row.take_first() {
					if next == Broken {
						return 0;
					}
				} else {
					// It's the final and as long as there's no groups left, it's complete
					if groups.is_empty() {
						return 1;
					} else {
						return 0;
					}
				}
				arrangements_inner(row, groups, memo, memo_row)
			};
			possibles += broken();
		}

		possibles
	};

	let ans = calculate();
	set_memo(memo, memo_key.0, memo_key.1, memo_row, ans);
	ans
}
