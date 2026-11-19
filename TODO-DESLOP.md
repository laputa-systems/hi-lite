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

**Tests: required scope and dispositions**

The required scope is the public bracket scan limit and UTF-8 column/display
mapping (`src/highlight.rs`); diff alignment, preview context bounds and scratch
reuse (`src/diff.rs`); extension and shebang detection (`src/language.rs`);
and Rust and JSON token boundaries (`src/highlight.rs`). Ordinary API and diff
tests protect the scan limit in both directions, mixed-width columns and reused
display buffers, numbered diff rows and changing scratch strides, `usize::MAX`
context, detection edge cases, and token kinds and lexer state across boundaries.
Golden fixtures exercise additional languages. The original maximum-context
panic was fixed and its regression retained at `884618d`. Removing either
alignment-table boundary clear failed the scratch-reuse cases; changing the
scan limit to 999 or a tab display kind failed the corresponding API test.

Recorded serial cargo-mutants 27.1.0 results on macOS ARM64 used `--gitignore
true --baseline run --timeout 10 --build-timeout 300 --jobs 1` with one Cargo
job and incremental compilation disabled. At `62b26cf`, bracket/display
selected 80: 73 caught, two missed, four timeouts and one unviable; a later
UTF-8 past-end test caught one miss. The remaining guard is redundant for all
four in-bounds callers. The four cursor-progress mutations also timed out at
45 seconds. At `5a38860`, diff selected 148: 137 caught across the candidate
and focused rerun, five missed, two confirmed 45-second hangs and four E0277
unviable. Four misses only alter allocation capacity; the fifth is an equivalent
LCS comparison. At `f73b2a7`, language detection selected 30: 25 caught, five
E0277 unviable, no misses or timeouts. Six hand-injected detection faults failed
at `e91982b`. At `75eed701`, Rust token scans selected 54: 47 caught, two
output-equivalent EOF iterations and five cursor-advance timeouts. JSON's 74
identities yielded 64 caught, two equivalent number-branch increments and eight
timeouts; two newly timed-out cursor updates also timed out at 45 seconds.
Unviable mutations are compiler rejections, not evidence of detection. Timeouts
without a recorded longer confirmation remain incomplete, not catches. There
are no accepted-risk exclusions in the required survivor groups.

The wider lexer and other configuration scanners, broad module sweeps and
whole-change mutation lane are advisory and unrun or incomplete. The opt-in XSH
registry comparison requires its pinned external repository; editor end-to-end
checks have shared environment-dependent failures recorded above. Neither is
claimed as a local pass here. No new mutation trial was run for this closure:
actual new trial cost is 0 runner-minutes. Historical trial durations were not
recorded. No new regression test was needed for the dispositioned survivors.

## Tickets




### DSLP-5 · P2 · Dead code: tools and debloat

- **Area:** deslop · dead code
- **Where:** whole repository
- **Problem:** No dead-code scan has run.
- **Recommendation:** Run the deslop Dead code step. Accept: tools clean or allowlisted with reasons; walk-backs filed as Needs owner tickets.
- **Fix with:** deslop
- **Effort:** M
