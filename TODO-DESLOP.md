Prefix: DSLP
Next: 8

Deslop campaign backlog ([plan](https://github.com/laputa-systems/stuff/blob/main/docs/campaigns/deslop.md), tier A). One agent works this file with the `deslop` skill: take the lowest-numbered open ticket whose area does not start with `Needs owner ·`. After DSLP-5, work the open P1 and P2 findings, then fill in `## Report` below.

## Report

Ground (DSLP-1) passed at base `6e96bc8b353a95994712ae3ea55d449e0435543f`. [Verification](AGENTS.md#verification) records the checked public contracts, the editor consumer, the existing benchmark workloads, and the separately enabled XSH registry check.


The first Diagnose slice at `cf23771` traced byte-indexed highlighting,
character-position delimiter queries and borrowed diff previews from the
library exports to their in-repo tests and the editor's call sites. The
editor locks `laputa-hi-lite` 0.4.0 through `stuff-native`. The registry's
native index advertised 0.4.0 and its crates.io replacement advertised
0.2.0, so the prior 0.3 dependency example could not resolve through either
path. The example now uses the verified native registry.

A disposable executable linked the built dependency-free crate and called
`find_bracket_match` with `line_count == 0`, position `(0, 0)`, and a callback
that indexes the supplied line slice. The callback received index 0 and
panicked before the search could return `None` (exit 101). The editor usually
supplies an in-range cursor, so this is a public API edge case rather than
a demonstrated editor crash. The remaining lexer and fixture tooling review
stay open under DSLP-2.

The bracket regression now checks an empty document, an out-of-range line
in a nonempty document, and a valid matching pair. `make test lint` passes.
The editor at `b29c90178b664e12d3c4f7f0f4f49772aee21c33` compiled
against the local candidate through Cargo's `patch.stuff-native` override.
Both the candidate and the original locked dependency produced the same
149 passing and 14 failing end-to-end tests in the macOS checkout. The
failures include status-bar truncation with the long fixture path and a
terminal-resize timeout. The comparison leaves those editor tests open.

Diagnose ranks the byte-to-character and lexer state boundaries first.
The public and golden fixtures cover mixed-width UTF-8, line-state
reuse and family-specific token kinds. The quote matcher asks its
caller for one line. The editor supplies that line from its cursor
path. The private conversion helper always returns a byte offset;
its unused optional result is a candidate for Types.

The opt-in XSH registry comparison fails with the clean XSH checkout
at `c83b870d1b4e55871a319e1567cd22815986c4b1`. A scratch regeneration
adds language tokens such as `assert`, `const` and `try`, and updates
type names. It adds 76 lines and removes five, including the
machine-specific source header. The test runs with
`XSH_REPO=~/repos/laputa-systems/xsh cargo test --locked --test xsh_lang -- --ignored`.
The tracked vocabulary records a source directory but no registry
revision. DSLP-7 tracks fixing the supported revision and updating
the generated vocabulary.

Types measured the library with its pinned Rust toolchain on macOS
ARM64. Clippy reported seven possible truncations, one possible sign
loss and two wildcard enum arms on both revisions. The casts cover
bounded diff rows, raw-string hash counts, static delimiter indexes
and a nonnegative reverse scan cursor. The wildcard arms implement
shared lexer dispatch. Unsafe code, unwrap, expect, redundant clone,
needless pass-by-value and the single-variant wildcard lint each
reported zero locations. None of the candidate lints met the adoption
threshold.

The bracket and quote matchers now receive a concrete offset from
`character_column_to_byte`. A scratch return of `None` failed with
E0308 at that helper. The normal library check passed after restoring
the branch, and all 134 ordinary tests and Clippy passed. The previous
macOS scheduling cancellation affected both the documentation PR and
its default base. The retried [macOS job](https://github.com/laputa-systems/hi-lite/actions/runs/44358336521/job/135004491711)
passed.

The first Tests scope is bracket search and byte-to-display mapping on
macOS ARM64 at base `62b26cf`. With cargo-llvm-cov 0.9.1, `make coverage`
passed 136 tests with the external XSH check ignored. Line coverage moved
from 3,950 to 3,987 of 4,165 and branches from 1,164 to 1,185 of 1,528.
The public tests check character offsets in both directions at the scan limit,
columns past the end of a UTF-8 line, and the exact kinds emitted for
expanded tabs and multi-byte characters when the output buffer is reused.
Changing the limit to 999 and changing the second kind of a multi-byte
line's tab to Normal each failed its corresponding test.

Serial cargo-mutants 27.1.0 selected the same 80 identities with
`--file src/highlight.rs --re 'src/highlight.rs:(23[3-9][0-9]|240[0-7]|24[7-9][0-9]|25[0-3][0-9]):'`,
`--gitignore true --baseline run --timeout 10 --build-timeout 300 --jobs 1`,
and `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1`. The baseline caught 50,
missed 26, timed out on three and could not build one because
`TextPosition` has no `Default`. The candidate caught 73 and missed two,
with four timeouts and the same unviable mutation. The added UTF-8
past-end case caught one of the two misses. The other is equivalent:
all four callers pass an in-bounds byte index, so the companion
`bi < byte_idx` guard prevents reaching `line.len()`. All four
mutations that stall offset progress also timed out at 45 seconds.
The lexer, diff and remaining public boundaries stay in DSLP-4.

The XSH vocabulary is pinned to registry commit
`c83b870d1b4e55871a319e1567cd22815986c4b1`. The opt-in check regenerated
the complete block from a bare Git repository and matched every byte,
including the revision header. All 136 ordinary tests and Clippy passed.
The editor `e` at `b29c901` passed all 17 display tests with its locked
crate and with the candidate. A shorter scratch path resolved three
shared status-bar truncation failures from the first editor attempt.

In the [native ARM64 timing run](https://github.com/laputa-systems/hi-lite/actions/runs/44377931502),
the release benchmarks compared `0587ac8` with the candidate source at
`f401c91`. Five interleaved pairs passed the pre- and post-sample load
checks. The vocabulary expansion initially slowed the four workloads.
Returning on the first matching word preserves the lookup precedence
and reduces comparisons for matching tokens.

| Workload | Baseline median and range (ms) | Candidate median and range (ms) |
| --- | ---: | ---: |
| All snippets, warm | 52.159 (52.141–52.165) | 51.062 (51.057–51.142) |
| All snippets, cold | 53.460 (53.439–53.495) | 52.504 (52.471–52.538) |
| XSH snippet, warm | 0.4434 (0.4417–0.4477) | 0.4280 (0.4262–0.4414) |
| XSH snippet, cold | 0.4580 (0.4552–0.4613) | 0.4472 (0.4434–0.4538) |

Allocation counts, allocated bytes and maximum live allocations matched.
The benchmark executable grew from 1,573,200 to 1,581,520 bytes. The
XSH-warm median process RSS snapshot grew from 2,715,648 to 2,785,280
bytes. Repeat with Rustybench 0.5, `hi_lite`, the same four snippet
filters and `--bench --format json --sample-count 60 --min-time 0.5
--max-time 1.5` on a quiet native ARM64 host.


**Diff behavior tests**

Serial cargo-mutants 27.1.0 selected all 148 `src/diff.rs` identities
at base `5a38860`. The baseline caught 116, missed 26, timed out on two
and could not build four. Use `--file src/diff.rs --gitignore true
--baseline run --timeout 10 --build-timeout 300 --jobs 1` on macOS
ARM64 with `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1`. The first
candidate caught 131 and missed 11 with the same other outcomes.
The final two cases caught six of those 11 in a focused rerun.
The measured scope now accounts for 137 caught, five missed, two
confirmed hangs and four compiler rejections.

The cases exercise exact numbered rows with repeated lines, both
exhausted-side paths, the final line without a newline, a source row
matching the omission text, and a reused preview at the zero-row
limit. Repeating the cursor-update mutations at lines 187 and 195
with a 45-second timeout confirmed both hangs. The four compiler
rejections are `E0277` because diff row types have no `Default`.
The four allocation-capacity mutations change allocation behavior
with the same diff rows. They retain their performance disposition
in DSLP-4. At a mismatched LCS cell, comparing the left neighbor to
`max(left, up)` is equivalent to comparing it to `up`. That explains
the remaining backtracking mutation at line 179.

**Full-range diff preview**

`DiffScratch::unified_preview_into` panicked on `usize::MAX` at
`5a38860`. The boundary test calls the public method and checks all
available preview rows in order. Bound the context to the input length
before the row loop. The alignment table now clears only the top row
and first column before using them. The recurrence overwrites the
interior cells in order. Removing either clear breaks the corresponding
scratch-reuse case when the row stride changes.

[Native ARM64 run 44386818140](https://github.com/laputa-systems/hi-lite/actions/runs/44386818140)
compared `5a38860` with `884618d` on the same Linux runner using
five interleaved pairs and the pinned Rust compiler. Cold diff fixture
medians were 1.020325 ms and 1.010567 ms. Warm medians were 0.653125 ms
and 0.644558 ms. The before and after ranges did not overlap. Allocation
counts, bytes and maximum live allocations matched. The compiled benchmark
was 24 bytes larger and the median process RSS was within 8 KiB. The job
used the identical Cargo lockfile on both sides and fetched its checksummed
public dependencies directly while the mirror recovered. The temporary
workflow at `884618d` records the benchmark command and readiness checks.

A local differential check reused the same scratch storage across all
3,969 pairs of binary-alphabet documents containing zero to five lines.
The complete diff and four preview settings per pair produced 19,845
matching records. Their SHA-256 was
`75d8115eb782b9a25111a1d4ae3e0c98e05f6f96a1fbc629fee9a549173e95ac`.

**Language detection**

The public cases cover CRLF, tab-separated `env -S` arguments,
a multi-part interpreter version, and a non-UTF-8 path prefix.
Unknown interpreters and a missing `env` command stay unresolved.
The extension cases preserve `.m` for Objective-C and `.M` for
Objective-C++.

All 18 API tests passed. At `e91982b`, six independent faults in
`src/language.rs` failed the two new cases. They remove ASCII line
trimming, remove the tab separator, read the flag as the interpreter,
split the version at its last dot, compare the special `M` without
case, or keep the leading extension dot. Run
`cargo test --locked --test api` with each fault in a separate copy.

At `f73b2a7d7b25f02c34b7c17ec484949fff6a7242`, the complete
`src/language.rs` selection accounted for 30 identities on macOS
ARM64. Tests caught 25. Five failed to build with E0277 because
`Language` and `RuleSet` do not implement `Default`. There were
no survivors or timeouts. Reproduce with the same serial
cargo-mutants 27.1.0 command as the diff census, replacing the
source selection with `--file src/language.rs`.

**Rust token boundaries**

On the public line highlighter, nested comment closers resume code
coloring, and the underscore lifetime and character literal have
distinct kinds. A raw byte prefix requires adjacent letters. A raw
string with 255 hashes keeps its state across a shorter delimiter
and closes at the full delimiter. The 256-hash opener does not enter
raw string state.

At `75eed701595361ae6fb7ff794c76bea833fd8474`, comparing identical
production sources across the two runs caught five additional
mutations. The 54 identities moved from 42 caught, seven survivors
and five timeouts to 47 caught, two survivors and the same five
timeouts. Both survivors add one iteration at EOF. Neither scanner
can match a nonempty delimiter at EOF, and both return the same
highlight range, end position and state. The five timeouts involve
changes to cursor advancement. Their timeout outcomes are separate
from test catches.

Reproduce with the earlier serial diff command and the environment
`RUST_TEST_THREADS=1 CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0`.
Use `--file src/highlight.rs` and
`--re ' in (rust_raw_string_open|rust_raw_string_close|scan_rust_raw_string|rust_lifetime_end|scan_rust_block_comment)$'`.
Compare the names and outcomes in `mutants.out/outcomes.json`.

## Tickets



### DSLP-4 · P2 · Tests: coverage, boundary gaps, mutation

- **Area:** deslop · tests
- **Where:** test suites and CI
- **Problem:** `make coverage` and the display and bracket trial cover the first Tests scope. The wider lexer and diff mutation inventory and the whole-change lane remain open.
- **Recommendation:** Run the deslop Tests step. Accept: no undispositioned survivor in critical code, the test cost budget holds, and the lane is green where the tools are mirrored.
- **Fix with:** deslop
- **Effort:** M

### DSLP-5 · P2 · Dead code: tools and debloat

- **Area:** deslop · dead code
- **Where:** whole repository
- **Problem:** No dead-code scan has run.
- **Recommendation:** Run the deslop Dead code step. Accept: tools clean or allowlisted with reasons; walk-backs filed as Needs owner tickets.
- **Fix with:** deslop
- **Effort:** M
