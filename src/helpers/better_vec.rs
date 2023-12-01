use std::borrow::{Borrow, BorrowMut};
use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct BetterVec<T>(pub Vec<T>);

impl<T> BetterVec<T> {
	pub fn get<I>(&self, index: I) -> Option<&T>
	where
		I: TryInto<usize>,
	{
		self.as_better_slice().get(index)
	}
	pub fn get_mut<I>(&mut self, index: I) -> Option<&mut T>
	where
		I: TryInto<usize>,
	{
		self.as_mut_better_slice().get_mut(index)
	}

	pub fn as_better_slice(&self) -> &BetterSlice<T> {
		self.as_ref()
	}

	pub fn as_mut_better_slice(&mut self) -> &mut BetterSlice<T> {
		self.as_mut()
	}
}

impl<T> Default for BetterVec<T> {
	fn default() -> Self {
		Self(Default::default())
	}
}

impl<T> From<Vec<T>> for BetterVec<T> {
	fn from(value: Vec<T>) -> Self {
		Self(value)
	}
}

impl<T> From<BetterVec<T>> for Vec<T> {
	fn from(value: BetterVec<T>) -> Self {
		value.0
	}
}

impl<T, I> Index<I> for BetterVec<T>
where
	I: TryInto<usize>,
{
	type Output = T;

	fn index(&self, index: I) -> &Self::Output {
		&AsRef::<BetterSlice<T>>::as_ref(self)[index]
	}
}

impl<T, I> IndexMut<I> for BetterVec<T>
where
	I: TryInto<usize>,
{
	fn index_mut(&mut self, index: I) -> &mut Self::Output {
		&mut AsMut::<BetterSlice<T>>::as_mut(self)[index]
	}
}

impl<T> Deref for BetterVec<T> {
	type Target = Vec<T>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<T> DerefMut for BetterVec<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl<T> AsRef<BetterSlice<T>> for BetterVec<T> {
	fn as_ref(&self) -> &BetterSlice<T> {
		self.as_slice().into()
	}
}

impl<T> AsMut<BetterSlice<T>> for BetterVec<T> {
	fn as_mut(&mut self) -> &mut BetterSlice<T> {
		self.as_mut_slice().into()
	}
}

impl<T> AsRef<Vec<T>> for BetterVec<T> {
	fn as_ref(&self) -> &Vec<T> {
		self
	}
}

impl<T> AsMut<Vec<T>> for BetterVec<T> {
	fn as_mut(&mut self) -> &mut Vec<T> {
		self
	}
}

impl<T> AsRef<[T]> for BetterVec<T> {
	fn as_ref(&self) -> &[T] {
		self
	}
}

impl<T> AsMut<[T]> for BetterVec<T> {
	fn as_mut(&mut self) -> &mut [T] {
		self
	}
}

impl<T> FromIterator<T> for BetterVec<T> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		Self(Vec::from_iter(iter))
	}
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct BetterSlice<T>(pub [T]);

impl<T> BetterSlice<T> {
	pub fn get<I>(&self, index: I) -> Option<&T>
	where
		I: TryInto<usize>,
	{
		let u = index.try_into().ok()?;
		self.0.get(u)
	}

	pub fn get_mut<I>(&mut self, index: I) -> Option<&mut T>
	where
		I: TryInto<usize>,
	{
		let u = index.try_into().ok()?;
		self.0.get_mut(u)
	}

	pub fn as_slice(&self) -> &[T] {
		self
	}

	pub fn as_mut_slice(&mut self) -> &mut [T] {
		self
	}
}

impl<'a, T> From<&'a [T]> for &'a BetterSlice<T> {
	fn from(value: &'a [T]) -> Self {
		unsafe { &*(value as *const [T] as *const BetterSlice<T>) }
	}
}

impl<'a, T> From<&'a BetterSlice<T>> for &'a [T] {
	fn from(value: &'a BetterSlice<T>) -> Self {
		unsafe { &*(value as *const BetterSlice<T> as *const [T]) }
	}
}

impl<'a, T> From<&'a mut [T]> for &'a mut BetterSlice<T> {
	fn from(value: &'a mut [T]) -> Self {
		unsafe { &mut *(value as *mut [T] as *mut BetterSlice<T>) }
	}
}

impl<'a, T> From<&'a mut BetterSlice<T>> for &'a mut [T] {
	fn from(value: &'a mut BetterSlice<T>) -> Self {
		unsafe { &mut *(value as *mut BetterSlice<T> as *mut [T]) }
	}
}

impl<'a, T> Default for &'a BetterSlice<T> {
	fn default() -> Self {
		unsafe { &*(&[] as *const [T] as *const BetterSlice<T>) }
	}
}

impl<'a, T> Default for &'a mut BetterSlice<T> {
	fn default() -> Self {
		unsafe { &mut *(&mut [] as *mut [T] as *mut BetterSlice<T>) }
	}
}

impl<T, I> Index<I> for BetterSlice<T>
where
	I: TryInto<usize>,
{
	type Output = T;

	fn index(&self, index: I) -> &Self::Output {
		self.get(index).expect("index was out of bounds")
	}
}

impl<T, I> IndexMut<I> for BetterSlice<T>
where
	I: TryInto<usize>,
{
	fn index_mut(&mut self, index: I) -> &mut Self::Output {
		self.get_mut(index).expect("index was out of bounds")
	}
}

impl<T> Deref for BetterSlice<T> {
	type Target = [T];

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<T> DerefMut for BetterSlice<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

impl<T> AsRef<[T]> for BetterSlice<T> {
	fn as_ref(&self) -> &[T] {
		self
	}
}

impl<T> AsMut<[T]> for BetterSlice<T> {
	fn as_mut(&mut self) -> &mut [T] {
		self
	}
}

impl<T> Borrow<BetterSlice<T>> for BetterVec<T> {
	fn borrow(&self) -> &BetterSlice<T> {
		self.as_ref()
	}
}

impl<T> BorrowMut<BetterSlice<T>> for BetterVec<T> {
	fn borrow_mut(&mut self) -> &mut BetterSlice<T> {
		self.as_mut()
	}
}

impl<T: PartialEq> PartialEq<BetterSlice<T>> for BetterVec<T> {
	fn eq(&self, other: &BetterSlice<T>) -> bool {
		self.0 == other.0
	}
}

impl<T: PartialOrd> PartialOrd<BetterSlice<T>> for BetterVec<T> {
	fn partial_cmp(&self, other: &BetterSlice<T>) -> Option<std::cmp::Ordering> {
		self.as_slice().partial_cmp(other.as_slice())
	}
}

impl<T: PartialEq> PartialEq<BetterVec<T>> for BetterSlice<T> {
	fn eq(&self, other: &BetterVec<T>) -> bool {
		self.0 == other.0
	}
}

impl<T: PartialOrd> PartialOrd<BetterVec<T>> for BetterSlice<T> {
	fn partial_cmp(&self, other: &BetterVec<T>) -> Option<std::cmp::Ordering> {
		self.as_slice().partial_cmp(other.as_slice())
	}
}
