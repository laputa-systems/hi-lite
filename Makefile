.PHONY: lint bump

lint:
	cargo fmt --all
	cargo clippy --fix --allow-dirty --all-targets --all-features -- --deny warnings

bump:
	python3 tools/bump_minor_release.py
