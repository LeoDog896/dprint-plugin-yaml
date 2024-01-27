# dprint-plugin-yaml

YAML code formatting plugin for dprint.

## Development

## Setup

Ensure you have Rust installed, and the `wasm32-unknown-unknown` target:

```bash
rustup target add wasm32-unknown-unknown
```

### Building Wasm file

You may wish to try out the plugin by building from source:

1. Run `cargo build --target wasm32-unknown-unknown --release`
2. Reference the file at `./target/wasm32-unknown-unknown/release/dprint_plugin_yaml.wasm` in a dprint configuration file.

### Testing

The tests are in the `./tests/specs` folder. To run the tests, run `cargo test`.
