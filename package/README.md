# FileGo

A file splitting & merging solution.

## Quick Start

Split file from a path to a directory with `Split` struct.

```rust
use std::path::PathBuf;

use filego::split::{Split, SplitResult};

let result: SplitResult = Split::new()
    .in_file(PathBuf::from("path").join("to").join("file"))
    .out_dir(PathBuf::from("path").join("to").join("dir"))
    .run()
    .unwrap();
```

Async version also available with the `async_std` and `tokio` features:

```rust
// This is a `async_std` example

use async_std::path::PathBuf;

use filego::split::{
    Split,
    SplitResult,
    async_std::SplitAsyncExt as _,
};

let result: SplitResult = Split::new()
    .in_file(PathBuf::from("path").join("to").join("file"))
    .out_dir(PathBuf::from("path").join("to").join("dir"))
    .run_async()
    .await
    .unwrap();
```

```rust
// This is a `tokio` example

use std::path::PathBuf;

use filego::split::{
    Split,
    SplitResult,
    tokio::SplitAsyncExt as _,
};

let result: SplitResult = Split::new()
    .in_file(PathBuf::from("path").join("to").join("file"))
    .out_dir(PathBuf::from("path").join("to").join("dir"))
    .run_async()
    .await
    .unwrap();
```

## License

This project is licensed under the terms of the MIT license.
