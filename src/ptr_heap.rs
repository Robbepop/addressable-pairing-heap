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

use stash::*;
// use itertools::*;

/// A handle to access stored elements within an addressable pairing heap.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Handle(usize);

impl Handle {
    #[inline]
    fn uninitialized() -> Self {
        Handle(usize::max_value())
    }
}

impl From<usize> for Handle {
    fn from(val: usize) -> Handle {
        Handle(val)
    }
}

impl From<Handle> for usize {
    fn from(handle: Handle) -> usize {
        handle.0
    }
}

/// Represents a trait for keys within an addressable pairing heap.
///
/// A user can use custom type for the key type by implementing this trait.
///
/// This trait is implicitely implemented already for all types that
/// are `Copy`, `PartialOrd` and `Ord`.
pub trait Key: Copy + PartialOrd + Ord {}
impl<T> Key for T where T: Copy + PartialOrd + Ord {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Node<K>
where
    K: Key,
{
    parent: Option<Handle>,
    child: Option<Handle>,
    left: Handle,
    right: Handle,
    key: K,
}

impl<K> Node<K>
where
    K: Key,
{
    #[inline]
    fn with_key(key: K) -> Self {
        Node {
            parent: None,
            child: None,
            left: Handle::uninitialized(),
            right: Handle::uninitialized(),
            key: key,
        }
    }

    fn is_child(&self) -> bool {
        !self.is_root()
    }

    fn is_root(&self) -> bool {
        self.parent.is_none()
    }
}

/// Errors that can be caused while using `PairingHeap`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    /// Caused when using `decrease_key` method with a `new_key` that is greater than the old one.
    DecreaseKeyOutOfOrder,
}

/// Generic `Result` type for `PairingHeap` methods.
pub type Result<T> = ::std::result::Result<T, Error>;

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
#[derive(Debug, Clone)]
pub struct PairingHeap<T, K>
where
    K: Key,
{
    /// Handle to the element with the minimum key within the pairing heap.
    min: Option<Handle>,

    /// In the ```data``` vector all elements are stored.
    /// This indirection to the real data allows for efficient addressable elements via handles.
    nodes: Stash<Node<K>, Handle>,
    elems: Stash<T, Handle>,
}

struct RawHandleIter {
    sentinel: Handle,
    peek: Handle,
    done: bool,
}

impl RawHandleIter {
    fn children<T, K>(heap: &PairingHeap<T, K>, parent: Handle) -> RawHandleIter
    where
        K: Key,
    {
        match heap.node(parent).child {
            None => RawHandleIter::empty(),
            Some(child) => RawHandleIter::siblings(child),
        }
    }

    fn siblings(handle: Handle) -> RawHandleIter {
        RawHandleIter {
            sentinel: handle,
            peek: handle,
            done: false,
        }
    }

    fn empty() -> RawHandleIter {
        RawHandleIter {
            sentinel: Handle::uninitialized(),
            peek: Handle::uninitialized(),
            done: true,
        }
    }

    fn next<T, K>(&mut self, heap: &PairingHeap<T, K>) -> Option<Handle>
    where
        K: Key,
    {
        match self.done {
            true => None,
            false => {
                let next = self.peek;
                self.peek = heap.node(next).right;
                if self.peek == self.sentinel {
                    self.done = true;
                }
                Some(next)
            }
        }
    }
}

struct HandleIter<'a, T, K>
where
    K: Key + 'a,
    T: 'a,
{
    heap: &'a PairingHeap<T, K>,
    iter: RawHandleIter, // sentinel: Handle,
                         // peek    : Handle,
                         // done    : bool
}

