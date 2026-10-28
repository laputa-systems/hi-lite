Prefix: HIL
Next: 9

### HIL-6 · P3 · A checkpoint from another language panics

- **Area:** Types
- **Where:** `src/highlight.rs:347`, `src/languages/generic.rs:207`, `src/highlight.rs:2610`
- **Problem:** `State` is opaque but not tied to a language. `MultiLineString(index)` indexes `string_delims` with no bounds check. Probe: a Python `"""` checkpoint passed to a LaTeX `Highlighter` through `set_state` panics at `highlight.rs:347` (`index out of bounds: the len is 0 but the index is 0`). LaTeX uses `TEX_RULES`, whose `string_delims` is empty. `set_language` resets state. `set_state` does not.
- **Recommendation:** Carry the producing language inside `State`, and do not index `string_delims` when the index is out of range. Ignore a checkpoint whose language does not match the highlighter.
- **Fix with:** make-invalid-states-unrepresentable
- **Effort:** S

### HIL-7 · P3 · Family-language goldens do not pin a grammar

- **Area:** Tests
- **Where:** `tests/golden.rs:23`, `tests/fixtures/language_probe.snippet:1`, `tools/generate-hi-lite-goldens.sh:21`
- **Problem:** C++, C#, SCSS, Batch, OCaml, SQL, Regex, and the other family variants all compare `language_probe.snippet`, which is the two lines `42` and `"Ada"`. The generator asks syntect for a C token for most of them. A wrong keyword list, a wrong comment delimiter, or HIL-1, HIL-3, and HIL-5 do not fail `make test`. The bespoke snippet languages (Rust, Python, Go, and the others in the first half of `FIXTURES`) do have real goldens.
- **Recommendation:** Replace the shared probe goldens with one short snippet per family that contains a keyword and a comment the lexer claims to support. Delete the duplicate goldens that only repeat the probe.
- **Fix with:** renovate-tests
- **Effort:** M

### HIL-8 · P3 · `find_quote_match` ignores `line_count`

- **Area:** Architecture
- **Where:** `src/highlight.rs:2396`
- **Problem:** The signature matches `find_bracket_match`, which scans other lines, but `_line_count` is unused. Quotes pair only on the current line, in source order, and an unmatched quote returns `None`. A caller that passes a document line count still gets a same-line result.
- **Recommendation:** Remove `line_count` in a breaking cleanup, or document a multiline search and implement it. Do not leave a parameter that implies the bracket search.
- **Fix with:** api-surface-and-module-boundaries
- **Effort:** S
