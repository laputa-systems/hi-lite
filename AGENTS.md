# hi-lite

Rust library for line-oriented syntax highlighting and renderer-neutral diff previews.

- Build: `cargo build --locked`
- Test: `make test`
- Check: `make lint`
- Regenerate XSH vocabulary: `make gen-xsh` (`XSH_REPO`, otherwise `~/d/laputa-systems/xsh`). See `examples/gen_xsh.rs`.
- Publish: `make publish`
- `Highlighter` returns one semantic `Kind` per input byte and retains lexical `State` across lines.
- Highlight kinds and `Run` offsets are byte-indexed.
- XSH string openers in `src/languages/xsh.rs` are longest-match first.

## Docs

- Installation and public API: [README.md](README.md).
- Fixture formats and regeneration: [docs/testing-fixtures.md](docs/testing-fixtures.md).
- Benchmark workloads: [docs/benchmarks.md](docs/benchmarks.md).
- Open findings: [TODO.md](TODO.md).
- Comment delimiters: [docs/rfc/HIL-3-comment-delimiters.md](docs/rfc/HIL-3-comment-delimiters.md).
- Diff table bound: [docs/rfc/HIL-4-diff-line-table.md](docs/rfc/HIL-4-diff-line-table.md).
