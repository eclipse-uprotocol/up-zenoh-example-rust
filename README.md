# up-zenoh-example-rust

**NOTE**: The repository is no longer available since the examples is moved back to [up-transport-zenoh-rust](https://github.com/eclipse-uprotocol/up-transport-zenoh-rust) now

Example code of using [up-transport-zenoh-rust](https://github.com/eclipse-uprotocol/up-transport-zenoh-rust)

## Build

```shell
# Check clippy
cargo clippy --all-targets
# Build
cargo build
```

## Run examples

```shell
# Publisher
cargo run --bin publisher
# Subscriber
cargo run --bin subscriber
# RPC Server
cargo run --bin rpc_server
# RPC Client
cargo run --bin rpc_client
```
