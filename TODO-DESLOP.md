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

## Tickets



### DSLP-4 · P2 · Tests: coverage, boundary gaps, mutation

- **Area:** deslop · tests
- **Where:** test suites and CI
- **Problem:** Coverage and mutation results are unmeasured; no deslop mutation lane exists.
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

### DSLP-7 · P2 · Pin the supported XSH vocabulary revision and regenerate it

- **Area:** XSH vocabulary and fixtures
- **Where:** src/languages/xsh.rs; examples/gen_xsh.rs; tests/xsh_lang.rs
- **Problem:** The opt-in vocabulary test fails against clean XSH c83b870d1b4e55871a319e1567cd22815986c4b1. The checked-in vocabulary predates several keywords and types, and the source header gives a local path without a revision.
- **Recommendation:** Select and record the XSH revision the library supports, regenerate the vocabulary from that revision, and rerun the opt-in test, normal goldens and editor consumer. Qualify the highlighting workload for the generated table change.
- **Fix with:** test-audit
- **Effort:** M
