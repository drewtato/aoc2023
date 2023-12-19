use std::iter::Filter;
use std::slice::Split;

/// Type of the grid returned by [`InputData::grid`].
pub type Grid<G> = Vec<Vec<G>>;

/// A value that can be used as input. This is usually either [`Vec<u8>`] or [`&[u8]`](slice).
pub trait InputData<'a> {
	/// Return type of [`lines`](InputData::lines).
	type Lines: Iterator<Item = &'a [u8]>;

	/// Return type of [`words`](InputData::words).
	type Words: Iterator<Item = &'a [u8]>;

	/// Return type of [`delimiter`](InputData::delimiter).
	type Delimiter<D>: Iterator<Item = &'a [u8]>
	where
		Self: 'a,
		D: Delimiter;

	/// Returns an iterator over slices between `\n` bytes. Does not include the `\n` byte.
	fn lines(&'a self) -> Self::Lines;

	/// Returns an iterator over slices between whitespace. Does not include whitespace.
	fn words(&'a self) -> Self::Words;

	/// Returns an iterator over slices divided by a delimiter. Does not include the delimiter.
	fn delimiter<D>(&'a self, delimiter: D) -> Self::Delimiter<D>
	where
		D: Delimiter;

	/// Returns a 2D grid of the input after transforming with the provided closure.
	///
	/// This will be a fully rectangular grid, so all the lengths of the inner [`Vec`]s will be the
	/// same.
	fn grid<G, F>(&self, f: F) -> Grid<G>
	where
		F: FnMut(u8) -> G,
		G: Default;

	/// Runs a closure on each line of input.
	///
	/// The closure is given a slice that starts at the beginning of a line and goes all the way to
	/// the end of the input. The closure returns how many characters to skip forward so that this
	/// function doesn't have to recheck those for a newline byte. It also allows it to consume
	/// multiple lines if necessary. If the closure returns `Ok(skip)`, `consume_lines` will skip
	/// forward that much, and then skip until a newline is found. If the closure returns
	/// `Err(skip)`, `consume_lines` will skip forward exactly that much and no further. This is
	/// useful if the newline has been found inside the closure, or if the closure doesn't want to
	/// skip an entire line.
	fn consume_lines<F>(&self, f: F)
	where
		F: FnMut(&[u8]) -> Result<usize, usize>;
}

impl<'a> InputData<'a> for [u8] {
	type Lines = Filter<Split<'a, u8, fn(&u8) -> bool>, fn(&&[u8]) -> bool>;
	type Words = Filter<Split<'a, u8, fn(&u8) -> bool>, fn(&&[u8]) -> bool>;
	type Delimiter<D> = DelimiterIter<'a, D> where D: Delimiter;

	fn lines(&'a self) -> Self::Lines {
		self.split((|&b: &u8| b == b'\n') as _)
			.filter(slice_is_not_empty)
	}

	fn words(&'a self) -> Self::Words {
		self.split((|&b: &u8| b.is_ascii_whitespace()) as _)
			.filter(slice_is_not_empty)
	}

	fn delimiter<D>(&'a self, delimiter: D) -> Self::Delimiter<D>
	where
		D: Delimiter,
	{
		DelimiterIter::new(self, delimiter)
	}

	fn grid<G, F>(&self, mut f: F) -> Grid<G>
	where
		F: FnMut(u8) -> G,
		G: Default,
	{
		let mut grid: Grid<G> = self
			.lines()
			.map(|line| line.iter().map(|&byte| f(byte)).collect())
			.collect();

		let max = grid.iter().map(|v| v.len()).max().unwrap_or_default();
		for row in &mut grid {
			row.resize_with(max, Default::default);
		}

		grid
	}

	fn consume_lines<F>(&self, mut f: F)
	where
		F: FnMut(&[u8]) -> Result<usize, usize>,
	{
		let mut current = self;
		loop {
			let skip = f(current);
			match advance_to_newline(current, skip) {
				Some(new_current) => current = new_current,
				None => break,
			}
		}
	}
}

fn advance_to_newline(mut current: &[u8], skip: Result<usize, usize>) -> Option<&[u8]> {
	match skip {
		Ok(s) => current = current.get(s..)?,
		Err(s) => return current.get(s..),
	}

	loop {
		let &first = current.take_first()?;
		if first == b'\n' {
			break;
		}
	}

	Some(current)
}

/// Necessary for higher-ranked lifetime error when using closure instead
fn slice_is_not_empty(s: &&[u8]) -> bool {
	!s.is_empty()
}

/// A type that can act as a delimiter for a [`[u8]`](std::slice) slice.
pub trait Delimiter {
	/// Check if the slice starts with this delimiter.
	///
	/// If the slice starts with the delimiter, returns the length of the delimiter. Otherwise,
	/// returns `None`.
	fn starts_with_delimiter(&self, slice: &[u8]) -> Option<usize>;
}

impl Delimiter for u8 {
	fn starts_with_delimiter(&self, slice: &[u8]) -> Option<usize> {
		if slice.first() == Some(self) {
			Some(1)
		} else {
			None
		}
	}
}

impl Delimiter for &[u8] {
	fn starts_with_delimiter(&self, slice: &[u8]) -> Option<usize> {
		if slice.starts_with(self) {
			Some(self.len())
		} else {
			None
		}
	}
}

impl<const N: usize> Delimiter for [u8; N] {
	fn starts_with_delimiter(&self, slice: &[u8]) -> Option<usize> {
		if slice.starts_with(self) {
			Some(self.len())
		} else {
			None
		}
	}
}

impl<const N: usize> Delimiter for &[u8; N] {
	fn starts_with_delimiter(&self, slice: &[u8]) -> Option<usize> {
		if slice.starts_with(self.as_slice()) {
			Some(self.len())
		} else {
			None
		}
	}
}

impl Delimiter for char {
	fn starts_with_delimiter(&self, slice: &[u8]) -> Option<usize> {
		let mut c_slice = [0; 4];
		let c_slice = self.encode_utf8(&mut c_slice);
		if slice.starts_with(c_slice.as_bytes()) {
			Some(c_slice.len())
		} else {
			None
		}
	}
}

impl Delimiter for &str {
	fn starts_with_delimiter(&self, slice: &[u8]) -> Option<usize> {
		if slice.starts_with(self.as_bytes()) {
			Some(self.len())
		} else {
			None
		}
	}
}

impl Delimiter for String {
	fn starts_with_delimiter(&self, slice: &[u8]) -> Option<usize> {
		if slice.starts_with(self.as_bytes()) {
			Some(self.len())
		} else {
			None
		}
	}
}

macro_rules! impl_delimiter_slice {
	($($t:ty),* $(,)?) => {
		$(
			impl Delimiter for &[$t] {
				fn starts_with_delimiter(&self, slice: &[u8]) -> Option<usize> {
					for c in *self {
						if let Some(l) = c.starts_with_delimiter(slice) {
							return Some(l);
						}
					}
					None
				}
			}

			impl<const N: usize> Delimiter for [$t; N] {
				fn starts_with_delimiter(&self, slice: &[u8]) -> Option<usize> {
					for c in self {
						if let Some(l) = c.starts_with_delimiter(slice) {
							return Some(l);
						}
					}
					None
				}
			}

			impl<const N: usize> Delimiter for &[$t; N] {
				fn starts_with_delimiter(&self, slice: &[u8]) -> Option<usize> {
					for c in *self {
						if let Some(l) = c.starts_with_delimiter(slice) {
							return Some(l);
						}
					}
					None
				}
			}
		)*
	};
}

impl_delimiter_slice! {
	char,
	&str,
	&[u8],
	String,
}

impl<const M: usize> Delimiter for &[[u8; M]] {
	fn starts_with_delimiter(&self, slice: &[u8]) -> Option<usize> {
		for c in *self {
			if let Some(l) = c.starts_with_delimiter(slice) {
				return Some(l);
			}
		}
		None
	}
}

impl<const N: usize, const M: usize> Delimiter for [[u8; M]; N] {
	fn starts_with_delimiter(&self, slice: &[u8]) -> Option<usize> {
		for c in self {
			if let Some(l) = c.starts_with_delimiter(slice) {
				return Some(l);
			}
		}
		None
	}
}

impl<const N: usize, const M: usize> Delimiter for &[[u8; M]; N] {
	fn starts_with_delimiter(&self, slice: &[u8]) -> Option<usize> {
		for c in *self {
			if let Some(l) = c.starts_with_delimiter(slice) {
				return Some(l);
			}
		}
		None
	}
}

#[derive(Debug, Clone, Copy)]
pub struct DelimiterIter<'a, D> {
	slice: Option<&'a [u8]>,
	delimiter: D,
}

impl<'a, D> Iterator for DelimiterIter<'a, D>
where
	D: Delimiter,
{
	type Item = &'a [u8];

	fn next(&mut self) -> Option<Self::Item> {
		let Self { slice, delimiter } = self;
		let Some(slice) = slice else {
			return None;
		};
		let original = *slice;

		loop {
			if let Some(len) = delimiter.starts_with_delimiter(slice) {
				let item_len = original.len() - slice.len();
				let item = &original[..item_len];
				let take = slice.len().min(len);
				*slice = &slice[take..];
				break Some(item);
			}

			if slice.take_first().is_none() {
				self.slice = None;
				break Some(original);
			}
		}
	}
}

impl<'a, D> DoubleEndedIterator for DelimiterIter<'a, D>
where
	D: Delimiter,
{
	fn next_back(&mut self) -> Option<Self::Item> {
		let Self { slice, delimiter } = self;
		let Some(slice) = slice else {
			return None;
		};
		let mut index = slice.len();

		loop {
			index = index.wrapping_sub(1);
			let Some(end_slice) = slice.get(index..) else {
				let item = *slice;
				self.slice = None;
				break Some(item);
			};

			// println!("{}", DisplaySlice(end_slice));

			if let Some(len) = delimiter.starts_with_delimiter(end_slice) {
				slice.take(index..).unwrap();
				break Some(&end_slice[len..]);
			}
		}
	}
}

impl<'a, D> DelimiterIter<'a, D> {
	fn new(slice: &'a [u8], delimiter: D) -> Self {
		Self {
			slice: Some(slice),
			delimiter,
		}
	}

	pub fn slice(&self) -> &'a [u8] {
		self.slice.unwrap_or_default()
	}
}

#[cfg(test)]
mod tests {
	use itertools::Itertools;

	use super::*;

	#[test]

	fn consume_lines() {
		let data = b"hello\n123   \n123\nyes\n";
		let mut res = Vec::new();
		data.consume_lines(|line| {
			let line_stuff = line
				.iter()
				.copied()
				.take_while(|b| !b.is_ascii_whitespace())
				.collect_vec();
			let len = line_stuff.len();
			res.push(line_stuff);
			if line.get(len) == Some(&b'\n') {
				Err(len + 1)
			} else {
				Ok(len)
			}
		});
		assert_eq!(
			res.as_slice(),
			&[
				b"hello".to_vec(),
				b"123".to_vec(),
				b"123".to_vec(),
				b"yes".to_vec(),
				b"".to_vec(),
			]
		);
	}

	#[test]
	fn delimiter_u8() {
		let data = b"1,2,3";
		let delimited = data.delimiter(b',').collect_vec();
		assert_eq!(delimited, [b"1", b"2", b"3"]);

		let data = b"1,2,3,";
		let delimited = data.delimiter(b',').collect_vec();
		let target: &[&[u8]] = &[b"1", b"2", b"3", b""];
		assert_eq!(delimited, target);
	}

	#[test]
	fn delimiter_char() {
		let data = b"1,2,3";
		let delimited = data.delimiter(',').collect_vec();
		assert_eq!(delimited, [b"1", b"2", b"3"]);

		let data = b"1,2,3,";
		let delimited = data.delimiter(',').collect_vec();
		let target: &[&[u8]] = &[b"1", b"2", b"3", b""];
		assert_eq!(delimited, target);
	}

	#[test]
	fn delimiter_slice() {
		let data = b"1,2,3";
		let delimited = data.delimiter(b",").collect_vec();
		assert_eq!(delimited, [b"1", b"2", b"3"]);

		let data = b"1,2,3,";
		let delimited = data.delimiter(b",").collect_vec();
		let target: &[&[u8]] = &[b"1", b"2", b"3", b""];
		assert_eq!(delimited, target);
	}

	#[test]
	fn delimiter_str() {
		let data = b"1,2,3";
		let delimited = data.delimiter(b",").collect_vec();
		assert_eq!(delimited, [b"1", b"2", b"3"]);

		let data = b"1,2,3,";
		let delimited = data.delimiter(b",").collect_vec();
		let target: &[&[u8]] = &[b"1", b"2", b"3", b""];
		assert_eq!(delimited, target);
	}

	#[test]
	fn delimiter_multiple_bytes() {
		let data = b"abc,,123,,,456";
		let delimited = data.delimiter([",,,", ",,"]).collect_vec();
		assert_eq!(delimited, [b"abc", b"123", b"456"]);
	}

	#[test]
	fn delimiter_multiple_bytes_greedy() {
		let data = b"abc,,123,,,456";
		let delimited = data.delimiter([",,", ",,,"]).collect_vec();
		let target: &[&[u8]] = &[b"abc", b"123", b",456"];
		assert_eq!(delimited, target);
	}
}
