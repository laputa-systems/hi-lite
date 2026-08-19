# hi-lite syntax goldens

`tests/fixtures/*.snippet` contains substantial, syntax-complete examples for
the specialized and representative popular languages. Their `.golden` files are
normalized semantic runs, not ANSI output. The normalization maps syntect
scope stacks to hi-lite's eleven `Kind` values and intentionally keeps a string
parent dominant over template interpolation.

The specialized corpus is complemented by `tests/fixtures/language_probe.snippet`.
Its checked-in goldens cover every remaining canonical `Language` variant,
including the generic rule families; `xsh.snippet` has an
explicit hi-lite-maintained golden because XSH is not part of syntect's bundle.
The probe intentionally checks shared number/string behavior rather than
pretending that every generic family has a distinct embedded grammar.

To regenerate the syntect-backed fixtures and registry probe goldens, run:

```sh
tools/generate-hi-lite-goldens.sh
```

The reusable oracle lives in `tools/hi-lite-syntect/`; it is deliberately a
standalone Cargo project and is not a workspace member. Syntect's default
bundle does not ship Dockerfile, INI, or TOML grammars, so those three checked-in
goldens are maintained as explicit semantic fixtures and are not overwritten by
the generator. Syntect is therefore not added to the crate manifest, and
ordinary `cargo test` uses only the checked-in goldens.

The public `hi_lite::Language` registry is a maintained subset of that default
bundle. Languages with equivalent lexical structure share static rule families
in `src/languages/generic.rs`; embedded grammars resolve to their host language.

The same corpus is benchmarked independently with rustybench:

```sh
cargo bench --bench hi_lite
```

The benchmark target lives in `benches/hi_lite.rs` and can also be run directly
with `cargo bench --bench hi_lite`.

`hi_lite_highlight_all_goldens_warm` reports steady-state throughput with
reused scratch storage; the `_cold` variant includes per-fixture scratch setup.
The per-language entries (and their `_cold` counterparts) make slower lexers and
allocation-heavy fixtures easy to attribute.
