TODO List for Addressable Pairing Heap
======================================

- implement a `Result` and `Error` type for `PairingHeap` and fix associated test.
- change return type of `PairingHeap::decrease_key` to `Result` that fails on higher new keys.
- implement `Values` and `ValuesMut` iterators to iterate over elements stored in the `PairingHeap`.
- implement `DrainMin` iterator to iterate over elements in an ordered fashion. This drains the `PairingHeap`.
- implement `PairingHeap::get_key(h: Handle) -> K` method to retrieve the key associated with the element that is associated with the given handle `h`.
- implement `PairingHeap::merge` to efficiently merge two instances of `PairingHeap` together. Should result in improved performance compared to naive approach.
- implement `unsafe` versions of `get_min`, `get_min_mut` and `take_min`.
- add new tests to test `PairingHeap` implementation.
- add `DefaultPairingHeap<T>` that defaults the `K` (key) type to `i64`.
- add crates.io cathegories and improve `Cargo.toml` in general.
