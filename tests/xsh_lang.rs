//! Guard the generated vocabulary region of `src/languages/xsh.rs` against
//! drift from the XSH registry.
//!
//! Re-runs the generator against the pinned Git revision and compares the
//! entire generated region. The test runs on demand with an XSH repository
//! that contains the pinned commit.

// The generator is only executed through `default_repo()`/`regenerate()` here;
// the CLI entry points it also defines are dead code in this crate.
#[allow(dead_code)]
#[path = "../examples/gen_xsh.rs"]
mod gen_xsh;

use std::path::Path;

#[test]
#[ignore = "needs the XSH registry checkout (XSH_REPO or ~/d/laputa-systems/xsh)"]
fn xsh_lang_matches_registry() {
    let repo = gen_xsh::default_repo();
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
        committed,
        spliced,
        "src/languages/xsh.rs is out of date with the registry at {}; run `make gen-xsh`",
        repo.display()
    );
}
