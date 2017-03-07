#![cfg_attr(all(feature = "bench", test), feature(test))]

#![deny(unused_imports)]
#![deny(missing_docs)]

//! An addressable pairing heap implementation for Rust.
//! 
//! Addressable heaps return handles to stored elements that make it possible
//! to query and edit them. For example this allows for the `decrease_key(h: Handle)` method
//! that decreases the key (priority) of the element that is associated with the
//! given handle.
//! 
//! This implementation stores elements within a `Stash` that allocates elements
//! densely within an array.
//!
//! It is possible to use custom types as the underlying `Key` type by implementing
//! the `Key` trait.

#[cfg(all(feature = "bench", test))]
extern crate test;
extern crate rand;

extern crate stash;
extern crate itertools;
extern crate unreachable;

pub mod ptr_heap;
pub mod vec_heap;
