.PHONY: lint publish

lint:
	cargo fmt --all
	cargo clippy --fix --allow-dirty --all-targets --all-features -- --deny warnings

publish:
	./tools/bump_minor_release.sh
