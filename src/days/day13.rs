#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]

use std::convert::identity;

use crate::helpers::*;

pub type A1 = impl Display + Debug + Clone;
pub type A2 = impl Display + Debug + Clone;

#[derive(Debug, Default, Clone)]
pub struct Solution {
	file: Vec<Vec<Vec<u8>>>,
}

impl Solver for Solution {
	type AnswerOne = A1;
	type AnswerTwo = A2;

	fn initialize(file: Vec<u8>, _: u8) -> Self {
		let file = file
			.delimiter("\n\n")
			.map(|grid| grid.grid(identity))
			.collect();
		Self { file }
	}

	fn part_one(&mut self, _: u8) -> Self::AnswerOne {
		let mut rows = 0;
		let mut cols = 0;

		'outer: for grid in &self.file {
			// Horizontal
			for y in 1..grid.len() {
				let (rows_before, rows_after) = grid.split_at(y);
				if rows_before
					.iter()
					.rev()
					.zip(rows_after)
					.all(|(row_b, row_a)| row_b == row_a)
				{
					rows += y;
					continue 'outer;
				}
			}

			// Vertical
			for x in 1..grid[0].len() {
				if grid.iter().all(|row| {
					let (items_before, items_after) = row.split_at(x);
					items_before
						.iter()
						.rev()
						.zip(items_after)
						.all(|(item_b, item_a)| item_b == item_a)
				}) {
					cols += x;
					continue 'outer;
				}
			}

			panic!("no symmetry found");
		}

		cols + 100 * rows
	}

	fn part_two(&mut self, _: u8) -> Self::AnswerTwo {
		let mut rows = 0;
		let mut cols = 0;

		'outer: for grid in &self.file {
			// Horizontal
			for y in 1..grid.len() {
				let (rows_before, rows_after) = grid.split_at(y);
				let mut iter = rows_before
					.iter()
					.rev()
					.zip(rows_after)
					.flat_map(|(row_b, row_a)| row_b.iter().zip(row_a))
					.filter(|&(item_b, item_a)| item_b != item_a);
				if let (Some(_), None) = (iter.next(), iter.next()) {
					rows += y;
					continue 'outer;
				}
			}

			// Vertical
			for x in 1..grid[0].len() {
				let mut iter = grid
					.iter()
					.flat_map(|row| {
						let (items_before, items_after) = row.split_at(x);
						items_before.iter().rev().zip(items_after)
					})
					.filter(|&(item_b, item_a)| item_b != item_a);
				if let (Some(_), None) = (iter.next(), iter.next()) {
					cols += x;
					continue 'outer;
				}
			}

			panic!("no symmetry found");
		}

		cols + 100 * rows
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
