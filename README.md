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
test bench::binary_heap_clone        ... bench:      28,823 ns/iter (+/- 1,823)
test bench::binary_heap_pop          ... bench:   3,373,355 ns/iter (+/- 203,532)
test bench::binary_heap_pop_bigpod   ... bench:  19,527,395 ns/iter (+/- 893,724)
test bench::binary_heap_push         ... bench:   1,590,254 ns/iter (+/- 144,186)
test bench::binary_heap_push_bigpod  ... bench:  11,129,994 ns/iter (+/- 583,714)
```

## Pairing Heap
```
test bench::pairing_heap_clone       ... bench:   1,936,221 ns/iter (+/- 252,472)
test bench::pairing_heap_pop         ... bench:  12,540,419 ns/iter (+/- 363,525)
test bench::pairing_heap_pop_bigpod  ... bench:  26,912,984 ns/iter (+/- 398,958)
test bench::pairing_heap_push        ... bench:   2,889,341 ns/iter (+/- 104,221)
test bench::pairing_heap_push_bigpod ... bench:  11,301,338 ns/iter (+/- 166,789)

```
