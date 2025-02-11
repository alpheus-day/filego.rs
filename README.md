# FileGo

A file splitting & merging solution.

## Installation

To install this package, run the following command:

```bash
cargo add filego
```

## Quick Start

Split file from a path to a directory with `Split` struct.

```rust
use std::path::PathBuf;

use filego::split::{Split, SplitResult};

fn example() {
    let result: SplitResult = Split::new()
        .in_file(PathBuf::from("path").join("to").join("file"))
        .out_dir(PathBuf::from("path").join("to").join("dir"))
        .run()
        .unwrap();
}
```

Async version also available with the `async-std`/`async_std` and `tokio` features:

```rust
// This is a `async-std` example

use async_std::path::PathBuf;

use filego::split::{
    Split,
    SplitResult,
    async_std::AsyncSplitExt as _,
};

async fn example() {
    let result: SplitResult = Split::new()
        .in_file(PathBuf::from("path").join("to").join("file"))
        .out_dir(PathBuf::from("path").join("to").join("dir"))
        .run_async()
        .await
        .unwrap();
}
```

```rust
// This is a `tokio` example

use std::path::PathBuf;

use filego::split::{
    Split,
    SplitResult,
    tokio::AsyncSplitExt as _,
};

async fn example() {
    let result: SplitResult = Split::new()
        .in_file(PathBuf::from("path").join("to").join("file"))
        .out_dir(PathBuf::from("path").join("to").join("dir"))
        .run_async()
        .await
        .unwrap();
}
```

## License

This project is licensed under the terms of the MIT license.
