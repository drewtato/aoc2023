#![feature(type_alias_impl_trait)]
#![feature(array_windows)]
#![feature(array_chunks)]
#![feature(array_try_from_fn)]
#![feature(byte_slice_trim_ascii)]
#![feature(slice_take)]
#![feature(iter_array_chunks)]
#![feature(iter_collect_into)]
#![feature(get_many_mut)]
#![feature(split_as_slice)]
#![feature(coroutines)]
#![feature(iter_from_coroutine)]
#![feature(slice_split_once)]
#![feature(impl_trait_in_assoc_type)]
#![feature(slice_group_by)]
#![feature(iter_advance_by)]
#![feature(array_try_map)]
#![feature(slice_first_last_chunk)]

pub const YEAR: u32 = 2023;
pub type Res<T> = Result<T, AocError>;
pub mod solution;
pub use solution::Solver;
mod error;
pub use error::AocError;
pub mod helpers;
pub mod runner;

pub mod days;
