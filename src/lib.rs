#![cfg_attr(feature = "unstable", feature(unsize, core_intrinsics))]

#[macro_use]
extern crate coalesce;
extern crate unreachable;
extern crate nodrop;

mod util;
mod array;
mod vector;
mod array_vec;
mod small_dst;
mod small_vec;

pub use util::PointerExt;
pub use array::{Array, ArrayIndex, Addressable};
pub use vector::Vector;
pub use small_vec::{SmallVec, Spilled};
pub use array_vec::ArrayVec;
pub use small_dst::SmallDST;
