use std::{
    fs, io,
    path::{Path, PathBuf},
};

/// Run asynchronously with `async_std` feature.
///
/// To use it, add the following code to the `Cargo.toml` file:
///
/// ```toml
/// [dependencies]
/// filego = { version = "*", features = ["async_std"] }
/// ```
#[cfg(feature = "async_std")]
pub mod async_std {
    pub use crate::async_std::check::CheckAsyncExt;
}

/// Run asynchronously with `tokio` feature.
///
/// To use it, add the following code to the `Cargo.toml` file:
///
/// ```toml
/// [dependencies]
/// filego = { version = "*", features = ["tokio"] }
/// ```
#[cfg(feature = "tokio")]
pub mod tokio {
    pub use crate::tokio::check::CheckAsyncExt;
}

/// Error type of the result from the check process.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckResultErrorType {
    /// Some of the chunks are missing to merge the file.
    Missing,
    /// The actual file size is not equal the input file size.
    Size,
}

impl CheckResultErrorType {
    /// Get the error type from code.
    pub fn from_code<C: AsRef<str>>(code: C) -> Option<Self> {
        match code.as_ref() {
            | "missing" => Some(Self::Missing),
            | "size" => Some(Self::Size),
            | _ => None,
        }
    }

    /// Get the code of the error type as `&str`.
    pub fn as_code(&self) -> &str {
        match self {
            | Self::Missing => "missing",
            | Self::Size => "size",
        }
    }

    /// Get the code of the error type as `String`.
    pub fn to_code(&self) -> String {
        self.as_code().to_string()
    }
}

/// Error of the result from the check process.
#[derive(Debug, Clone)]
pub struct CheckResultError {
    /// Type of error of the check.
    pub error_type: CheckResultErrorType,
    /// Error message of the check.
    pub message: String,
    /// Missing chunk(s) to merge the file.
    pub missing: Option<Vec<usize>>,
}

/// Result of the check process.
#[derive(Debug, Clone)]
pub struct CheckResult {
    /// Successful / Failed check.
    pub success: bool,
    /// Error details of the check.
    pub error: Option<CheckResultError>,
}

/// Process to check the file integrity.
///
/// The function will return [`CheckResult`] (that may come
/// with `success:false`) when the checking process runs successfully.
/// Otherwise, it will return Error.
///
/// ## Example
///
/// ```no_run
/// use std::path::PathBuf;
///
/// use filego::check::{Check, CheckResult};
///
/// let result: CheckResult = Check::new()
///     .in_dir(PathBuf::from("path").join("to").join("dir"))
///     .file_size(0) // result from split function...
///     .total_chunks(0) // result from split function...
///     .run()
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct Check {
    pub in_dir: Option<PathBuf>,
    pub file_size: Option<usize>,
    pub total_chunks: Option<usize>,
}

impl Check {
    /// Create a new check process.
    pub fn new() -> Self {
        Self { in_dir: None, file_size: None, total_chunks: None }
    }

    /// Create a new check process from an existing one.
    pub fn from<P: Into<Check>>(process: P) -> Self {
        process.into()
    }

    /// Set the input directory.
    pub fn in_dir<InDir: AsRef<Path>>(
        mut self,
        path: InDir,
    ) -> Self {
        self.in_dir = Some(path.as_ref().to_path_buf());
        self
    }

    /// Set the size of the original file.
    pub fn file_size(
        mut self,
        size: usize,
    ) -> Self {
        self.file_size = Some(size);
        self
    }

    /// Set the total number of chunks splitted from the original file.
    pub fn total_chunks(
        mut self,
        chunks: usize,
    ) -> Self {
        self.total_chunks = Some(chunks);
        self
    }

    /// Run the check process.
    pub fn run(&self) -> io::Result<CheckResult> {
        let in_dir: &Path = match self.in_dir {
            | Some(ref p) => {
                let p: &Path = p.as_ref();

                // if in_dir not exists
                if !p.exists() {
                    return Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        "in_dir path not found",
                    ));
                }

                // if in_dir not a directory
                if !p.is_dir() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "in_dir is not a directory",
                    ));
                }

                p
            },
            | None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "in_dir is not set",
                ));
            },
        };

        let file_size: usize = match self.file_size {
            | Some(s) => s,
            | None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "file_size is not set",
                ));
            },
        };

        let total_chunks: usize = match self.total_chunks {
            | Some(s) => s,
            | None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "total_chunks is not set",
                ));
            },
        };

        let mut actual_size: usize = 0;
        let mut missing: Vec<usize> = Vec::new();

        for i in 0..total_chunks {
            let target_file: PathBuf = in_dir.join(i.to_string());

            if !target_file.exists() || !target_file.is_file() {
                missing.push(i);
                continue;
            }

            actual_size += fs::OpenOptions::new()
                .read(true)
                .open(&target_file)?
                .metadata()?
                .len() as usize;
        }

        if !missing.is_empty() {
            return Ok(CheckResult {
                success: false,
                error: Some(CheckResultError {
                    error_type: CheckResultErrorType::Missing,
                    message: "Missing chunk(s)".to_string(),
                    missing: Some(missing),
                }),
            });
        }

        if actual_size != file_size {
            return Ok(CheckResult {
                success: false,
                error: Some(CheckResultError {
                    error_type: CheckResultErrorType::Size,
                    message:
                        "the size of chunks is not equal to file_size parameter"
                            .to_string(),
                    missing: None,
                }),
            });
        }

        Ok(CheckResult { success: true, error: None })
    }
}

impl Default for Check {
    fn default() -> Self {
        Self::new()
    }
}
