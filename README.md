# up-zenoh-example-rust

Example code of using [up-client-zenoh-rust](https://github.com/eclipse-uprotocol/up-client-zenoh-rust)

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
cargo run --example publisher
# Subscriber
cargo run --example subscriber
# RPC Server
cargo run --example rpc_server
# RPC Client
cargo run --example rpc_client
```
