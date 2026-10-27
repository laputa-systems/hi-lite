Prefix: HIL
Next: 9

### HIL-1 · P2 · CSS block comments do not cross lines

- **Area:** Types
- **Where:** `src/highlight.rs:641`, `src/language.rs:493`, `src/languages/css.rs:11`
- **Problem:** The CSS post-pass replaces the code scanner's returned state with `CssBlock` or `Normal`. An unclosed `/*` is painted as a comment on that line, then the next line is highlighted as code. SCSS uses `CSS_RULES`, so it has the same drop. Probe: `Highlighter` on `/* open` then `color: red;` leaves `state().is_normal()` true and does not mark the second line as `Comment`. `still */` is not a comment either.
- **Recommendation:** Keep `BlockComment` and `MultiLineString` from the code scanner. Use `CssBlock` only when that state is `Normal`.
- **Fix with:** none
- **Effort:** S

### HIL-2 · P2 · Rust identifiers are classified as XSH macros

- **Area:** Types
- **Where:** `src/languages/rust.rs:26`, `src/languages/xsh.rs:149`, `src/highlight.rs:1402`
- **Problem:** `RUST_RULES.macros` is a subset of `XSH_MACROS` (`env`, `fs`, `io`, `path`, `test`, `print`, and others). `identifier_kind` checks that list before the `ident!` rule, so those words are `Kind::Macro` even without `!`. Probe: `let env = 1;` marks `env` as `Macro`, and `fn test() {}` marks `test` as `Macro`. Ordinary Rust files hit this on every `test` module and `std::env` path.
- **Recommendation:** Remove the copied list. `highlight_bang_macros` already marks `ident!`. Add a fixture line that would fail if `env` or `test` is `Macro` again.
- **Fix with:** single-source-of-truth
- **Effort:** S

### HIL-3 · P2 · `Language::comment` and the lexer disagree

- **Area:** Config
- **Where:** `src/language.rs:441`, `src/language.rs:486`, `src/language.rs:491`, `src/language.rs:493`, `src/language.rs:496`, `src/languages/css.rs:11`
- **Problem:** `comment()` is a separate match from `rules()`, and four public languages disagree with the lexer that actually runs. Probe: SCSS `comment()` is `//` but `// line comment` is `Normal` (`CSS_RULES.line_comment` is empty). OCaml `comment()` is `//` (the default arm) but `-- dash` is `Comment` via `DASH_RULES`. Batch `comment()` is `#` but `# hash` is `Normal` and `// slash` is `Comment` via `C_LIKE_RULES`. Regex `comment()` is empty but `// slash` is `Comment` via `C_LIKE_RULES`. Editor comment toggling and highlighting cannot both be right.
- **Recommendation:** See the RFC. One field should own the delimiter the editor inserts and the delimiter the lexer honors.
- **Fix with:** single-source-of-truth
- **Effort:** M
- **Design:** [docs/rfc/HIL-3-comment-delimiters.md](docs/rfc/HIL-3-comment-delimiters.md)

### HIL-4 · P2 · Complete diffs allocate a dense LCS table

- **Area:** Performance
- **Where:** `src/diff.rs:122`, `src/diff.rs:61`
- **Problem:** `diff_into` resizes `table` to `(old_lines + 1) * (new_lines + 1)` usizes with no line cap. `DEFAULT_MAX_DISPLAY_LINES` (6) applies only while projecting a preview, after the table exists. Two 5,000-line buffers are 25,000,000 cells (200 MB for the table alone). Two 20,000-line buffers are about 3.2 GB. In debug, a wrapping multiply panics first. Probe: `diff` of 500 repeated lines plus one added line returned 501 rows, so this path has no early exit. Release OOM is the practical failure. The byte sizes are arithmetic from the formula, not a timed measurement.
- **Recommendation:** See the RFC. Keep the current tie-break for small inputs and refuse or elide inputs that would allocate the full table.
- **Fix with:** none
- **Effort:** M
- **Design:** [docs/rfc/HIL-4-diff-line-table.md](docs/rfc/HIL-4-diff-line-table.md)

### HIL-5 · P3 · SQL keywords match only lowercase

- **Area:** Types
- **Where:** `src/languages/generic.rs:156`, `src/highlight.rs:1392`, `src/language.rs:489`
- **Problem:** `SQL_RULES` lists lowercase keywords, and `keyword_search` compares raw bytes. Probe: `SELECT * FROM t;` leaves `SELECT` and `FROM` as `Normal`. The same line in lowercase marks both as `Keyword`. SQL source is usually uppercase. No fixture covers this. `language_probe.snippet` is only `42` and `"Ada"`.
- **Recommendation:** Case-fold the identifier before the SQL keyword and type search, and add one uppercase line to a SQL fixture.
- **Fix with:** none
- **Effort:** S

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
