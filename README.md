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

- [ ] Add expirations for keys
- [ ] Add persistance and recovery
- [ ] Add replication (based on one writer and multiple read copies)

### DX

- [ ] Add client Rust SDK
- [ ] Add a client CLI
