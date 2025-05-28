## 0.5.4 (2025-05-28)

### What's Changed

- Update documentation

## 0.5.3 (2025-03-12)

### What's Changed

- Downgrade `tokio` dependency from `^1.43.0` to `^1.40.0` for compatibility

## 0.5.2 (2025-02-24)

### What's Changed

- Fixed issues with async check functions
- Optimized async merge functions

## 0.5.1 (2025-02-22)

### What's Changed

- Update documentation

## 0.5.0 (2025-02-22)

### Breaking Changes

- Tokio based functions moved to `tokio` module (require `tokio` feature)

### What's New

- Add support for synchronize functions
- Add support for `async-std` (require `async-std`/`async_std` feature)
- Add `from` function to create a new process from existing process

### What's Changed

- Update to 2024 edition
- Move `CHUNK_SIZE_DEFAULT`
- Move `BUFFER_CAPACITY_MAX_DEFAULT`

### Migrating from 0.4.x to 0.5.0

`tokio` feature is required for tokio extension.

```diff
- filego = { version = "~0.4.1" }
+ filego = { version = "~0.5.0", features = ["tokio"] }
```

This is a migration example for split process:

```diff
use std::path::PathBuf;

use filego::split::{
    Split, 
    SplitResult, 
+   tokio::SplitAsyncExt as _,
};

async fn example() {
    let result: SplitResult = Split::new()
        .in_file(PathBuf::from("path").join("to").join("file"))
        .out_dir(PathBuf::from("path").join("to").join("dir"))
-       .run()
+       .run_async()
        .await
        .unwrap();
}
```

## 0.4.1 (2024-12-16)

### What's Changed

- Update dependencies
- Update documentation

## 0.4.0 (2024-10-26)

### Breaking Changes

- Rework in `split` module
- Rework in `check` module
- Rework in `merge` module

Please refer to docs for the new usage.

### What's New

- Add `config` module

### What's Changed

- Update dependencies
- Update documentation

## 0.3.0 (2024-10-13)

### Breaking Changes

- Move `split` related stuffs into `split` module
- Move `check` related stuffs into `check` module
- Move `merge` related stuffs into `merge` module
- Changes in accepted value type of `in_file` in `SplitOptions`:
    - `String` => `&PathBuf`
- Changes in accepted value type of `out_dir` in `SplitOptions`:
    - `String` => `&PathBuf`
- Changes in accepted value type of `in_dir` in `CheckOptions`:
    - `String` => `&PathBuf`
- Remove `std::fmt::Display` impl from `CheckResultErrorType`
- Changes in accepted value type of `in_dir` in `MergeOptions`:
    - `String` => `&PathBuf`
- Changes in accepted value type of `out_file` in `MergeOptions`:
    - `String` => `&PathBuf`

### What's New

- Add different derives for different structs
- Add `as_code` function for `CheckResultErrorType`
- Add `to_code` function for `CheckResultErrorType`

## 0.2.3 (2024-08-04)

### What's Changed

- Update description
- Update dependencies

## 0.2.2 (2024-06-22)

### What's Changed

- Update documentation

## 0.2.1 (2024-06-22)

### What's Changed

- Update `Cargo.toml`

## 0.2.0 (2024-06-19)

### What's New

- Add inline documentation for structs and functions

### What's Changed

- Update dependencies

## 0.1.0 (2024-05-22)

First release
