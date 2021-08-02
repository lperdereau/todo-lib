publish:
	cargo login ${{ secrets.CARGO_REGISTRY_TOKEN }} && cargo publish

ci: fmt-check test clippy publish-dry-run

fmt-check:
	cargo fmt --all -- --check

test:
	cargo test --all-features

clippy:
	cargo clippy --all-features -- -D warnings

publish-dry-run:
	cargo publish --dry-run