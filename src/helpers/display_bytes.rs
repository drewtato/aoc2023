use std::fmt::{Debug, Display};

/// Wrapper to allow displaying a slice as UTF-8
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplaySlice<T>(pub T);

impl<T> Display for DisplaySlice<T>
where
	T: AsRef<[u8]>,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = std::str::from_utf8(self.0.as_ref()).unwrap();
		write!(f, "{s}")
	}
}

impl<T> Debug for DisplaySlice<T>
where
	T: AsRef<[u8]>,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = std::str::from_utf8(self.0.as_ref()).unwrap();
		write!(f, "{s:?}")
	}
}

impl<T> DisplaySlice<T> {
	pub fn new(s: T) -> Self {
		Self(s)
	}
}

/// Extension trait to wrap a slice in [`DisplaySlice`]
pub trait ToDisplaySlice: Sized {
	fn to_display_slice(self) -> DisplaySlice<Self>;
}

impl ToDisplaySlice for &[u8] {
	fn to_display_slice(self) -> DisplaySlice<Self> {
		DisplaySlice(self)
	}
}

impl ToDisplaySlice for Vec<u8> {
	fn to_display_slice(self) -> DisplaySlice<Self> {
		DisplaySlice(self)
	}
}

impl<const N: usize> ToDisplaySlice for [u8; N] {
	fn to_display_slice(self) -> DisplaySlice<Self> {
		DisplaySlice(self)
	}
}

/// Wrapper to allow displaying `u8` as an ASCII character
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DisplayByte(pub u8);

impl Display for DisplayByte {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0 as char)
	}
}

impl Debug for DisplayByte {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.0 as char)
	}
}

impl DisplayByte {
	pub fn new(b: u8) -> Self {
		Self(b)
	}
}

/// Extension trait to wrap a byte in [`DisplayByte`]
pub trait ToDisplayByte {
	fn to_display_byte(self) -> DisplayByte;
}

impl ToDisplayByte for u8 {
	fn to_display_byte(self) -> DisplayByte {
		DisplayByte(self)
	}
}
