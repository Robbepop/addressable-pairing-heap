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

## Binary Heap (standard-lib)
```
test binary_heap_clone            ... bench:      27,537 ns/iter (+/- 1,006)
test binary_heap_pop              ... bench:   3,312,799 ns/iter (+/- 63,197)
test binary_heap_pop_bigpod       ... bench:  80,815,870 ns/iter (+/- 234,285)
test binary_heap_push             ... bench:   1,878,911 ns/iter (+/- 105,333)
test binary_heap_push_bigpod      ... bench:  43,423,423 ns/iter (+/- 283,598)
```

## Pairing Heap (offset-based implementation)
```
test ptr_pairing_heap_clone       ... bench:   1,554,327 ns/iter (+/- 143,369)
test ptr_pairing_heap_pop         ... bench:  16,134,333 ns/iter (+/- 1,517,587)
test ptr_pairing_heap_pop_bigpod  ... bench:  48,197,309 ns/iter (+/- 157,727)
test ptr_pairing_heap_push        ... bench:   4,726,005 ns/iter (+/- 101,163)
test ptr_pairing_heap_push_bigpod ... bench:  39,347,464 ns/iter (+/- 446,011)
```

## Pairing Heap (vec-based implementation)
```
test vec_pairing_heap_clone       ... bench:   1,868,083 ns/iter (+/- 75,156)
test vec_pairing_heap_pop         ... bench:  11,877,821 ns/iter (+/- 637,347)
test vec_pairing_heap_pop_bigpod  ... bench:  58,990,571 ns/iter (+/- 370,100)
test vec_pairing_heap_push        ... bench:   2,960,636 ns/iter (+/- 117,900)
test vec_pairing_heap_push_bigpod ... bench:  26,025,722 ns/iter (+/- 57,091)
```
