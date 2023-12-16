use std::collections::VecDeque;
use std::ops::{Index, IndexMut};

use super::Grid;

/// Adds methods to index into collections with any integer
pub trait BetterGet<I, T> {
	/// Returns a reference to an element if the index is in range
	fn bget(&self, index: I) -> Option<&T>;
	/// Returns a mutable reference to an element if the index is in range
	fn bget_mut(&mut self, index: I) -> Option<&mut T>;
}

/// A wrapper to enable indexing with any integer
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct BetterGetter<C: ?Sized>(pub C);

impl<C: ?Sized> BetterGetter<C> {
	pub fn new(c: C) -> Self
	where
		C: Sized,
	{
		Self(c)
	}

	pub fn into_inner(self) -> C
	where
		C: Sized,
	{
		self.0
	}

	pub fn get<I, T>(&self, index: I) -> Option<&T>
	where
		C: BetterGet<I, T>,
	{
		self.0.bget(index)
	}

	pub fn get_mut<I, T>(&mut self, index: I) -> Option<&mut T>
	where
		C: BetterGet<I, T>,
	{
		self.0.bget_mut(index)
	}
}

macro_rules! better_get {
	($($container:ty, $generic:ident;)*) => {
		$(

			impl<$generic, I> BetterGet<I, T> for $container
			where
				I: TryInto<usize>,
			{
				fn bget(&self, index: I) -> Option<&T> {
					self.get(index.try_into().ok()?)
				}

				fn bget_mut(&mut self, index: I) -> Option<&mut T> {
					self.get_mut(index.try_into().ok()?)
				}
			}

			impl<$generic, I> Index<I> for BetterGetter<$container>
			where
				I: TryInto<usize>,
			{
				type Output = $generic;

				fn index(&self, index: I) -> &Self::Output {
					self.get(index).unwrap()
				}
			}

			impl<$generic, I> IndexMut<I> for BetterGetter<$container>
			where
				I: TryInto<usize>,
			{
				fn index_mut(&mut self, index: I) -> &mut Self::Output {
					self.get_mut(index).unwrap()
				}
			}
		)*
	};
}

better_get! {
	[T], T;
	Vec<T>, T;
	VecDeque<T>, T;
}

impl<const N: usize, T, I> BetterGet<I, T> for [T; N]
where
	I: TryInto<usize>,
{
	fn bget(&self, index: I) -> Option<&T> {
		self.get(index.try_into().ok()?)
	}

	fn bget_mut(&mut self, index: I) -> Option<&mut T> {
		self.get_mut(index.try_into().ok()?)
	}
}

impl<const N: usize, T, I> Index<I> for BetterGetter<[T; N]>
where
	I: TryInto<usize>,
{
	type Output = T;

	fn index(&self, index: I) -> &Self::Output {
		self.get(index).unwrap()
	}
}

impl<const N: usize, T, I> IndexMut<I> for BetterGetter<[T; N]>
where
	I: TryInto<usize>,
{
	fn index_mut(&mut self, index: I) -> &mut Self::Output {
		self.get_mut(index).unwrap()
	}
}

pub trait GridGet<T, I> {
	fn grid_get(&self, coordinates: [I; 2]) -> Option<&T>;

	fn grid_get_mut(&mut self, coordinates: [I; 2]) -> Option<&mut T>;
}

impl<T, I> GridGet<T, I> for Grid<T>
where
	I: TryInto<usize>,
{
	fn grid_get(&self, coordinates: [I; 2]) -> Option<&T> {
		let [y, x] = coordinates;
		self.bget(y).and_then(|row| row.bget(x))
	}

	fn grid_get_mut(&mut self, coordinates: [I; 2]) -> Option<&mut T> {
		let [y, x] = coordinates;
		self.bget_mut(y).and_then(|row| row.bget_mut(x))
	}
}
