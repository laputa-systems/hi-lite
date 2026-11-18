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

## Tickets


### DSLP-3 · P2 · Types: strict profile and typing findings

- **Area:** deslop · types
- **Where:** type checker and lint configuration; Rust sources
- **Problem:** The strict-profile candidates in the deslop skill's tools reference have not been measured here.
- **Recommendation:** Run the deslop Types step. Accept: clean at the adopted profile, escape hatches not increased, witnesses recorded.
- **Fix with:** deslop
- **Effort:** M

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
