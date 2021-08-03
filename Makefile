publish:
	cargo login ${{ secrets.CARGO_REGISTRY_TOKEN }} && cargo publish

ci: fmt-check coverage clippy publish-dry-run

fmt-check:
	cargo fmt --all -- --check

coverage:
	cargo install cargo-tarpaulin && cargo tarpaulin -v --fail-under=95 -- --test-threads=1

clippy:
	cargo clippy --all-features -- -D warnings

publish-dry-run:
	cargo publish --dry-run