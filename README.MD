# sdre-rust-logging

This crate provides logging traits/structs that automatically format log output.

## Usage Example

In this example, we will see a drop in replacement for the standard `log` crate's `info!` macro

```rust
use sdre_rust_logging::SetupLogging;

fn main() {
    let logger: u8 = 0;
    logger.enable_logging();
    info!("Hello World!"); // will print
    debug!("Hello World!"); // will not print
}
```
