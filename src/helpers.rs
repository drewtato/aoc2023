mod parse_bytes;
use std::hash::{BuildHasher, Hasher};
use std::io::stdin;
use std::num::Saturating;
use std::ops::{Add, AddAssign, Div, Mul};
use std::str::FromStr;

pub use crate::{AocError, Res, Solver};

pub use std::array::{from_fn as from_fn_array, try_from_fn};
pub use std::cmp::Reverse;
pub use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};
pub use std::fmt::{Debug, Display};
pub use std::iter::{
	empty as empty_iter, from_coroutine as gen_iter, from_fn as from_fn_iter, once as once_iter,
	once_with as once_with_iter, repeat as repeat_iter, repeat_with as repeat_with_iter,
	successors,
};
pub use std::mem::{replace, swap, take};
pub use std::num::Wrapping;
pub use std::str::from_utf8;

pub use itertools::Itertools;
pub use regex::bytes::Regex;

pub use num_integer::*;

pub use ahash::{AHashMap as HashMap, AHashSet as HashSet, HashMapExt, HashSetExt};

pub use parse_bytes::*;

mod neighbors;
pub use neighbors::*;

mod multi_parse;
pub use multi_parse::*;

mod input_data;
pub use input_data::*;

mod min_heap;
pub use min_heap::*;

mod display_bytes;
pub use display_bytes::*;

mod better_get;
pub use better_get::*;

mod more_itertools;
pub use more_itertools::*;

mod numeric;
pub use numeric::*;

/// Computes the triangular number.
///
/// # Example
/// ```
/// # use aoc2023::helpers::triangular_number;
/// for (n, ans) in [0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55].into_iter().enumerate() {
///     assert_eq!(triangular_number(n), ans);
/// }
/// ```
pub fn triangular_number<N>(n: N) -> N
where
	N: Add<Output = N> + Mul<Output = N> + Div<Output = N> + From<u8> + Copy,
{
	n * (n + 1u8.into()) / 2u8.into()
}

/// Reads a value from standard input.
///
/// Panics if reading from stdin fails. Returns an error if parsing the resulting string fails.
pub fn read_value<T>() -> Result<T, T::Err>
where
	T: FromStr,
{
	stdin().lines().next().unwrap().unwrap().trim().parse()
}

/// Waits for a newline from stdin.
pub fn pause() {
	stdin().lines().next().unwrap().unwrap();
}

/// [`std::iter::Sum`] but without the issue of needing to specify the output type.
pub trait SelfSum: Iterator + Sized
where
	Self::Item: AddAssign + Default + Sized,
{
	fn sum_self(self) -> Self::Item {
		self.fold(Default::default(), |mut left, right| {
			left += right;
			left
		})
	}
}

impl<I> SelfSum for I
where
	I: Iterator + Sized,
	Self::Item: AddAssign + Default + Sized,
{
}

/// Returns a curried function that compares a value to another value.
///
/// Example:
///
/// ```
/// # use aoc2023::helpers::is;
/// assert!(is("hello")("hello"));
/// ```
pub fn is<T>(byte: T) -> impl for<'b> Fn(&'b T) -> bool
where
	T: PartialEq,
{
	move |b| byte.eq(b)
}

/// Universal methods for wrapping values.
///
/// # Examples
///
/// ```
/// # use aoc2023::helpers::Wrap;
/// let i = 3;
/// assert_eq!(Some(3), i.wrap(Some));
/// ```
///
/// ```
/// # use aoc2023::helpers::Wrap;
/// let i = 3;
/// assert_eq!(Box::new(3), i.wrap_box());
/// ```
///
/// ```
/// # use aoc2023::helpers::Wrap;
/// let i = 3;
/// assert_eq!(&3, i.refer());
/// let mut i = 3;
/// assert_eq!(&mut 3, i.refmut());
/// ```
///
/// ```
/// # use aoc2023::helpers::Wrap;
/// let i = usize::MAX.wrap_wrapping() + 1.wrap_wrapping();
/// assert_eq!(0, i.0);
/// ```
///
/// ```
/// # use aoc2023::helpers::Wrap;
/// let mut i = 3.wrap_repeat();
/// assert_eq!(Some(3), i.nth(1_000_000));
/// ```
///
/// ```
/// # use aoc2023::helpers::Wrap;
/// let mut i = 3.wrap_once();
/// assert_eq!(Some(3), i.next());
/// assert_eq!(None, i.next());
/// ```
pub trait Wrap: Sized {
	fn wrap<F, T>(self, func: F) -> T
	where
		F: FnOnce(Self) -> T,
	{
		func(self)
	}

