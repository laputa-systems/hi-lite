Prefix: HIL
Next: 12
### HIL-9 · P3 · Benchmark representative editor work with correctness checks

- **Area:** Bench · workloads
- **Where:** benches/hi_lite.rs
- **Problem:** Small repeated snippets and two diff fixtures do not establish latency or memory scaling for realistic editor documents, and timed checksums cancel over the 512 repetitions and are not checked against expected output.
- **Recommendation:** Define representative document and diff sizes with correctness and completed-work checks, then measure scaling.
- **Fix with:** none
- **Effort:** M

### HIL-10 · P4 · Review public APIs absent from the known editor consumer

- **Area:** Needs owner · public API
- **Where:** src/lib.rs:11, src/highlight.rs:2548, src/diff.rs:9, src/language.rs:52
- **Problem:** The e checkout uses streaming highlighting, conversion into a buffer, detection, delimiters and XSH scanning, but has no direct call sites for the diff API, allocating byte conversion, stateless highlighting, runs, Language::from_name or Language::detect, and several Highlighter convenience methods; other consumers are unknown and these remain supported public contracts.
- **Recommendation:** Decide whether to retain these APIs for other consumers before considering any contract change; check downstream usage beyond e.
- **Fix with:** none
- **Effort:** M

### HIL-11 · P4 · Make benchmark regressions attributable and comparable

- **Area:** Bench · measurement
- **Where:** docs/benchmarks.md
- **Problem:** The suite has no accepted baseline or budget, host validity gate, interleaved comparison, CPU profile or retained-memory attribution; allocation traffic alone cannot explain every regression.
- **Recommendation:** Define a comparison protocol and budget for representative workloads, and document low-overhead CPU and heap profiling separately from timing.
- **Fix with:** none
- **Effort:** M

