//! # FileGo
//!
//! A file splitting & merging solution.
//!
//! ## Quick Start
//!
//! Split file from a path to a directory with `Split` struct.
//!
//! ```no_run
//! use std::path::PathBuf;
//!
//! use filego::split::{Split, SplitResult};
//!
//! let result: SplitResult = Split::new()
//!     .in_file(PathBuf::from("path").join("to").join("file"))
//!     .out_dir(PathBuf::from("path").join("to").join("dir"))
//!     .run()
//!     .unwrap();
//! ```
//!
//! Async version also available with the `async_std` and `tokio` features:
//!
//! ```no_run
//! // This is a `async_std` example
//!
//! use async_std::path::PathBuf;
//!
//! use filego::split::{
//!     Split,
//!     SplitResult,
//!     async_std::SplitAsyncExt as _,
//! };
//!
//! # async fn example() {
//! let result: SplitResult = Split::new()
//!     .in_file(PathBuf::from("path").join("to").join("file"))
//!     .out_dir(PathBuf::from("path").join("to").join("dir"))
//!     .run_async()
//!     .await
//!     .unwrap();
//! # }
//! ```
//!
//! ```no_run
//! // This is a `tokio` example
//!
//! use std::path::PathBuf;
//!
//! use filego::split::{
//!     Split,
//!     SplitResult,
//!     tokio::SplitAsyncExt as _,
//! };
//!
//! # async fn example() {
//! let result: SplitResult = Split::new()
//!     .in_file(PathBuf::from("path").join("to").join("file"))
//!     .out_dir(PathBuf::from("path").join("to").join("dir"))
//!     .run_async()
//!     .await
//!     .unwrap();
//! # }
//! ```

/// Split module.
pub mod split;

/// Check module.
pub mod check;

/// Merge module.
pub mod merge;

/// Functions implemented with `async_std`.
#[cfg(feature = "async_std")]
pub(crate) mod async_std;

/// Functions implemented with `tokio`.
#[cfg(feature = "tokio")]
pub(crate) mod tokio;

/// The default chunk size in bytes.
pub const CHUNK_SIZE_DEFAULT: usize = 2 * 1024 * 1024;

/// The default maximum size of the buffer capacity in bytes.
pub const BUFFER_CAPACITY_MAX_DEFAULT: usize = 10 * 1024 * 1024;