impl<'a, T, K> HandleIter<'a, T, K>
where
    K: Key + 'a,
    T: 'a,
{
    /// Iterator over the children of the given parent node.
    fn children(heap: &'a PairingHeap<T, K>, parent: Handle) -> HandleIter<'a, T, K> {
        HandleIter {
            heap: heap,
            iter: RawHandleIter::children(heap, parent),
        }
        // match heap.node(parent).child {
        // 	None => HandleIter{
        // 		heap: heap,
        // 		iter: RawHandleIter::empty()
        // 		// sentinel: Handle::uninitialized(),
        // 		// peek    : Handle::uninitialized(),
        // 		// done    : true
        // 	},
        // 	Some(child) => HandleIter::siblings(heap, child)
        // }
    }

    /// Iterator over the siblings of the given child node.
    ///
    /// This also iterates inclusively over the given child.
    fn siblings(heap: &'a PairingHeap<T, K>, child: Handle) -> HandleIter<'a, T, K> {
        HandleIter {
            heap: heap,
            iter: RawHandleIter::siblings(child), // sentinel: child,
                                                  // peek    : child,
                                                  // done    : false
        }
    }
}

impl<'a, T, K> Iterator for HandleIter<'a, T, K>
where
    K: Key + 'a,
    T: 'a,
{
    type Item = Handle;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next(self.heap)
        // match self.done {
        // 	true  => None,
        // 	false => {
        // 		let next = self.peek;
        // 		self.peek = self.heap.node(next).right;
        // 		if self.peek == self.sentinel {
        // 			self.done = true;
        // 		}
        // 		Some(next)
        // 	}
        // }
    }
}

impl<T, K> PairingHeap<T, K>
where
    K: Key,
{
    /// Creates a new instance of a `PairingHeap`.
    #[inline]
    pub fn new() -> Self {
        PairingHeap {
            min: None,
            nodes: Stash::default(),
            elems: Stash::default(),
        }
    }

    /// Returns the number of elements stored in this `PairingHeap`.
    #[inline]
    pub fn len(&self) -> usize {
        self.elems.len()
    }

    /// Returns true if this `PairingHeap` is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a reference to the `Node` that is associated with the given handle.
    /// Note that this won't fail on usage for a correct implementation of `PairingHeap`.
    #[inline]
    fn node(&self, handle: Handle) -> &Node<K> {
        unsafe { self.nodes.get_unchecked(handle) }
    }

    /// Returns a mutable reference to the `Node` that is associated with the given handle.
    /// Note that this won't fail on usage for a correct implementation of `PairingHeap`.
    #[inline]
    fn node_mut(&mut self, handle: Handle) -> &mut Node<K> {
        unsafe { self.nodes.get_unchecked_mut(handle) }
    }

    fn raw_children(&self, parent: Handle) -> RawHandleIter {
        RawHandleIter::children(self, parent)
    }

    fn raw_siblings(&self, sibling: Handle) -> RawHandleIter {
        RawHandleIter::siblings(sibling)
    }

    /// Returns an iterator over all children of the given parent node.
    #[inline]
    fn children<'a>(&'a self, parent: Handle) -> HandleIter<'a, T, K> {
        HandleIter::children(self, parent)
    }

    /// Returns an iterator over all siblings of a given child node.
    ///
    /// This also iterates inclusively over the given child.
    #[inline]
    fn siblings<'a>(&'a self, child: Handle) -> HandleIter<'a, T, K> {
        HandleIter::siblings(self, child)
    }

    /// Adds the given new child to the given child's siblings.
    #[inline]
    fn add_sibling(&mut self, child: Handle, new_child: Handle) {
        self.detach_siblings(new_child); // experimental!
        self.node_mut(new_child).parent = self.node(child).parent;
        self.node_mut(new_child).right = self.node(child).right;
        self.node_mut(new_child).left = child;
        self.node_mut(child).right = new_child;
        let rightright = self.node(new_child).right;
        self.node_mut(rightright).left = new_child;
    }

    /// Adds the given child to the parent node.
    #[inline]
    fn add_child(&mut self, parent: Handle, new_child: Handle) {
        self.detach_siblings(new_child); // experimental!
        match self.node(parent).child {
            None => {
                self.node_mut(parent).child = Some(new_child);
                self.node_mut(new_child).left = new_child;
                self.node_mut(new_child).right = new_child;
                self.node_mut(new_child).parent = Some(parent);
            }
            Some(child) => self.add_sibling(child, new_child),
        }
    }

    /// Links the given `lower` tree under the given `upper` tree thus making `lower`
    /// a children of `upper`.
    fn link(&mut self, upper: Handle, lower: Handle) {
        debug_assert!(upper != lower, "cannot link to self!");
        debug_assert!(
            self.node(lower).is_root(),
            "lower cannot have multiple parents!"
        );

        self.add_child(upper, lower);
        self.update_min(upper);
    }

    /// Links the element with the lower key over the element with the higher key.
    /// Thus making one the child of the other.
    fn union(&mut self, fst: Handle, snd: Handle) {
        debug_assert!(self.node(fst).is_root());
        debug_assert!(self.node(snd).is_root());
        debug_assert!(fst != snd, "cannot union self with itself");

        if self.node(fst).key < self.node(snd).key {
            self.link(fst, snd)
        } else {
            self.link(snd, fst)
        }
    }

    /// Pairwise unifies roots in the `PairingHeap` which
    /// effectively decreases the number of roots to half.
    fn pairwise_union(&mut self) {
        if let Some(min) = self.min {
            let mut siblings = self.siblings(min).collect::<Vec<_>>().into_iter();
            loop {
                match (siblings.next(), siblings.next()) {
                    (Some(left), Some(right)) => self.union(left, right),
                    (Some(left), None) => self.update_min(left),
                    _ => break,
                }
            }
            // let mut raw_siblings = self.raw_siblings(min);
            // loop {
            // 	match (raw_siblings.next(self), raw_siblings.next(self)) {
            // 		(Some(left), Some(right)) => self.union(left, right),
            // 		(Some(left), None       ) => self.update_min(left),
            // 		_                         => break
            // 	}
            // }
        }
    }

    /// Adds the given handle as a new root node into the heap.
    #[inline]
    fn insert_root(&mut self, new_root: Handle) {
        // self.detach_siblings(new_root); // experimental!
        match self.min {
            None => {
                self.node_mut(new_root).parent = None;
                self.min = Some(new_root);
            }
            Some(min) => {
                self.add_sibling(min, new_root);
                self.update_min(new_root);
            }
        }
        debug_assert!(self.node(new_root).is_root());
    }

    /// Updates the internal pointer to the current minimum element by hinting
    /// to a new possible min element within the heap.
    #[inline]
    fn update_min(&mut self, new: Handle) {
        match self.min {
            None => {
                self.min = Some(new);
            }
            Some(min) => {
                if self.node(new).key < self.node(min).key {
                    self.min = Some(new);
                }
            }
        }
    }

    /// Creates a new root node.
    #[inline]
    fn make_entry(&mut self, key: K, elem: T) -> Handle {
        let node_handle = self.nodes.put(Node::with_key(key));
        self.node_mut(node_handle).left = node_handle;
        self.node_mut(node_handle).right = node_handle;
        let elem_handle = self.elems.put(elem);
        debug_assert_eq!(node_handle, elem_handle);
        node_handle
    }

    /// Inserts the given element into the `PairingHeap` with its associated key
    /// and returns a `Handle` to it that allows to directly address it.
    ///
    /// The handle is for example required in order to use methods like `decrease_key`.
    #[inline]
    pub fn push(&mut self, elem: T, key: K) -> Handle {
        let handle = self.make_entry(key, elem);
        self.insert_root(handle);
        handle
    }

    /// Detaches the given child from its siblings.
    #[inline]
    fn detach_siblings(&mut self, child: Handle) {
        let right = self.node(child).right;
        let left = self.node(child).left;

        self.node_mut(right).left = left;
        self.node_mut(left).right = right;

        // self.node_mut(child).left  = child;
        // self.node_mut(child).right = child;
    }

    /// Cuts the given `child` from its parent and inserts it as a root into the `PairingHeap`.
    /// Will panic if the given `child` is not a child and thus a root node already.
    #[inline]
    fn cut(&mut self, child: Handle) {
        debug_assert!(self.node(child).is_child());

        self.detach_siblings(child);
        self.insert_root(child);
    }

    /// Decreases the key of the element with the associated given `handle`.
    /// Will panic if the given new key is not lower than the previous key.
    pub fn decrease_key(&mut self, handle: Handle, new_key: K) -> Result<()> {
        if new_key >= self.node(handle).key {
            return Err(Error::DecreaseKeyOutOfOrder);
        }

        self.node_mut(handle).key = new_key;
        match self.node(handle).is_root() {
            true => self.update_min(handle),
            false => self.cut(handle),
        }
        Ok(())
    }

    /// Release children from the given parent making them root nodes.
    fn release_children(&mut self, parent: Handle) {
        let mut raw_children = self.raw_children(parent);
        while let Some(child) = raw_children.next(self) {
            self.insert_root(child)
        }
        // for child in self.children(parent).collect::<Vec<_>>() {
        // 	self.insert_root(child)
        // }
        self.node_mut(parent).child = None;
    }

    /// Removes the element associated with the minimum key within this `PairingHeap` and returns it.
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        match self.min {
            Some(_) => Some(unsafe { self.pop_unchecked() }),
            None => None,
        }
    }

    /// Removes the element associated with the minimum key within this `PairingHeap` without
    /// checking for emptiness and returns it.
    ///
    /// So use this method carefully!
    pub unsafe fn pop_unchecked(&mut self) -> T {
        match self.min {
            None => ::unreachable::unreachable(),
            Some(min) => {
                self.release_children(min);
                self.min = Some(self.node(min).right);
                let right = self.node(min).right;
                if right != min {
                    self.min = Some(right);
                    self.detach_siblings(min);
                    self.pairwise_union();
                } else {
                    self.min = None;
                }
                self.nodes.take_unchecked(min);
                self.elems.take_unchecked(min)
            }
        }
    }

    /// Returns a reference to the element associated with the given handle.
    #[inline]
    pub fn get(&self, handle: Handle) -> Option<&T> {
        self.elems.get(handle)
    }

    /// Returns a mutable reference to the element associated with the given handle.
    #[inline]
    pub fn get_mut(&mut self, handle: Handle) -> Option<&mut T> {
        self.elems.get_mut(handle)
    }

    /// Returns a reference to the element associated with the given handle.
    ///
    /// Does not perform bounds checking so use it carefully!
    #[inline]
    pub unsafe fn get_unchecked(&self, handle: Handle) -> &T {
        self.elems.get_unchecked(handle)
    }

    /// Returns a mutable reference to the element associated with the given handle.
    ///
    /// Does not perform bounds checking so use it carefully!
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, handle: Handle) -> &mut T {
        self.elems.get_unchecked_mut(handle)
    }

    /// Returns a reference to the current minimum element if not empty.
    #[inline]
    pub fn peek(&self) -> Option<&T> {
        match self.min {
            Some(min) => self.get(min),
            None => None,
        }
    }

    /// Returns a reference to the current minimum element.
    ///
    /// Does not perform bounds checking so use it carefully!
    #[inline]
    pub unsafe fn peek_unchecked(&self) -> &T {
        match self.min {
            Some(min) => self.get_unchecked(min),
            None => ::unreachable::unreachable(),
        }
    }

    /// Returns a mutable reference to the current minimum element if not empty.
    #[inline]
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        match self.min {
            Some(min) => self.get_mut(min),
            None => None,
        }
    }

    /// Returns a reference to the current minimum element without bounds checking.
    /// So use it very carefully!
    #[inline]
    pub unsafe fn peek_unchecked_mut(&mut self) -> &mut T {
        match self.min {
            Some(min) => self.get_unchecked_mut(min),
            None => ::unreachable::unreachable(),
        }
    }

    /// Iterate over the values in this `PairingHeap` by reference in unspecified order.
    #[inline]
    pub fn values<'a>(&'a self) -> stash::Values<'a, T> {
        self.elems.values()
    }

    /// Iterate over the values in this `PairingHeap` by mutable reference unspecified order.
    #[inline]
    pub fn values_mut<'a>(&'a mut self) -> stash::ValuesMut<'a, T> {
        self.elems.values_mut()
    }

    /// Iterate over values stored within a `PairingHeap` in a sorted-by-min order. Drains the heap.
    #[inline]
    pub fn drain_min(self) -> DrainMin<T, K> {
        DrainMin { heap: self }
    }
}

