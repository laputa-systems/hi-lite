# hi-lite

`hi-lite` is a Rust library for line-oriented syntax highlighting. It has no
runtime dependencies and returns semantic `Kind` values; callers choose how to
render them.

## Install

Add the package to `Cargo.toml`:

```toml
[dependencies]
laputa-hi-lite = "0.3"
```

The package is named `laputa-hi-lite`; import its Rust library as `hi_lite`:

```rust
use hi_lite::{Highlighter, Kind, Language};

let mut highlighter = Highlighter::new(Language::Rust);
let mut kinds = Vec::new();
let result = highlighter.highlight_into(b"fn main() {}", &mut kinds);

assert_eq!(result[0], Kind::Keyword);
```

`highlight_into` returns one `Kind` per input byte and retains lexical state
between lines. Reuse the output `Vec<Kind>` to avoid reallocating it. Use
`state()` and `set_state()` to save and restore an opaque checkpoint, or
`highlight_line_into` for a stateless caller-sized buffer.

`runs` exposes contiguous byte ranges. For UTF-8 text, use
`byte_kinds_to_char_kinds_into` to map byte highlights to character positions;
tabs expand to two display entries.

## Languages

`Language::from_name` accepts the maintained language registry and common
aliases such as `rs`, `py`, `js`, `ts`, `sh`, `yml`, and `md`. Use
`Language::from_file_ext`, `Language::from_shebang`, and `Language::detect` to
select by extension, first-line shebang, or both. `detect` tries a recognized
extension first; selectors return `None` when no mode matches.

## Diff previews

`diff` returns borrowed line operations and marks trailing-newline changes.
It stores one LCS cell per pair of input lines, so large buffers need a lot of
memory. `diff_preview` and `unified_preview` produce renderer-neutral context,
addition, deletion, and elision rows. `DiffScratch` supports reusable storage.

## Help

Report problems on the project's
[GitHub Issues page](https://github.com/laputa-systems/hi-lite/issues).

## License

MIT; see [LICENSE](LICENSE).