	fn refer(&self) -> &Self {
		self
	}

	fn refmut(&mut self) -> &mut Self {
		self
	}

	fn wrap_box(self) -> Box<Self> {
		Box::new(self)
	}

	fn wrap_wrapping(self) -> Wrapping<Self> {
		Wrapping(self)
	}

	fn wrap_saturating(self) -> Saturating<Self> {
		Saturating(self)
	}

	fn wrap_repeat(self) -> std::iter::Repeat<Self>
	where
		Self: Clone,
	{
		repeat_iter(self)
	}

	fn wrap_once(self) -> std::iter::Once<Self> {
		once_iter(self)
	}

	fn wrap_rev(self) -> Reverse<Self> {
		Reverse(self)
	}
}

impl<T> Wrap for T where T: Sized {}

/// Adds each element of two arrays together.
pub fn add<const N: usize, T>(a: [T; N], b: [T; N]) -> [T; N]
where
	T: Add<Output = T>,
{
	let mut a = a.into_iter();
	let mut b = b.into_iter();
	std::array::from_fn(|_| a.next().unwrap() + b.next().unwrap())
}

/// Gets an element of a 2D slice.
pub fn get_2d<T, V>(map: &[V], point: [isize; 2]) -> Option<&T>
where
	V: AsRef<[T]>,
{
	map.get(point[0] as usize)
		.and_then(|row| row.as_ref().get(point[1] as usize))
}

/// A hasher that does nothing.
#[derive(Debug, Clone, Copy, Default)]
pub struct IdentityHasher(u64);

impl Hasher for IdentityHasher {
	fn finish(&self) -> u64 {
		self.0
	}

	fn write(&mut self, _bytes: &[u8]) {
		panic!("This hasher only hashes u64");
	}

	fn write_u64(&mut self, i: u64) {
		self.0 = i;
	}
}

impl BuildHasher for IdentityHasher {
	type Hasher = Self;

	fn build_hasher(&self) -> Self::Hasher {
		*self
	}
}

pub use crate::dbg_small;
/// Modified [`std::dbg`] macro that doesn't use alternate form.
#[macro_export]
macro_rules! dbg_small {
	// NOTE: We cannot use `concat!` to make a static string as a format argument
    // of `eprintln!` because `file!` could contain a `{` or
	// `$val` expression could be a block (`{ .. }`), in which case the `eprintln!`
	// will be malformed.
	() => {
		eprintln!("[{}:{}]", file!(), line!())
	};

    ($val:expr $(,)?) => {
		// Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
			tmp => {
				eprintln!("[{}:{}] {} = {:?}",
				file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };

    ($($val:expr),+ $(,)?) => {
		($(dbg_small!($val)),+,)
    };
}

pub use crate::dbg_pause;
/// Modified [`std::dbg`] macro that doesn't use alternate form and waits for a newline from stdin.
#[macro_export]
macro_rules! dbg_pause {
	() => {{
		dbg_small!();
		pause()
	}};

    ($val:expr $(,)?) => {{
		let v = dbg_small!($val);
		pause();
		v
	}};

	($($val:expr),+ $(,)?) => {{
		let v = ($(dbg_small!($val)),+,);
		pause();
		v
    }};
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]

	fn neighbors() {
		#[rustfmt::skip]
		let v = vec![
			vec![1, 0, 0],
			vec![0, 0, 0],
			vec![1, 1, 0]
		];

		let arr = v.neighbors(0, 0);
		#[rustfmt::skip]
		assert_eq!(arr, [
			[None, None,     None    ],
			[None, Some(&1), Some(&0)],
			[None, Some(&0), Some(&0)],
		]);

		let arr = v.neighbors(1, 1);
		#[rustfmt::skip]
		assert_eq!(arr, [
			[Some(&1), Some(&0), Some(&0)],
			[Some(&0), Some(&0), Some(&0)],
			[Some(&1), Some(&1), Some(&0)],
		]);

		let arr = v.neighbors(2, 2);
		#[rustfmt::skip]
		assert_eq!(arr, [
			[Some(&0), Some(&0), None],
			[Some(&1), Some(&0), None],
			[None,     None,     None],
		]);
	}
}