use std::ops::{Index, IndexMut};

impl<T, K> Index<Handle> for PairingHeap<T, K>
where
    K: Key,
{
    type Output = T;

    fn index(&self, handle: Handle) -> &Self::Output {
        self.elems
            .get(handle)
            .expect("no node found for given handle")
    }
}

impl<T, K> IndexMut<Handle> for PairingHeap<T, K>
where
    K: Key,
{
    fn index_mut(&mut self, handle: Handle) -> &mut Self::Output {
        self.elems
            .get_mut(handle)
            .expect("no node found for given handle")
    }
}

/// Iterator over values stored within a `PairingHeap` in a sorted-by-min order. Drains the heap.
pub struct DrainMin<T, K: Key> {
    heap: PairingHeap<T, K>,
}

impl<T, K: Key> Iterator for DrainMin<T, K> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.heap.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn take_min() {
        let mut ph = PairingHeap::new();
        ph.push(0, 6);
        ph.push(1, 10);
        ph.push(2, -42);
        ph.push(3, 1337);
        ph.push(4, -1);
        ph.push(5, 1);
        ph.push(6, 2);
        ph.push(7, 3);
        ph.push(8, 4);
        ph.push(9, 5);
        println!("ph = {:?}", ph);
        assert_eq!(Some(2), ph.pop());
        println!("ph = {:?}", ph);
        assert_eq!(Some(4), ph.pop());
        println!("ph = {:?}", ph);
        assert_eq!(Some(5), ph.pop());
        println!("ph = {:?}", ph);
        assert_eq!(Some(6), ph.pop());
        println!("ph = {:?}", ph);
        assert_eq!(Some(7), ph.pop());
        println!("ph = {:?}", ph);
        assert_eq!(Some(8), ph.pop());
        println!("ph = {:?}", ph);
        assert_eq!(Some(9), ph.pop());
        println!("ph = {:?}", ph);
        assert_eq!(Some(0), ph.pop());
        println!("ph = {:?}", ph);
        assert_eq!(Some(1), ph.pop());
        println!("ph = {:?}", ph);
        assert_eq!(Some(3), ph.pop());
        println!("ph = {:?}", ph);
        assert_eq!(None, ph.pop());
        println!("ph = {:?}", ph);
    }

    #[test]
    fn decrease_key() {
        let mut ph = PairingHeap::new();
        let a = ph.push(0, 0);
        let b = ph.push(1, 50);
        let c = ph.push(2, 100);
        let d = ph.push(3, 150);
        let e = ph.push(4, 200);
        let f = ph.push(5, 250);
        assert_eq!(Some(&0), ph.peek());
        assert_eq!(Ok(()), ph.decrease_key(f, -50));
        assert_eq!(Some(&5), ph.peek());
        assert_eq!(Ok(()), ph.decrease_key(e, -100));
        assert_eq!(Some(&4), ph.peek());
        assert_eq!(Ok(()), ph.decrease_key(d, -99));
        assert_eq!(Some(&4), ph.peek());
        assert_eq!(Err(Error::DecreaseKeyOutOfOrder), ph.decrease_key(c, 1000));
        assert_eq!(Some(&4), ph.peek());
        assert_eq!(Ok(()), ph.decrease_key(b, -1000));
        assert_eq!(Some(&1), ph.peek());
        assert_eq!(Err(Error::DecreaseKeyOutOfOrder), ph.decrease_key(a, 100));
        assert_eq!(Some(&1), ph.peek());
    }

    #[test]
    fn empty_take() {
        let mut ph = PairingHeap::<usize, usize>::new();
        assert_eq!(None, ph.pop());
    }

    fn setup() -> PairingHeap<char, i64> {
        let mut ph = PairingHeap::new();
        ph.push('a', 100);
        ph.push('b', 50);
        ph.push('c', 150);
        ph.push('d', -25);
        ph.push('e', 999);
        ph.push('f', 42);
        ph.push('g', 43);
        ph.push('i', 41);
        ph.push('j', -100);
        ph.push('k', -77);
        ph.push('l', 123);
        ph.push('m', -123);
        ph.push('n', 0);
        ph.push('o', -1);
        ph.push('p', 2);
        ph.push('q', -3);
        ph.push('r', 4);
        ph.push('s', -5);
        ph
    }

    // fn setup_vec() -> Vec<(char, i64)> {
    // 	vec![
    // 		('a',  0), ('A', 26), ('.', 52),
    // 		('b',  1), ('B', 27), (',', 53),
    // 		('c',  2), ('C', 28), (';', 54),
    // 		('d',  3), ('D', 29), ('!', 55),
    // 		('e',  4), ('E', 30), ('&', 56),
    // 		('f',  5), ('F', 31), ('|', 57),
    // 		('g',  6), ('G', 32), ('(', 58),
    // 		('h',  7), ('H', 33), (')', 59),
    // 		('i',  8), ('I', 34), ('[', 60),
    // 		('j',  9), ('J', 35), (']', 61),
    // 		('k', 10), ('K', 36), ('{', 62),
    // 		('l', 11), ('L', 37), ('}', 63),
    // 		('m', 12), ('M', 38), ('=', 64),
    // 		('n', 13), ('N', 39), ('?', 65),
    // 		('o', 14), ('O', 40), ('+', 66),
    // 		('p', 15), ('P', 41), ('-', 67),
    // 		('q', 16), ('Q', 42), ('*', 68),
    // 		('r', 17), ('R', 43), ('/', 69),
    // 		('s', 18), ('S', 44), ('<', 70),
    // 		('t', 19), ('T', 45), ('>', 71),
    // 		('u', 20), ('U', 46), ('=', 72),
    // 		('v', 21), ('V', 47), ('#', 73),
    // 		('w', 22), ('W', 48), ('~', 74),
    // 		('x', 23), ('X', 49), ('?', 75),
    // 		('y', 24), ('Y', 50), (':', 76),
    // 		('z', 25), ('Z', 51), ('^', 77)
    // 	]
    // }

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

