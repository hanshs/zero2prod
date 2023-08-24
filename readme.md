
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


curl --request POST \
--data 'name=ants&email=hansh.starkopf%40gmail.com' \
https://zero2prod-mt4k4.ondigitalocean.app/subscriptions \
--verbose

# run a single test with additional logging
$env:RUST_LOG="sqlx=error,info"; $env:TEST_LOG="enabled"; cargo t subscribe_fails_if_there_is_a_fatal_database_error | bunyan


```

