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

extern crate stash;
extern crate unreachable;
extern crate itertools;

use itertools::Itertools;

/// A handle to access stored elements within an addressable pairing heap.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Handle(usize);

impl Handle {
	fn undef() -> Self {
		Handle(usize::max_value())
	}

	fn is_undef(self) -> bool {
		self == Handle::undef()
	}

	fn to_usize(self) -> usize { self.0 }
}

/// Represents a trait for keys within an addressable pairing heap.
/// 
/// A user can use custom type for the key type by implementing this trait.
/// 
/// This trait is implicitely implemented already for all types that 
/// are `Copy`, `PartialOrd` and `Ord`.
pub trait Key: Copy + PartialOrd + Ord {}
impl<T> Key for T where T: Copy + PartialOrd + Ord {}

/// An entry within an addressable pairing heap.
#[derive(Debug, PartialEq, Eq)]
struct Entry<T, K> where K: Key {
	key : K,
	elem: T
}

impl<T, K> Entry<T, K>
	where K: Key
{
	fn new(key: K, elem: T) -> Self {
		Entry{
			key : key,
			elem: elem
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Position{
	/// root node at index
	Root(usize),

	/// child of parent with index
	Child(Handle, usize)
}

impl Position {
	fn child(parent: Handle, index: usize) -> Self {
		Position::Child(parent, index)
	}

	fn root(index: usize) -> Self {
		Position::Root(index)
	}

	fn is_root(self) -> bool {
		match self {
			Position::Root(_) => true,
			_                 => false
		}
	}

	fn is_child(self) -> bool {
		match self {
			Position::Child(..) => true,
			_                   => false
		}
	}
}

#[derive(Debug, PartialEq, Eq)]
struct Node<T, K>
	where K: Key
{
	pos     : Position,
	entry   : Entry<T, K>,
	children: Vec<Handle>
}

impl<T, K> Node<T, K>
	where K: Key
{
	fn new_root(at: usize, entry: Entry<T, K>) -> Self {
		Node{
			entry   : entry,
			pos     : Position::root(at),
			children: Vec::new()
		}
	}
}

/// Errors that can be caused while using `PairingHeap`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
	/// Caused when using `decrease_key` method with a `new_key` that is greater than the old one.
	DecreaseKeyOutOfOrder
}

/// Generic `Result` type for `PairingHeap` methods.
pub type Result<T> = ::std::result::Result<T, Error>;

use stash::*;

/// Type alias for `PairingHeap` that has `i64` as default `Key` type.
pub type DefaultPairingHeap<T> = PairingHeap<T, i64>;

/// An addressable pairing heap implementation.
/// 
/// Stores elements with an associated key.
/// The key can be thought of as the priority of the element that is associated to it.
/// 
/// Supports usages like `take_min` that takes the element with the minimum key out of this storage.
/// 
/// Inserting elements into this data structure provides the caller with handles
/// that makes accessing the elements possible - this is called "addressable".
/// Handles are always local to the associated pairing heap instance and thus should not be
/// exchanged throughout various instances of pairing heaps.
/// 
/// An special feature of addressable pairing heaps is the possibility to explicitely
/// decrease the key of an already stored element with the `decrease_key` operation which
/// simply increases the priority of the associated element.
/// 
/// It is possible to use different implementations for `Key` as the key type.
#[derive(Debug)]
pub struct PairingHeap<T, K>
	where K: Key
{
	/// Handle to the element with the minimum key within the pairing heap.
	min: Handle,
	/// The roots of the ```PairingHeap``` where
	/// the first root within this ```Vec``` always represents the one with the minimum ```key```.
	roots: Vec<Handle>,

	/// In the ```data``` vector all elements are stored.
	/// This indirection to the real data allows for efficient addressable elements via handles.
	data: Stash<Node<T, K>>
}

impl<T, K> PairingHeap<T, K>
	where K: Key
{
	/// Creates a new instance of a `PairingHeap`.
	pub fn new() -> Self {
		PairingHeap{
			min  : Handle::undef(),
			roots: Vec::new(),
			data : Stash::new()
		}
	}

	/// Returns the number of elements stored in this `PairingHeap`.
	pub fn len(&self) -> usize {
		self.data.len()
	}

	/// Returns true if this `PairingHeap` is empty.
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// Returns a reference to the `Node` that is associated with the given handle.
	/// Note that this won't fail on usage for a correct implementation of `PairingHeap`.
	fn get(&self, handle: Handle) -> &Node<T, K> {
		unsafe{ self.data.get_unchecked(handle.to_usize()) }
	}

	/// Returns a mutable reference to the `Node` that is associated with the given handle.
	/// Note that this won't fail on usage for a correct implementation of `PairingHeap`.
	fn get_mut(&mut self, handle: Handle) -> &mut Node<T, K> {
		unsafe{ self.data.get_unchecked_mut(handle.to_usize()) }
	}

	/// Links the given `lower` tree under the given `upper` tree thus making `lower`
	/// a children of `upper`.
	fn link(&mut self, upper: Handle, lower: Handle) {

		debug_assert!(upper != lower, "cannot link to self!");
		debug_assert!(self.get(lower).pos.is_root(), "lower cannot have multiple parents!");

		let idx = self.get(upper).children.len();
		self.get_mut(upper).children.push(lower);
		self.get_mut(lower).pos = Position::child(upper, idx);
		self.insert_root(upper);
	}

	/// Links the element with the lower key over the element with the higher key.
	/// Thus making one the child of the other.
	fn union(&mut self, fst: Handle, snd: Handle) {
		debug_assert!(fst != snd, "cannot union self with itself");

		if self.get(fst).entry.key < self.get(snd).entry.key {
			self.link(fst, snd)
		}
		else {
			self.link(snd, fst)
		}
	}

	/// Pairwise unifies roots in the `PairingHeap` which
	/// effectively decreases the number of roots to half.
	fn pairwise_union(&mut self) {
		let mut roots = Vec::with_capacity(self.roots.len());
		roots.append(&mut self.roots);
		if roots.len() % 2 == 0 {
			for (left, right) in roots.drain(..).tuples::<(_,_)>() {
				self.union(left, right)
			}
		}
		else if let Some((&fst, rest)) = roots.split_first() {
			self.insert_root(fst);
			for (&left, &right) in rest.iter().tuples::<(_,_)>() {
				self.union(left, right)
			}
		}
	}

	/// Updates the internal pointer to the current minimum element by hinting
	/// to a new possible min element within the heap.
	fn update_min(&mut self, handle: Handle) {
		if self.min.is_undef() || self.get(handle).entry.key < self.get(self.min).entry.key {
			self.min = handle;
		}
	}

	/// Creates a new root node.
	fn mk_root_node(&mut self, elem: T, key: K) -> Handle {
		let idx = self.len();
		Handle(
			self.data.put(
				Node::new_root(idx, Entry::new(key, elem))))
	}

	/// Inserts a new root into the `PairingHeap` and checks whether it is the new minimum element.
	fn insert_root(&mut self, new_root: Handle) {
		let idx = self.roots.len();
		self.roots.push(new_root);
		self.get_mut(new_root).pos = Position::root(idx);
		self.update_min(new_root);
	}

	/// Inserts the given element into the `PairingHeap` with its associated key
	/// and returns a `Handle` to it that allows to directly address it.
	/// 
	/// The handle is for example required in order to use methods like `decrease_key`.
	pub fn insert(&mut self, elem: T, key: K) -> Handle {
		let handle = self.mk_root_node(elem, key);
		self.insert_root(handle);
		handle

	}

	/// Cuts the given `child` from its parent and inserts it as a root into the `PairingHeap`.
	/// Will panic if the given `child` is not a child and thus a root node already.
	fn cut(&mut self, child: Handle) {
		debug_assert!(self.get(child).pos.is_child());

		match self.get(child).pos {
			Position::Root(_) => unsafe{ ::unreachable::unreachable() },
			Position::Child(parent, idx) => {
				self.get_mut(parent).children.swap_remove(idx);
				self.get_mut(child).pos = Position::root(self.len());
				self.insert_root(child);
			}
		}
	}

	/// Decreases the key of the element with the associated given `handle`.
	/// Will panic if the given new key is not lower than the previous key.
	pub fn decrease_key(&mut self, handle: Handle, new_key: K) -> Result<()> {
		if new_key >= self.get(handle).entry.key {
			return Err(Error::DecreaseKeyOutOfOrder)
		}

		self.get_mut(handle).entry.key = new_key;
		match self.get(handle).pos {
			Position::Root(_) => {
				self.update_min(handle);
			},
			Position::Child(..) => {
				self.cut(handle)
			}
		}
		Ok(())
	}

	/// Returns a reference to the current minimum element if not empty.
	pub fn get_min(&self) -> Option<&T> {
		self.data
			.get(self.min.to_usize())
			.and_then(|node| Some(&node.entry.elem))
	}

	/// Returns a reference to the current minimum element without bounds checking.
	/// So use it very carefully!
	pub unsafe fn get_min_unchecked(&self) -> &T {
		&self.get(self.min).entry.elem
	}

	/// Returns a mutable reference to the current minimum element if not empty.
	pub fn get_min_mut(&mut self) -> Option<&mut T> {
		self.data
			.get_mut(self.min.to_usize())
			.and_then(|node| Some(&mut node.entry.elem))
	}

	/// Returns a reference to the current minimum element without bounds checking.
	/// So use it very carefully!
	pub unsafe fn get_min_unchecked_mut(&mut self) -> &mut T {
		let min = self.min;
		&mut self.get_mut(min).entry.elem
	}

	/// Removes the element associated with the minimum key within this `PairingHeap` and returns it.
	pub fn take_min(&mut self) -> Option<T> {
		match self.is_empty() {
			true => None,
			_    => unsafe{ Some(self.take_min_unchecked()) }
		}
	}

	/// Removes the element associated with the minimum key within this `PairingHeap` without
	/// checking for emptiness and returns it.
	/// 
	/// So use this method carefully!
	pub unsafe fn take_min_unchecked(&mut self) -> T {
		let min = self.min;
		match self.get(min).pos {
			Position::Child(..) => ::unreachable::unreachable(),
			Position::Root(idx) => {
				self.roots.swap_remove(idx);
				self.min = Handle::undef();
				let mut roots = Vec::with_capacity(self.get(min).children.len());
				roots.append(&mut self.get_mut(min).children);
				for &child in roots.iter() {
					self.insert_root(child);
				}
				self.pairwise_union();
				self.data.take_unchecked(min.to_usize()).entry.elem
			}
		}
	}

	/// Iterate over the values in this `PairingHeap` by reference in unspecified order.
	pub fn values<'a>(&'a self) -> Values<'a, T, K> {
		Values{iter: self.data.values()}
	}

	/// Iterate over the values in this `PairingHeap` by mutable reference unspecified order.
	pub fn values_mut<'a>(&'a mut self) -> ValuesMut<'a, T, K> {
		ValuesMut{iter: self.data.values_mut()}
	}

	/// Iterate over values stored within a `PairingHeap` in a sorted-by-min order. Drains the heap.
	pub fn drain_min(self) -> DrainMin<T, K> {
		DrainMin{heap: self}
	}
}

