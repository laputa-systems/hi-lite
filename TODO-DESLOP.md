Prefix: DSLP
Next: 7

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

## Tickets

### DSLP-2 · P2 · Diagnose: ultrareview into this backlog

- **Area:** deslop · diagnose
- **Where:** whole repository
- **Problem:** No review has ranked this repository's risks for the deslop pass.
- **Recommendation:** Run the deslop Diagnose step. Accept: the diff touches only docs, comments, and this file, and every finding names its Fix with.
- **Fix with:** deslop
- **Effort:** S

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


### DSLP-6 · P3 · Reject an out-of-range bracket starting line

- **Area:** delimiter matching
- **Where:** src/highlight.rs:2331; tests/api.rs
- **Problem:** A built-crate fixture with line_count 0 and start line 0 invoked an indexing callback and panicked before the API could return None. The callback only has lines in 0..line_count.
- **Recommendation:** Return None before calling the line loader when the start is outside line_count. Re-run the empty-slice fixture and exercise a valid text line with a match as a control.
- **Fix with:** test-audit
- **Effort:** S

