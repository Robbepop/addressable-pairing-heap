TODO List for Addressable Pairing Heap
======================================

- add tests for `Values`, `ValuesMut` and `DrainMin` iterators.
- implement `PairingHeap::get_key(h: Handle) -> K` method to retrieve the key associated with the element that is associated with the given handle `h`.
- implement `PairingHeap::merge` to efficiently merge two instances of `PairingHeap` together. Should result in improved performance compared to naive approach.
- add new tests to test `PairingHeap` implementation.
