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
test binary_heap_clone            ... bench:      27,059 ns/iter (+/- 600)
test binary_heap_pop              ... bench:   3,244,202 ns/iter (+/- 69,019)
test binary_heap_pop_bigpod       ... bench:  18,860,402 ns/iter (+/- 75,813)
test binary_heap_push             ... bench:   1,803,009 ns/iter (+/- 90,223)
test binary_heap_push_bigpod      ... bench:  10,559,628 ns/iter (+/- 194,993)
```

## Pairing Heap (offset-based implementation)
```
test ptr_pairing_heap_clone       ... bench:   1,451,785 ns/iter (+/- 18,567)
test ptr_pairing_heap_pop         ... bench:  21,101,458 ns/iter (+/- 512,387)
test ptr_pairing_heap_pop_bigpod  ... bench:  27,793,057 ns/iter (+/- 365,974)
test ptr_pairing_heap_push        ... bench:   4,604,275 ns/iter (+/- 97,326)
test ptr_pairing_heap_push_bigpod ... bench:  12,298,900 ns/iter (+/- 211,846)
```

## Pairing Heap (vec-based implementation)
```
test vec_pairing_heap_clone       ... bench:   1,725,481 ns/iter (+/- 43,596)
test vec_pairing_heap_pop         ... bench:  11,155,717 ns/iter (+/- 215,189)
test vec_pairing_heap_pop_bigpod  ... bench:  28,404,857 ns/iter (+/- 276,313)
test vec_pairing_heap_push        ... bench:   3,273,197 ns/iter (+/- 43,445)
test vec_pairing_heap_push_bigpod ... bench:  12,700,110 ns/iter (+/- 52,233)
```