use std::ops::{Index, IndexMut};

impl<T, K> Index<Handle> for PairingHeap<T, K>
	where K: Key
{
	type Output = T;

	fn index(&self, handle: Handle) -> &Self::Output {
		&self.data
			.get(handle.to_usize())
			.expect("no node found for given handle")
			.entry.elem
	}
}

impl<T, K> IndexMut<Handle> for PairingHeap<T, K>
	where K: Key
{
	fn index_mut(&mut self, handle: Handle) -> &mut Self::Output {
		&mut self.data
			.get_mut(handle.to_usize())
			.expect("no node found for given handle")
			.entry.elem
	}
}

/// Iterator over references to values stored within a `PairingHeap`.
pub struct Values<'a, T: 'a, K: 'a + Key> {
	iter: ::stash::stash::Values<'a, Node<T, K>>
}

/// Iterator over mutable references to values stored within a `PairingHeap`.
pub struct ValuesMut<'a, T: 'a, K: 'a + Key> {
	iter: ::stash::stash::ValuesMut<'a, Node<T, K>>
}

impl<'a, T, K: Key> Iterator for Values<'a, T, K> {
	type Item = &'a T;

	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|node| &node.entry.elem)
	}
}

impl<'a, T, K: Key> Iterator for ValuesMut<'a, T, K> {
	type Item = &'a mut T;

