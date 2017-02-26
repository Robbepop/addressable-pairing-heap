[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/addressable-pairing-heap.svg)](https://crates.io/crates/addressable-pairing-heap)

Addressable Pairing-Heap
========================

An addressable pairing heap implementation for Rust.  

Allows to efficiently store elements associated with a key and access them via handles.
Handles to elements make it possible to query and edit elements stored in the `PairingHeap`.  

Documentation: [link](https://docs.rs/addressable-pairing-heap)  
crates.io: [link](https://crates.io/crates/addressable-pairing-heap)

Benchmarks show that the current implementation suffers from performance issues that have yet to be fixed:

## Binary Heap
```
test bench::binary_heap_clone        ... bench:      27,409 ns/iter (+/- 1,958)
test bench::binary_heap_pop          ... bench:   3,227,855 ns/iter (+/- 35,525)
test bench::binary_heap_pop_bigpod   ... bench:  17,386,429 ns/iter (+/- 85,175)
test bench::binary_heap_push         ... bench:   1,522,285 ns/iter (+/- 39,222)
test bench::binary_heap_push_bigpod  ... bench:  10,600,908 ns/iter (+/- 226,227)
```

## Pairing Heap
```
test bench::pairing_heap_clone       ... bench:   1,713,716 ns/iter (+/- 43,337)
test bench::pairing_heap_pop         ... bench:  11,255,949 ns/iter (+/- 209,302)
test bench::pairing_heap_pop_bigpod  ... bench:  26,683,916 ns/iter (+/- 136,507)
test bench::pairing_heap_push        ... bench:   2,644,503 ns/iter (+/- 34,863)
test bench::pairing_heap_push_bigpod ... bench:  10,997,608 ns/iter (+/- 161,914)

```
