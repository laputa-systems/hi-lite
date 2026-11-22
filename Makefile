.PHONY: test lint coverage gen-xsh publish bench bench-check bench-list

test:
	cargo test --locked

lint:
	cargo clippy --all-targets --all-features -- --deny warnings

coverage:
	mkdir -p .work/coverage .work/tmp
	TMPDIR="$(CURDIR)/.work/tmp" cargo llvm-cov --locked --branch --json --output-path .work/coverage/report.json

# Regenerate the XSH vocabulary region of src/languages/xsh.rs from the XSH
# registry checkout (XSH_REPO overrides the default path).
gen-xsh:
	cargo run --quiet --example gen_xsh

publish:
	./tools/bump_minor_release.sh

bench:
	@python3 tools/bench.py run

bench-check:
	@python3 tools/bench.py check

bench-list:
	@python3 tools/bench.py list

.PHONY: clean
clean:
	@if test -L ./target || { test -e ./target && ! test -d ./target; }; then \
	  printf '%s\n' 'make clean: ./target must be a local build directory' >&2; exit 1; \
	fi
	rm -rf -- ./target