	fn next(&mut self) -> Option<Self::Item> {
		self.iter.next().map(|node| &mut node.entry.elem)
	}
}

/// Iterator over values stored within a `PairingHeap` in a sorted-by-min order. Drains the heap.
pub struct DrainMin<T, K: Key> {
	heap: PairingHeap<T, K>
}

impl<T, K: Key> Iterator for DrainMin<T, K> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		self.heap.take_min()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn take_min() {
		let mut ph = PairingHeap::new();
		ph.insert(0,   6);
		ph.insert(1,  10);
		ph.insert(2, -42);
		ph.insert(3,1337);
		ph.insert(4,  -1);
		ph.insert(5,   1);
		ph.insert(6,   2);
		ph.insert(7,   3);
		ph.insert(8,   4);
		ph.insert(9,   5);
		assert_eq!(Some(2), ph.take_min());
		assert_eq!(Some(4), ph.take_min());
		assert_eq!(Some(5), ph.take_min());
		assert_eq!(Some(6), ph.take_min());
		assert_eq!(Some(7), ph.take_min());
		assert_eq!(Some(8), ph.take_min());
		assert_eq!(Some(9), ph.take_min());
		assert_eq!(Some(0), ph.take_min());
		assert_eq!(Some(1), ph.take_min());
		assert_eq!(Some(3), ph.take_min());
		assert_eq!(None   , ph.take_min());
	}

	#[test]
	fn decrease_key() {
		let mut ph = PairingHeap::new();
		let a = ph.insert(0,   0);
		let b = ph.insert(1,  50);
		let c = ph.insert(2, 100);
		let d = ph.insert(3, 150);
		let e = ph.insert(4, 200);
		let f = ph.insert(5, 250);
		assert_eq!(Some(&0), ph.get_min());
		assert_eq!(Ok(()), ph.decrease_key(f, -50));
		assert_eq!(Some(&5), ph.get_min());
		assert_eq!(Ok(()), ph.decrease_key(e, -100));
		assert_eq!(Some(&4), ph.get_min());
		assert_eq!(Ok(()), ph.decrease_key(d, -99));
		assert_eq!(Some(&4), ph.get_min());
		assert_eq!(Err(Error::DecreaseKeyOutOfOrder), ph.decrease_key(c, 1000));
		assert_eq!(Some(&4), ph.get_min());
		assert_eq!(Ok(()), ph.decrease_key(b, -1000));
		assert_eq!(Some(&1), ph.get_min());
		assert_eq!(Err(Error::DecreaseKeyOutOfOrder), ph.decrease_key(a, 100));
		assert_eq!(Some(&1), ph.get_min());
	}

	#[test]
	fn empty_take() {
		let mut ph = PairingHeap::<usize, usize>::new();
		assert_eq!(None, ph.take_min());
	}

	fn setup() -> PairingHeap<char, i64> {
		let mut ph = PairingHeap::new();
		ph.insert('a', 100);
		ph.insert('b',  50);
		ph.insert('c', 150);
		ph.insert('d', -25);
		ph.insert('e', 999);
		ph.insert('f',  42);
		ph.insert('g',  43);
		ph.insert('i',  41);
		ph.insert('j',-100);
		ph.insert('k', -77);
		ph.insert('l', 123);
		ph.insert('m',-123);
		ph.insert('n',   0);
		ph.insert('o',  -1);
		ph.insert('p',   2);
		ph.insert('q',  -3);
		ph.insert('r',   4);
		ph.insert('s',  -5);
		ph
	}

	#[test]
	fn drain_min() {
		let ph = setup();
		let mut drain = ph.drain_min();

		assert_eq!(drain.next(), Some('m'));
		assert_eq!(drain.next(), Some('j'));
		assert_eq!(drain.next(), Some('k'));
		assert_eq!(drain.next(), Some('d'));
		assert_eq!(drain.next(), Some('s'));
		assert_eq!(drain.next(), Some('q'));
		assert_eq!(drain.next(), Some('o'));
		assert_eq!(drain.next(), Some('n'));

		assert_eq!(drain.next(), Some('p'));
		assert_eq!(drain.next(), Some('r'));
		assert_eq!(drain.next(), Some('i'));
		assert_eq!(drain.next(), Some('f'));
		assert_eq!(drain.next(), Some('g'));
		assert_eq!(drain.next(), Some('b'));
		assert_eq!(drain.next(), Some('a'));
		assert_eq!(drain.next(), Some('l'));
		assert_eq!(drain.next(), Some('c'));
		assert_eq!(drain.next(), Some('e'));

		assert_eq!(drain.next(), None);
	}

	#[test]
	fn values() {
		let ph = setup();
		let values = ph.values();

		// cannot test order of values since it is unspecified!
		assert_eq!(values.count(), 18);
	}
}
