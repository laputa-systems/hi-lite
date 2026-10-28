Prefix: HIL
Next: 9

### HIL-7 · P3 · Family-language goldens do not pin a grammar

- **Area:** Tests
- **Where:** `tests/golden.rs:23`, `tests/fixtures/language_probe.snippet:1`, `tools/generate-hi-lite-goldens.sh:21`
- **Problem:** C++, C#, SCSS, Batch, OCaml, SQL, Regex, and the other family variants all compare `language_probe.snippet`, which is the two lines `42` and `"Ada"`. The generator asks syntect for a C token for most of them. A wrong keyword list, a wrong comment delimiter, or HIL-1, HIL-3, and HIL-5 do not fail `make test`. The bespoke snippet languages (Rust, Python, Go, and the others in the first half of `FIXTURES`) do have real goldens.
- **Recommendation:** Replace the shared probe goldens with one short snippet per family that contains a keyword and a comment the lexer claims to support. Delete the duplicate goldens that only repeat the probe.
- **Fix with:** renovate-tests
- **Effort:** M