#[cfg(all(feature = "bench", test))]
mod bench {
    use super::*;
    use std::collections::BinaryHeap;
    use test::{black_box, Bencher};

    fn setup_sample() -> Vec<i64> {
        use rand::{sample, thread_rng};
        let n = 100_000;
        let mut rng = thread_rng();
        sample(&mut rng, 1..n, n as usize)
    }

    fn setup_sample_bigpod() -> Vec<BigPod> {
        use rand::{sample, thread_rng};
        let n = 100_000;
        let mut rng = thread_rng();
        sample(&mut rng, 1..n, n as usize)
            .into_iter()
            .map(|val| val.into())
            .collect::<Vec<BigPod>>()
    }

    #[derive(Debug, Clone, PartialEq, Eq, Ord)]
    struct BigPod {
        elems0: [i64; 32],
        elems1: [i64; 32],
        elems2: [i64; 32],
        elems3: [i64; 32],
    }

    impl From<i64> for BigPod {
        fn from(val: i64) -> BigPod {
            let mut bp = BigPod {
                elems0: [0; 32],
                elems1: [1; 32],
                elems2: [2; 32],
                elems3: [3; 32],
            };
            bp.elems0[0] = val;
            bp
        }
    }

    impl PartialOrd for BigPod {
        fn partial_cmp(&self, other: &BigPod) -> Option<::std::cmp::Ordering> {
            self.elems0[0].partial_cmp(&other.elems0[0])
        }
    }

