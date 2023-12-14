#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::convert::identity;

use crate::helpers::*;

pub type A1 = u64;
pub type A2 = A1;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	seeds: Vec<u64>,
	maps: Vec<Vec<[u64; 3]>>,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let mut fiter = file.trim_ascii().delimiter("\n\n");
		let seeds = fiter
			.next()
			.unwrap()
			.delimiter([' ', '\n'])
			.filter_empty()
			.filter(|c| c[0].is_ascii_digit())
			.multi_parse()
			.unwrap();

		let maps = fiter
			.map(|group| {
				group
					.delimiter([' ', '\n'])
					.filter_empty()
					.filter(|c| c[0].is_ascii_digit())
					.map(|item| item.parse().unwrap())
					.array_chunks()
					.sorted_by_key(|[_, b, _]| *b)
					.collect_vec()
			})
			.collect_vec();

		Self { seeds, maps }
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		self.seeds
			.iter()
			.map(|&seed| {
				let mut current = seed;
				for map in &self.maps {
					let next = match map.binary_search_by_key(&current, |[_, b, _]| *b) {
						Ok(index) => {
							if map[index][2] > 0 {
								current - map[index][1] + map[index][0]
							} else {
								current
							}
						}
						Err(index) => {
							if index == 0 {
								current
							} else {
								let index = index - 1;
								if map[index][1] + map[index][2] > current {
									current - map[index][1] + map[index][0]
								} else {
									current
								}
							}
						}
					};
					current = next;
				}
				current
			})
			.min()
			.unwrap()
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		let mut ranges = self
			.seeds
			.array_chunks()
			.map(|&[start, length]| [start, start + length])
			.collect_vec();
		let mut next_ranges = Vec::new();

		for map in &self.maps {
			// println!("Ranges: {ranges:?}");
			for &[mut start, end] in &ranges {
				// println!("Range: {:?}", [start, end]);
				let index = map
					.binary_search_by_key(&start, |[_, b, _]| *b)
					.unwrap_or_else(identity)
					.saturating_sub(1);

				if index == map.len() {
					next_ranges.push([start, end]);
				} else {
					for line @ &[dest_start, source_start, len] in &map[index..] {
						// println!("Line: {line:?}");
						let source_end = source_start + len;
						let dest_end = dest_start + len;

						if start >= source_end {
							continue;
						}
						if end <= source_start {
							next_ranges.push([start, end]);
							start = end;
							break;
						}
						if start < source_start {
							next_ranges.push([start, source_start]);
							start = source_start;
						}
						if end <= source_end {
							next_ranges.push([
								start + dest_start - source_start,
								end + dest_start - source_start,
							]);
							start = end;
							break;
						} else {
							next_ranges.push([start + dest_start - source_start, dest_end]);
							start = source_end;
						}
					}
					if start < end {
						next_ranges.push([start, end]);
					}
				}
			}

			ranges.clear();
			std::mem::swap(&mut ranges, &mut next_ranges);

			ranges.sort_unstable();
			next_ranges.push(ranges[0]);
			for &[start, end] in &ranges[1..] {
				if start == end {
					continue;
				}
				let last = &mut next_ranges.last_mut().unwrap()[1];
				if *last >= start {
					*last = end;
				} else {
					next_ranges.push([start, end]);
				}
			}

			ranges.clear();
			std::mem::swap(&mut ranges, &mut next_ranges);
		}

		ranges.into_iter().map(|[start, _]| start).min().unwrap()
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
