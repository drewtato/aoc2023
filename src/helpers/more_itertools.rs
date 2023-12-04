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
}

impl<I: Iterator> MoreItertools for I {}
