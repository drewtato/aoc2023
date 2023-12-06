pub trait MoreItertools: Iterator {
	fn array<const N: usize>(mut self) -> Option<[Self::Item; N]>
	where
		Self: Sized,
	{
		let a = std::array::try_from_fn(|_| self.next())?;
		if self.next().is_some() {
			None
		} else {
			Some(a)
		}
	}

	fn filter_empty(self) -> Empty<Self>
	where
		Self: Sized,
		Self::Item: Collection,
	{
		Empty::new(self)
	}
}

impl<I: Iterator> MoreItertools for I {}

pub trait Collection {
	fn is_empty(&self) -> bool {
		self.len() == 0
	}

	fn len(&self) -> usize;
}

impl<T: Collection + ?Sized> Collection for &T {
	fn len(&self) -> usize {
		(*self).len()
	}

	fn is_empty(&self) -> bool {
		(*self).is_empty()
	}
}

impl<T> Collection for Vec<T> {
	fn len(&self) -> usize {
		self.len()
	}

	fn is_empty(&self) -> bool {
		self.is_empty()
	}
}

impl<T> Collection for [T] {
	fn len(&self) -> usize {
		self.len()
	}

	fn is_empty(&self) -> bool {
		self.is_empty()
	}
}

impl Collection for String {
	fn len(&self) -> usize {
		self.len()
	}

	fn is_empty(&self) -> bool {
		self.is_empty()
	}
}

impl Collection for str {
	fn len(&self) -> usize {
		self.len()
	}

	fn is_empty(&self) -> bool {
		self.is_empty()
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Empty<I> {
	iter: I,
}

impl<I> Empty<I> {
	pub fn new(iter: I) -> Self {
		Self { iter }
	}
	pub fn into_inner(self) -> I {
		self.iter
	}
}

impl<I: Iterator> Iterator for Empty<I>
where
	I::Item: Collection,
{
	type Item = I::Item;

	fn next(&mut self) -> Option<Self::Item> {
		self.iter.find(|item| !item.is_empty())
	}
}
