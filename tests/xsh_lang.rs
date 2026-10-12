//! Guard the generated vocabulary region of `src/languages/xsh.rs` against
//! drift from the XSH registry.
//!
//! Re-runs the same `syn`-based vocabulary extraction as `examples/gen_xsh.rs`
//! and re-splices it into the committed file.  The comparison ignores the
//! `// Generated from <path>` header line so it works regardless of where the
//! registry checkout lives.  The test needs the XSH registry
//! checkout, so it is `#[ignore]`d and runs on demand instead of skipping silently.

// The generator is only executed through `default_repo()`/`regenerate()` here;
// the CLI entry points it also defines are dead code in this crate.
#[allow(dead_code)]
#[path = "../examples/gen_xsh.rs"]
mod gen_xsh;

use std::path::Path;

/// Drop the `// Generated from <path>` line, which records the local checkout
/// location rather than a property of the vocabulary.
fn strip_generated_from(content: &str) -> String {
    content
        .lines()
        .filter(|line| !line.starts_with("// Generated from"))
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
#[ignore = "needs the XSH registry checkout (XSH_REPO or ~/d/laputa-systems/xsh)"]
fn xsh_lang_matches_registry() {
    let repo = gen_xsh::default_repo();
    assert!(
        repo.join("Cargo.toml").exists(),
        "{} is not an xsh checkout; set XSH_REPO",
        repo.display()
    );
    let vocab = gen_xsh::regenerate(&repo).expect("parsing the XSH registry source");
    let committed = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("languages")
            .join("xsh.rs"),
    )
    .expect("reading src/languages/xsh.rs");
    let spliced = gen_xsh::splice_into(&committed, &vocab.content)
        .expect("committed xsh.rs has the generation markers");
    assert_eq!(
        strip_generated_from(&committed),
        strip_generated_from(&spliced),
        "src/languages/xsh.rs is out of date with the registry at {}; run `make gen-xsh`",
        repo.display()
    );
}
