
```
cargo watch -x check -x test -x run

// coverage:

cargo tarpaulin --ignore-tests
cargo clippy -- -D warnings

// unused deps:
cargo +nightly udeps

RUST_LOG=trace

// When you want to see all logs coming out of a certain test case to debug it you can run
TEST_LOG=true cargo test health_check_works | bunyan
// We are using the `bunyan` CLI to prettify the outputted logs
// The original `bunyan` requires NPM, but you can install a Rust-port with
cargo install bunyan
docker build --tag zero2prod --file Dockerfile .



doctl apps create --spec spec.yaml
```

