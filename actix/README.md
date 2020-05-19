# Actix & JSON

## Usage

### server

```bash
cargo run
# Started http server: 127.0.0.1:8080
```

## Benchmark

Make a release build, and execute the binary.

```sh
cargo build --release
target/release/rust-actix
```

Then run performance test

```sh
echo 'GET http://localhost:8080/health' | vegeta attack -duration 2m | tee result-rust-actix.bin | vegeta report
```
