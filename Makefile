run:
	cargo run | bunyan
check:
	cargo check
test:
	TEST_LOG=true cargo test | bunyan
coverate:
	cargo tarpaulin --ignore-tests
lint:
	cargo clippy -- -D warnings
format:
	cargo fmt 
audit:
	cargo audit
unused-dep:
	cargo +nightly udepsrun:
	cargo run