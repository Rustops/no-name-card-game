# No Name Card Game
Everything is in the beginning.Maybe this is a card game developed with [Amethyst](https://github.com/amethyst/amethyst).

## Usage

```sh
# The current version is unstable. Use this version to avoid problems.
rustup default 1.47.0
# build binary
cargo build --release
# start server, listening port of 6666（localhost）
./target/release/server -p 6666 --name server
# start client, connect to host of url
cargo run --bin no-name-card-game
# or cp ./target/release/no-name-card-game ./client && ./client/no-name-card-game --url 127.0.0.1:6666 --name client1

# start client2
./client/no-name-card-game --url 127.0.0.1:6666 --name client2
```

# References
- [Amethyst](https://github.com/amethyst/amethyst)
