# Bifrost

Bifrost is a Redis-like in-memory database server implemented in Rust. It uses the RESP (Redis Serialization Protocol) for client-server communication and supports basic Redis commands.

## Features

- In-memory key-value storage
- RESP (Redis Serialization Protocol) support
- Concurrent connections using async I/O
- Thread-safe storage using `parking_lot::RwLock`

## Building

To build the project, you'll need Rust and Cargo installed. Then run:

```bash
cargo build --release
```

## Running

To start the Bifrost server:

```bash
cargo run --release
```

By default, the server listens on `127.0.0.1:6379`.

## Testing

Run the test suite with:

```bash
cargo test
```

## Supported Commands

Bifrost currently supports the following Redis commands:

- `PING` - Test connection
- `ECHO <message>` - Echo back a message
- `GET <key>` - Get the value of a key
- `SET <key> <value>` - Set the value of a key
- `DEL <key>` - Delete a key
- `EXISTS <key>` - Check if a key exists
- `INCR <key>` - Increment the integer value of a key
- `DECR <key>` - Decrement the integer value of a key

## Connecting

You can connect to Bifrost using any Redis client. For example, using `redis-cli`:

```bash
redis-cli -p 6379
```

Example commands:
```
127.0.0.1:6379> PING
PONG
127.0.0.1:6379> SET mykey "Hello"
OK
127.0.0.1:6379> GET mykey
"Hello"
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.