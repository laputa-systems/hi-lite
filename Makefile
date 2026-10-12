.PHONY: test lint gen-xsh publish

test:
	cargo test --locked

lint:
	cargo clippy --all-targets --all-features -- --deny warnings

# Regenerate the XSH vocabulary region of src/languages/xsh.rs from the XSH
# registry checkout (XSH_REPO overrides the default path).
gen-xsh:
	cargo run --quiet --example gen_xsh

publish:
	./tools/bump_minor_release.sh
