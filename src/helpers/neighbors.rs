use std::array;
use std::iter::Flatten;

use crate::helpers::Grid;

/// Type of the regular 3x3 neighbors.
pub type Neighbors<'a, T> = NeighborsExtra<'a, 3, 3, T>;

/// Type of the exended YxX neighbors. Good for pattern matching.
pub type NeighborsExtra<'a, const Y: usize, const X: usize, T> = [[Option<&'a T>; X]; Y];

/// Collection type that can return neighbors of a `(y, x)` location.
pub trait GetNeighbors {
	/// The collection's item type.
	type Neighbor;

	/// The 3x3 neighbors centered on the selected coordinate.
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

	/// Iterator over the 3x3 neighbors of an element, including itself.
	fn neighbors_iter(&self, y: usize, x: usize) -> NeighborIter<Self::Neighbor, 3, 3> {
		self.neighbors(y, x).into_iter().flatten().flatten()
	}

	/// Iterator over the YxX neighbors of an element, including itself.
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

	/// Iterator over all the elements from [`GetNeighbors::neighbors_extra_offset`] with
	/// coordinates.
	fn neighbors_extra_offset_coords<const Y_LEN: usize, const X_LEN: usize>(
		&self,
		y: usize,
		x: usize,
		off_y: isize,
		off_x: isize,
	) -> NeighborCoords<Self::Neighbor, Y_LEN, X_LEN> {
		NeighborCoords::new(
			self.neighbors_extra_offset::<Y_LEN, X_LEN>(y, x, off_y, off_x),
			y.wrapping_add_signed(off_y),
			x.wrapping_add_signed(off_x),
		)
	}

	/// Iterator over all the elements from [`GetNeighbors::neighbors_extra`] with
	/// coordinates.
	fn neighbors_extra_coords<const Y_LEN: usize, const X_LEN: usize>(
		&self,
		y: usize,
		x: usize,
	) -> NeighborCoords<Self::Neighbor, Y_LEN, X_LEN> {
		NeighborCoords::new(
			self.neighbors_extra::<Y_LEN, X_LEN>(y, x),
			((Y_LEN as isize - 1) / -2) as usize,
			((X_LEN as isize - 1) / -2) as usize,
		)
	}

	/// Iterator over all the elements from [`GetNeighbors::neighbors`] with
	/// coordinates.
	fn neighbors_coords(&self, y: usize, x: usize) -> NeighborCoords<Self::Neighbor, 3, 3> {
		NeighborCoords::new(
			self.neighbors_extra::<3, 3>(y, x),
			y.wrapping_sub(1),
			x.wrapping_sub(1),
		)
	}
}

type NeighborIter<'a, T, const Y: usize, const X: usize> =
	Flatten<Flatten<std::array::IntoIter<[Option<&'a T>; X], Y>>>;

#[derive(Debug, Clone, Copy)]
pub struct NeighborCoords<'a, T, const Y_LEN: usize, const X_LEN: usize> {
	neighbors: [[Option<&'a T>; X_LEN]; Y_LEN],
	index: usize,
	offset_y: usize,
	offset_x: usize,
}

impl<'a, T, const Y_LEN: usize, const X_LEN: usize> Iterator
	for NeighborCoords<'a, T, Y_LEN, X_LEN>
{
	type Item = (&'a T, usize, usize);

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			let y = self.index / X_LEN;
			let x = self.index % X_LEN;
			if let Some(&opt) = self.neighbors.get(y).and_then(|row| row.get(x)) {
				self.index += 1;
				if let Some(item) = opt {
					break Some((
						item,
						y.wrapping_add(self.offset_y),
						x.wrapping_add(self.offset_x),
					));
				}
			} else {
				break None;
			}
		}
	}
}

impl<'a, T, const Y_LEN: usize, const X_LEN: usize> NeighborCoords<'a, T, Y_LEN, X_LEN> {
	fn new(neighbors: [[Option<&'a T>; X_LEN]; Y_LEN], offset_y: usize, offset_x: usize) -> Self {
		Self {
			neighbors,
			index: 0,
			offset_y,
			offset_x,
		}
	}

	pub fn reset(&mut self) {
		self.index = 0;
	}
}

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
			// let ny = (ny as isize + off_y) as usize;
			let ny = ny.wrapping_add_signed(off_y);
			array::from_fn(|dx| {
				let nx = x + dx;
				let nx = nx.wrapping_add_signed(off_x);
				self.get(ny).and_then(|row| row.get(nx))
			})
		})
	}
}
