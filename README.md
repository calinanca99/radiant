# radiant

`radiant` is an in-memory key-value store that was inspired by the ["Build your own Redis"](https://build-your-own.org/redis/) book and the [`mini-redis`](https://github.com/tokio-rs/mini-redis) project.

## Protocol

A frame has the following structure:

- 4 bytes size header
- JSON-encoded body for the command/response

The following commands are supported:

- Ping
- Get(Key)
- Set(Key, Bytes)

The following responses are sent:

- Pong
- Ok
- Error(String)
- Get(String, Bytes)

## Examples

- Run the server

```bash
$ cargo run --bin server
```

- Run the "hello-world" example

```bash
$ cargo run --bin hello-world
```

## Follow-up

- [ ] Expirations for keys
- [ ] Persistance
- [ ] (?) Replication
