# Benchmarks

`make bench-list` lists every workload, tier and area. `make bench-check`
checks the selected prerequisites without running anything. `make bench` defaults
to Edit; Full requires explicit `TIER=full` and is manual-only.

```sh
make bench TIER=edit
make bench TIER=change NAME=hi_lite_rust
make bench TIER=scenario AREA=diff DRY_RUN=1
make bench TIER=full SKIP=hi_lite_xsh_cold
```

`TIER` is a ceiling: edit, change, scenario or full. `NAME` and `SKIP`
accept exact workload names, and `AREA` accepts driver, highlight or diff;
space- or comma-separated values are allowed. Names and areas intersect.
Unknown values and empty selections fail before any workload runs. Dry runs
perform preflight and print commands without writing results. Preflight needs
Python 3, Cargo, the pinned Rust toolchain, checked-in fixtures and 512 MiB free
for benchmark builds. Cargo checks build freshness; fixtures are versioned with
this checkout. There are no external services or benchmark-owned background
processes. Host readiness is informational and not assessed by this driver.

Edit tests selection and preflight (estimated seconds). Change runs one Rust
snippet and one diff workload (estimated minutes including compilation).
Scenario adds corpus, single-line and cold diff workloads (estimated minutes).
Full adds all individual snippets (estimated tens of minutes). `NAME` selects
a smaller set at the applicable ceiling. Runs write per-workload logs and a
summary under `.work/bench/`; completion is not a performance verdict. The
driver requests 10 samples and 0.1–0.2 seconds from rustybench per workload;
compile and calibration may take longer. For comparisons use the pinned
revision, same command and fixtures, and interleaved baseline/candidate runs on
a quiet host with `perf-gate-ready` before and after each timing. Record host
conditions and treat busy-host observations as inconclusive.

The target embeds checked-in `.snippet` and diff inputs, not `.golden` files.
Warm workloads reuse highlighters and scratch/output buffers; cold means fresh
library objects and buffers, not a cold OS cache. Each closure processes 512
repetitions and consumes a checksum, but does not assert its correctness. The
ordinary golden and diff tests provide separate correctness checks. Rustybench
reports elapsed time, throughput and process allocation traffic; allocations
are not retained heap or peak RSS. CPU sampling (for example Instruments or
`perf`), and a heap profiler can help attribute regressions in separate,
instrumented runs. The separate `tools/hi-lite-syntect` crate is a comparison,
not part of this entrypoint, and requires matching semantics before comparison.
