This is a small proof-of-concept showing how a WASM interpreter/VM can hold and manage some state which the WASM programs can interact with.

In this example a tictactoe engine is in charge of keeping track of whos turn it is and whether or not the game is over (because of win or draw), while the input wasm programs (players) are responsible for making their move based on the state of the board.

The tictactoe engine is found in `src/main.rs`, while the code for the players is found in `src/lib.rs`.

To build and run the example:
```shell
$ cargo build
$ cargo build --target wasm32-unknown-unknown --release
$ ./target/debug/tictactoe-engine ./target/wasm32-unknown-unknown/release/player.wasm ./target/wasm32-unknown-unknown/release/player.wasm
```