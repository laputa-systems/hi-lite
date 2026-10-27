# Benchmarks

Run the benchmark target with:

```sh
cargo bench --bench hi_lite
```

`benches/hi_lite.rs` reads the checked-in `.snippet` and diff inputs directly;
it does not read the `.golden` files. It reports warm and cold workloads for
the full snippet set, the first non-empty line of each snippet, each individual
snippet, and the unified diff fixtures.

Warm workloads reuse highlighters and scratch/output buffers. Cold workloads
include their setup in the measured work, so they also show the cost of callers
that do not retain those buffers.
