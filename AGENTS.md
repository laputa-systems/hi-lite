# hi-lite

Rust library for line-oriented syntax highlighting and renderer-neutral diff previews.

- Build: `cargo build --locked`
- Test: `make test`
- Check: `make lint`
- Benchmarks: `make bench`, `make bench-check`, `make bench-list` (see [procedure](docs/benchmarks.md)).
- Regenerate XSH vocabulary with `make gen-xsh`. Point `XSH_REPO` at a Git repository carrying `XSH_REVISION` in `examples/gen_xsh.rs`. The default is `~/d/laputa-systems/xsh`.
- Publish: `make publish`
- `Highlighter` returns one semantic `Kind` per input byte and retains lexical `State` across lines.
- Highlight kinds and `Run` offsets are byte-indexed.
- XSH string openers in `src/languages/xsh.rs` are longest-match first.

The roadmap, [docs/roadmap.md](docs/roadmap.md), holds all current and future work; its Status is the resume point.

## Verification

`make clean` removes the checkout’s ordinary `./target` directory. It refuses a file or symlink at that path.

- On macOS ARM64, `make test` passed 133 tests in 9.57 s with a fresh Cargo target and 1.18 s warm. `make lint` passed in 5.39 s. Both ran with two Cargo jobs.
- `tests/api.rs` covers state reuse, byte and character boundaries, runs, language aliases, extension and shebang detection, and public XSH scanning.
- `tests/golden.rs` compares syntax results with checked-in Syntect-derived fixtures. See [fixture provenance and regeneration](docs/testing-fixtures.md).
- `tests/diff.rs` covers unified diff fixture output. `src/diff.rs` tests the structured preview and diff bounds.
- `tests/xsh_lang.rs` compares the entire generated region with the pinned XSH registry revision. Set `XSH_REPO` and run `cargo test --locked --test xsh_lang -- --ignored`.
- The editor `e` consumes highlighting and diff previews. Its tests against the candidate crate are the downstream check for changes to those APIs.
- `cargo bench --bench hi_lite` measures warm and cold highlighting, per-snippet work, and diff fixtures. Use [the benchmark procedure](docs/benchmarks.md) for changes to those paths.
- This library exposes Rust APIs. It has no user CLI, network endpoint, or persisted data format.

## Docs

- Installation and public API: [README.md](README.md).
- Fixture formats and regeneration: [docs/testing-fixtures.md](docs/testing-fixtures.md).
- Benchmark workloads: [docs/benchmarks.md](docs/benchmarks.md).
- Open findings: [TODO.md](TODO.md).
- Comment delimiters: [docs/rfc/HIL-3-comment-delimiters.md](docs/rfc/HIL-3-comment-delimiters.md).
- Diff table bound: [docs/rfc/HIL-4-diff-line-table.md](docs/rfc/HIL-4-diff-line-table.md).
