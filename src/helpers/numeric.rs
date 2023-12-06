pub trait NumericLength {
	fn length(self, base: Self) -> usize;
}

macro_rules! impl_numeric_length {
	($($t:ty,)*$(,)?) => {
		$(
			impl NumericLength for $t {
				fn length(mut self, base: Self) -> usize {
					if self == 0 {
						return 1;
					}

					let mut length = 0;
					while self > 0 {
						self /= base;
						length += 1;
					}
					length
				}
			}
		)*
	};
}

impl_numeric_length! {
	u8, u16, u32, u64, u128,
	i8, i16, i32, i64, i128,
}
