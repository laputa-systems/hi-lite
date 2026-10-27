# HIL-4: Bound the line-diff table

Status: draft

## Summary

`diff` and `DiffScratch::diff_into` allocate a dense LCS table, one `usize` per pair of input lines, before a preview throws most of those lines away. Cap that table. Keep today's tie-break for inputs under the cap.

## Motivation

`diff_into` sets `cells` to `(old_lines + 1) * (new_lines + 1)` and resizes `table` to that length (`src/diff.rs:122`). Nothing checks the line count first. `unified_preview_into` then keeps `DEFAULT_MAX_DISPLAY_LINES` source rows (6) plus elision. The expensive work is already done.

The backtrack prefers an add over a delete when the two LCS scores tie (`src/diff.rs:164`). The module documents that tie-break as the one `fx` depends on. A different algorithm that changes equal-score hunks will change previews callers already goldened (`tests/diff.rs` and `tests/fixtures/diff-*.golden`).

A 500-line probe completes and returns one row per line plus the extra line. That only shows the absence of a cap. The sizes below are the formula, not a benchmark: 5,000 by 5,000 cells is 200 MB, and 20,000 by 20,000 cells is about 3.2 GB, before the line index and the output.

## Design

Count lines before allocating `table`. When either side is over the cap, do not build the matrix. `diff` returns one elision-sized result the preview can show, and `diff_preview` returns a single elision row. Inputs at or under the cap keep the current matrix and the current `>=` tie-break, so existing fixtures stay valid.

Apply the same cap in `diff`, `diff_preview`, and `DiffScratch`. A scratch value must not be a way around it.

## Alternatives considered

Switch the whole diff to a linear-space Myers implementation. That removes the dense table, including for small inputs, but it has to reproduce the current tie-break or the goldens move. Do that only if the cap is too small for real callers.

Document the cost and leave the API. The README now states the cost. Callers can still pass two buffers and exhaust the process. A comment is not a bound.

Truncate each side to the cap and diff the prefix. That still presents a prefix as a complete diff, which is a silent wrong result. An explicit elision is easier to render.

## Open questions

- What is the cap? Recommendation: 2,048 lines per side. A six-row preview never needs a multi-thousand-line matrix, and 2,049 by 2,049 usizes is about 32 MB, which is still large but no longer a process killer.
- Should over-cap `diff` return `Result` instead of an elision row? Recommendation: no. The rest of this API does not use `Result`, and a single elision row is something the existing preview types can already carry.

## Decision

Unset.
