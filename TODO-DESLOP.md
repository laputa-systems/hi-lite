Prefix: DSLP
Next: 12

Deslop campaign backlog ([plan](https://github.com/laputa-systems/stuff/blob/main/docs/campaigns/deslop.md), tier A). One agent works this file with the `deslop` skill: take the lowest-numbered open ticket whose area does not start with `Needs owner ·`. After DSLP-5, work the open P1 and P2 findings, then fill in `## Report` below.

## Report

DSLP-1 Ground base: `6e96bc8b353a95994712ae3ea55d449e0435543f` (an ancestor of the reviewed HEAD `9a5eb2bb6510132d5898496cdcb08fbeb16b3097`). The completion review covered the cumulative diff and surrounding callers; it demonstrated no new defect and filed no new finding.

**Scorecard**

| Measure | Before | After / disposition |
| --- | --- | --- |
| Rust source and test lines | 7,024 `src/`; 788 `tests/` | 7,133 `src/`; 1,088 `tests/` (physical lines, including comments and blanks) |
| Warm `make test` | 1.18 s, macOS ARM64, two Cargo jobs | Not measured on a quiet host; current check is correctness only. Cold Ground: 9.57 s; lint: 5.39 s. |
| Line / branch coverage | 3,950/4,165 / 1,164/1,528 | 3,987/4,165 / 1,185/1,528 in the bracket/display slice (`62b26cf` to `0587ac8`), not a whole-campaign comparison. Later coverage unmeasured. |
| Escape hatches | 19 in `src/`, 14 in `tests/` | 19 and 13, respectively, counting Rust matches for `unsafe`, allow attributes, `unwrap()` and `expect()`. The removed test suppression belonged to dead code. |
| Dead code / dependencies | Unused fixture collector | 8 test lines and one import removed; zero dependencies removed. Public API decision remains DSLP-8. |
| Findings | Campaign filed P1: 0, P2: 1 (DSLP-7), P3: 2 (DSLP-6, 9), P4: 2 (DSLP-8, 10); excludes step tickets | Fixed P2: 1 and P3: 1; open P3: 1 and P4: 2. Two demonstrated defects fixed: out-of-range bracket access and maximum diff-context overflow; the latter was discovered in Tests rather than filed. |
| Performance | Ground identified highlight and diff workloads | **Perf gate deferred: host busy.** PR #26 remains parked pending a quiet-host YAML timing gate; its fix is not in HEAD. Previously accepted XSH and diff comparisons are linked below and do not establish a campaign-wide verdict. |
| Escaped regressions | Not measured | No new regression demonstrated by this review; not proof of zero escapes. The YAML doubled-quote defect was found later and remains in parked PR #26. |

**Required mutation scopes** (caught / missed / timeout / unviable; outcomes are diagnostic, not a success score). Runs used cargo-mutants 27.1.0, serial on macOS ARM64, `--gitignore true --baseline run --timeout 10 --build-timeout 300 --jobs 1`, one Cargo job and incremental compilation disabled. Historical runner-minutes were not recorded; the Tests closure added 0 runner-minutes. Wider lexer, other configuration scanners, broad sweeps and a whole-change lane are **advisory** and unrun or incomplete; no runner-minutes are recorded for them.

| Required scope | Before | After | Disposition |
| --- | --- | --- | --- |
| Bracket/display, 80 identities | 50 / 26 / 3 / 1 | 73 / 2 / 4 / 1 | A later UTF-8 past-end test caught one miss; the other is equivalent for four in-bounds callers. Four cursor-progress timeouts also timed out at 45 s. |
| Diff, 148 identities | 116 / 26 / 2 / 4 | 137 / 5 / 2 / 4 | Candidate plus focused rerun; four misses affect capacity only, one is equivalent LCS comparison; two hangs confirmed at 45 s. |
| Language detection, 30 identities | Not recorded | 25 / 0 / 0 / 5 | Compiler rejected the five unviable changes. Six independent injected faults failed the public tests. |
| Rust tokens, 54 identities | 42 / 7 / 5 / 0 | 47 / 2 / 5 / 0 | Both misses are output-equivalent EOF iterations; five cursor-progress timeouts remain incomplete. |
| JSON tokens, 74 identities | 44 / 24 / 6 / 0 | 64 / 2 / 8 / 0 | Both misses are equivalent increments; two new cursor timeouts also timed out at 45 s, others remain incomplete. |

Seven behavior groups are protected: bracket limits in both directions, mixed-width columns and reused display buffers, diff alignment and scratch reuse, preview context bounds, extension and shebang detection, and Rust and JSON token boundaries and lexer state. Timeouts are not catches; unviable mutations are compiler rejections. There are no accepted-risk exclusions in these required survivor groups.

**Steps and costs**

- Ground (DSLP-1, #13): mapped public boundaries, editor consumer and benchmark workloads; enabled Make tests in CI. Timings above; CI minutes not recorded.
- Diagnose (DSLP-2, #14–16): found the unresolved install example, invalid bracket start and unpinned XSH vocabulary. The bracket fix has empty, out-of-range and valid-pair regression cases. Editor candidate and locked dependency both had 149 passing and 14 failing end-to-end tests; a later shorter path resolved three shared failures. Wall time and CI minutes not recorded.
- Types (DSLP-3, #17): made the private character offset concrete, with a compiler witness. No strict candidate met adoption criteria: seven truncation, one sign-loss and two wildcard warnings were reviewed; other candidates had zero locations. Wall time and CI minutes not recorded.
- Tests (DSLP-4, #18, #20–25, #27): protected the behaviors above and fixed maximum-context overflow with a regression. An exhaustive small-document diff comparison produced 19,845 matching records. Coverage and mutation outcomes are above; total wall time and historical CI minutes not recorded. XSH's opt-in pinned-revision check passed in its original run; the editor's 17 display tests passed against both candidate and locked crate.
- XSH finding (DSLP-7, #19): pinned registry revision `c83b870d1b4e55871a319e1567cd22815986c4b1` and regenerated vocabulary. Five interleaved pairs passed host checks in [ARM64 run 44377931502](https://github.com/laputa-systems/hi-lite/actions/runs/44377931502); allocation counts matched, executable grew 8,320 bytes. Diff changes passed five paired comparisons in [run 44386818140](https://github.com/laputa-systems/hi-lite/actions/runs/44386818140), with matching allocations and 24-byte executable growth. CI minutes not recorded.
- Dead code (DSLP-5, #28): rustc and cargo-machete found no unsuppressed dead code or unused dependency. Removed the uncalled test collector after checking source, tests, examples, benches and tools; retained exported APIs and generator entry points. No runtime or consumer change. Wall time and CI minutes not recorded.
- Bench (DSLP-11, #29): added Edit/Change/Scenario/Full selection and proved Edit and a Rust Change workload. Fixed small snippets and diff fixtures do not establish editor-scale latency, tail latency or memory growth; repeated XOR checksums cancel. DSLP-9 and DSLP-10 retain the ranked workload and attribution gaps. Wall time and CI minutes not recorded.
- Report: reviewed cumulative diff; no demonstrated new finding. No mutation trials in this step; wall time and CI minutes not recorded.

**Method feedback:** Require a non-cancelling, checked completed-work witness for benchmarks; the existing 512-fold XOR cancels and could hide missing work. Mutation timeouts need separate confirmation and cannot be counted as catches. Strict-profile candidates found no adoptable lint; `unreachable_pub` produced advisory visibility warnings for this library rather than proving dead code.

**Shared-file gaps:** The shared deslop guidance should retain the distinction between required and advisory mutation scopes and require runner-minute recording at each trial; historical cost cannot be reconstructed here. The benchmark guidance should require a checked completed-work witness and a quiet-host verdict. No shared file was changed.

## Tickets





### DSLP-9 · P3 · Benchmark representative editor work with correctness checks

- **Area:** Bench · workloads
- **Where:** benches/hi_lite.rs
- **Problem:** Small repeated snippets and two diff fixtures do not establish latency or memory scaling for realistic editor documents, and timed checksums cancel over the 512 repetitions and are not checked against expected output.
- **Recommendation:** Define representative document and diff sizes with correctness and completed-work checks, then measure scaling.
- **Fix with:** none
- **Effort:** M

### DSLP-8 · P4 · Review public APIs absent from the known editor consumer

- **Area:** Needs owner · public API
- **Where:** src/lib.rs:11, src/highlight.rs:2548, src/diff.rs:9, src/language.rs:52
- **Problem:** The e checkout uses streaming highlighting, conversion into a buffer, detection, delimiters and XSH scanning, but has no direct call sites for the diff API, allocating byte conversion, stateless highlighting, runs, Language::from_name or Language::detect, and several Highlighter convenience methods; other consumers are unknown and these remain supported public contracts.
- **Recommendation:** Decide whether to retain these APIs for other consumers before considering any contract change; check downstream usage beyond e.
- **Fix with:** none
- **Effort:** M


### DSLP-10 · P4 · Make benchmark regressions attributable and comparable

- **Area:** Bench · measurement
- **Where:** docs/benchmarks.md
- **Problem:** The suite has no accepted baseline or budget, host validity gate, interleaved comparison, CPU profile or retained-memory attribution; allocation traffic alone cannot explain every regression.
- **Recommendation:** Define a comparison protocol and budget for representative workloads, and document low-overhead CPU and heap profiling separately from timing.
- **Fix with:** none
- **Effort:** M
