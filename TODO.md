TODO List for Addressable Pairing Heap
======================================

- Implement `PairingHeap::get_key(h: Handle) -> K` method to retrieve the key associated with the element that is associated with the given handle `h`.

- Find a solution to efficiently support merging of `PairingHeap` instances.
- Find a better API for `decrease_key`. Maybe `set_key` which is more efficient for lowering?
- Improve docs with code examples.
- Add benchmarks:
   - To compare performance between `SmallVec` and `Vec` for children storage in `Node`.
   - To compare performance between `PairingHeap` and `BinaryHeap` of the standard library.

- Renames:
   - `insert` to `push`
   - `take_min` to `pop`
   - `get` to `resolve`
   - `get_mut` to `resolve_mut`
   - `get_min` to `peek`
   - `get_min_mut` to `peek_mut`

- Add methods:
   - `get(&self, handle: Handle) -> Option<&T>`
   - `get_unchecked(&self, handle: Handle) -> &T`
   - `get_mut(&mut self, handle: Handle) -> Option<&mut T>`
   - `get_unchecked_mut(&mut self, handle: Handle) -> &mut T`
