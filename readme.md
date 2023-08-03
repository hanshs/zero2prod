cargo watch -x check -x test -x run

// coverage:
cargo tarpaulin --ignore-tests

cargo clippy -- -D warnings