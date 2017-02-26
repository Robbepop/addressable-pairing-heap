TODO List for Addressable Pairing Heap
======================================

- Implement `PairingHeap::get_key(h: Handle) -> K` method to retrieve the key associated with the element that is associated with the given handle `h`.
- Add benchmarks between `PairingHeap` and std `BinaryHeap`.
- Find a solution to efficiently support merging of `PairingHeap` instances.
- Find a better API for `decrease_key`. Maybe `set_key` which is more efficient for lowering?
- Improve docs with code examples.
- Add benchmarks to test whether `SmallVec` improves performance.
