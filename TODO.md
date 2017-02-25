TODO List for Addressable Pairing Heap
======================================

- add tests for `Values`, `ValuesMut` and `DrainMin` iterators.
- add `DefaultPairingHeap<T>` that defaults the `K` (key) type to `i64`.
- implement `PairingHeap::get_key(h: Handle) -> K` method to retrieve the key associated with the element that is associated with the given handle `h`.
- implement `PairingHeap::merge` to efficiently merge two instances of `PairingHeap` together. Should result in improved performance compared to naive approach.
- implement `unsafe` versions of `get_min`, `get_min_mut` and `take_min`.
- add new tests to test `PairingHeap` implementation.
