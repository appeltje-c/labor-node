# Labor Node

A FRAME-based [Substrate](https://www.substrate.io/) node to run the Labor chain :rocket:

## Getting Started

Follow the steps below to get started with the Labor Node.

### Rust Setup

First, complete the [basic Rust setup instructions](./docs/rust-setup.md).

### Run

Use Rust's native `cargo` command to build and launch the labor node:

```sh
cargo run --release -- --dev --tmp
```

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node without launching it:

```sh
cargo build --release
```

### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and subcommands:

```sh
./target/release/labor-node -h
```

## Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after you terminate the
process. After the project has been built, there are other ways to launch the node.

### Single-Node Development Chain

This command will start the single-node development chain with persistent state:

```bash
./target/release/labor-node --dev
```

Purge the development chain's state:

```bash
./target/release/labor-node purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_BACKTRACE=1 ./target/release/labor-node -ldebug --dev
```

After the node has been [built](#build), refer to the embedded documentation to learn more about the capabilities and
configuration parameters that it exposes:

```shell
./target/release/labor-node --help
```

### Connect with Labor-JS Apps Front-end

@todo

### Multi-Node Local Testnet

@todo
