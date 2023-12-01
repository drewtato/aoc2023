use std::array;
use std::iter::Flatten;

use crate::helpers::Grid;

/// Type of the regular 3x3 neighbors.
pub type Neighbors<'a, T> = NeighborsExtra<'a, 3, 3, T>;

/// Type of the exended YxX neighbors. Good for pattern matching.
pub type NeighborsExtra<'a, const Y: usize, const X: usize, T> = [[Option<&'a T>; X]; Y];

/// Collection type that can return neighbors of a `(y, x)` location.
pub trait GetNeighbors {
	/// The collection's item type
	type Neighbor;

	/// The 3x3 neighbors centered on the selected coordinate
	fn neighbors(&self, y: usize, x: usize) -> Neighbors<Self::Neighbor> {
		self.neighbors_extra(y, x)
	}

	/// The YxX neighbors centered on the selected coordinate Y and X should usually be odd, which
	/// ensures `(y, x)` is in the center.
	fn neighbors_extra<const Y_LEN: usize, const X_LEN: usize>(
		&self,
		y: usize,
		x: usize,
	) -> NeighborsExtra<Y_LEN, X_LEN, Self::Neighbor> {
		self.neighbors_extra_offset(y, x, (Y_LEN as isize - 1) / -2, (X_LEN as isize - 1) / -2)
	}

	/// The basis method of [`GetNeighbors`]. Retrieves a 2-dimensional array where the `(-off_y,
	/// -off_x)` element is `(y, x)`.
	fn neighbors_extra_offset<const Y_LEN: usize, const X_LEN: usize>(
		&self,
		y: usize,
		x: usize,
		off_y: isize,
		off_x: isize,
	) -> NeighborsExtra<Y_LEN, X_LEN, Self::Neighbor>;

	/// Iterator over the 3x3 neighbors of an element, including itself
	fn neighbors_iter(&self, y: usize, x: usize) -> NeighborIter<Self::Neighbor, 3, 3> {
		self.neighbors(y, x).into_iter().flatten().flatten()
	}

	/// Iterator over the YxX neighbors of an element, including itself
	fn neighbors_extra_iter<const Y_LEN: usize, const X_LEN: usize>(
		&self,
		y: usize,
		x: usize,
	) -> NeighborIter<Self::Neighbor, Y_LEN, X_LEN> {
		self.neighbors_extra::<Y_LEN, X_LEN>(y, x)
			.into_iter()
			.flatten()
			.flatten()
	}

	/// Iterator over all the elements from [`GetNeighbors::neighbors_extra_offset`].
	fn neighbors_extra_offset_iter<const Y_LEN: usize, const X_LEN: usize>(
		&self,
		y: usize,
		x: usize,
		off_y: isize,
		off_x: isize,
	) -> NeighborIter<Self::Neighbor, Y_LEN, X_LEN> {
		self.neighbors_extra_offset::<Y_LEN, X_LEN>(y, x, off_y, off_x)
			.into_iter()
			.flatten()
			.flatten()
	}
}

type NeighborIter<'a, T, const Y: usize, const X: usize> =
	Flatten<Flatten<std::array::IntoIter<[Option<&'a T>; X], Y>>>;

impl<T> GetNeighbors for Grid<T> {
	type Neighbor = T;

	fn neighbors_extra_offset<const Y_LEN: usize, const X_LEN: usize>(
		&self,
		y: usize,
		x: usize,
		off_y: isize,
		off_x: isize,
	) -> NeighborsExtra<Y_LEN, X_LEN, Self::Neighbor> {
		array::from_fn(|dy| {
			let ny = y + dy;
			let ny = (ny as isize + off_y) as usize;
			array::from_fn(|dx| {
				let nx = x + dx;
				let nx = (nx as isize + off_x) as usize;
				self.get(ny).and_then(|row| row.get(nx))
			})
		})
	}
}