    #[bench]
    fn ptr_pairing_heap_push(bencher: &mut Bencher) {
        let sample = setup_sample();
        bencher.iter(|| {
            let mut ph = PairingHeap::new();
            for &key in sample.iter() {
                black_box(ph.push((), key));
            }
        });
    }

    #[bench]
    fn ptr_pairing_heap_push_bigpod(bencher: &mut Bencher) {
        let sample = setup_sample_bigpod();
        bencher.iter(|| {
            let mut ph = PairingHeap::new();
            for bigpod in sample.iter() {
                black_box(ph.push(bigpod.clone(), bigpod.elems0[0]));
            }
        });
    }

    #[bench]
    fn binary_heap_push(bencher: &mut Bencher) {
        let sample = setup_sample();
        bencher.iter(|| {
            let mut bh = BinaryHeap::new();
            for &key in sample.iter() {
                black_box(bh.push(key));
            }
        });
    }

    #[bench]
    fn binary_heap_push_bigpod(bencher: &mut Bencher) {
        let sample = setup_sample_bigpod();
        bencher.iter(|| {
            let mut bh = BinaryHeap::new();
            for bigpod in sample.iter() {
                black_box(bh.push(bigpod.clone()));
            }
        });
    }

    #[bench]
    fn ptr_pairing_heap_pop(bencher: &mut Bencher) {
        let mut ph = PairingHeap::new();
        for key in setup_sample().into_iter() {
            ph.push((), key);
        }
        bencher.iter(|| {
            let mut ph = ph.clone();
            while let Some(_) = black_box(ph.pop()) {}
        });
    }

