# hi-lite

Rust library for line-oriented syntax highlighting and renderer-neutral diff previews.

- Build: `cargo build --locked`
- Test: `make test`
- Check: `make lint`
- `Highlighter` returns one semantic `Kind` per input byte and retains lexical `State` across lines.
- Highlight kinds and `Run` offsets are byte-indexed.

## Docs

- Installation and public API: [README.md](README.md).
- Fixture formats and regeneration: [docs/testing-fixtures.md](docs/testing-fixtures.md).
- Benchmark workloads: [docs/benchmarks.md](docs/benchmarks.md).
