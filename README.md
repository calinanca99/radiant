# radiant

`radiant` is an in-memory key-value store built in Rust.

## Protocol

The communication is done through gRPC. Check out the `protocol` module to see the RPC and message definitions.

## Examples

1. Run the server

   ```bash
   $ cargo run --bin server
   ```

1. Run the "simple-usage" example

   ```bash
   $ cargo run --bin simple-usage
   ```

## Follow-up

### Features

- [ ] Add memory limit for and memory eviction policy
- [ ] Implement authentication
- [ ] Add/Enable TLS
- [ ] Add persistance and recovery
- [ ] Add replication (based on one writer and multiple read copies)

### DX

- [ ] Add Docker Image for the server