    #[bench]
    fn ptr_pairing_heap_pop_bigpod(bencher: &mut Bencher) {
        let mut ph = PairingHeap::new();
        for bigpod in setup_sample_bigpod().into_iter() {
            let head = bigpod.elems0[0];
            ph.push(bigpod, head);
        }
        bencher.iter(|| {
            let mut ph = ph.clone();
            while let Some(_) = black_box(ph.pop()) {}
        });
    }

    #[bench]
    fn binary_heap_pop(bencher: &mut Bencher) {
        let mut bh = BinaryHeap::new();
        for key in setup_sample().into_iter() {
            bh.push(key);
        }
        bencher.iter(|| {
            let mut bh = bh.clone();
            while let Some(_) = black_box(bh.pop()) {}
        });
    }

    #[bench]
    fn binary_heap_pop_bigpod(bencher: &mut Bencher) {
        let mut bh = BinaryHeap::new();
        for bigpod in setup_sample_bigpod().into_iter() {
            bh.push(bigpod);
        }
        bencher.iter(|| {
            let mut bh = bh.clone();
            while let Some(_) = black_box(bh.pop()) {}
        });
    }

    #[bench]
    fn ptr_pairing_heap_clone(bencher: &mut Bencher) {
        let mut ph = PairingHeap::new();
        for key in setup_sample().into_iter() {
            ph.push((), key);
        }
        bencher.iter(|| {
            black_box(&ph.clone());
        });
    }

    #[bench]
    fn binary_heap_clone(bencher: &mut Bencher) {
        let mut bh = BinaryHeap::new();
        for key in setup_sample().into_iter() {
            bh.push(key);
        }
        bencher.iter(|| {
            black_box(&bh.clone());
        });
    }
}
