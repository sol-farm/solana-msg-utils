# solana-msg-utils

utilities for emitting efficient messages on-chain.

# Usage

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
solana-msg-utils = "0.1.0"
```

then you can use the crate like so:

```rust
// emit a trace level msg
use solana_program::msg;
use solana_msg_utils::{msg_trace, msg_panic, sum};
msg_trace!("{}", "this is a trace level log");
msg_panic!("{}", "this is a traced & off-chain parsable panic message for better error handling than annoying decimal numbers and hexadecimal digits, because debugging that is really, really f**king tilting (if you cant tell)");
```



# Documentation

* [docs.rs](https://docs.rs/solana-msg-utils")
