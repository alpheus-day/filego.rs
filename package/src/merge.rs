use std::{
    fs,
    io::{self, Read as _, Write as _},
    path::{Path, PathBuf},
};

use crate::BUFFER_CAPACITY_MAX_DEFAULT;

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
    pub use crate::async_std::merge::MergeAsyncExt;
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
    pub use crate::tokio::merge::MergeAsyncExt;
}

/// Process to merge chunks from a directory to a path.
///
/// ## Example
///
/// ```no_run
/// use std::path::PathBuf;
///
/// use filego::merge::Merge;
///
/// let result: bool = Merge::new()
///     .in_dir(PathBuf::from("path").join("to").join("dir"))
///     .out_file(PathBuf::from("path").join("to").join("file"))
///     .run()
///     .unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct Merge {
    pub in_dir: Option<PathBuf>,
    pub out_file: Option<PathBuf>,
    pub cap_max: usize,
}

impl Merge {
    /// Create a new merge process.
    pub fn new() -> Self {
        Self {
            in_dir: None,
            out_file: None,
            cap_max: BUFFER_CAPACITY_MAX_DEFAULT,
        }
    }

    /// Create a new merge process from an existing one.
    pub fn from<P: Into<Merge>>(process: P) -> Self {
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

    /// Set the output file.
    pub fn out_file<OutFile: AsRef<Path>>(
        mut self,
        path: OutFile,
    ) -> Self {
        self.out_file = Some(path.as_ref().to_path_buf());
        self
    }

    /// Set the maximum size of the buffer capacity.
    ///
    /// By default, the buffer capacity is based on the size of the inputs in
    /// the input directory. The buffer capacity is limited and will not
    /// exceed [`BUFFER_CAPACITY_MAX_DEFAULT`]. The default value is recommended
    /// unless a large size file will be processed through the split process.
    pub fn max_buffer_capacity(
        mut self,
        capacity: usize,
    ) -> Self {
        self.cap_max = capacity;
        self
    }

    /// Run the merge process.
    pub fn run(&self) -> io::Result<bool> {
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

        let out_file: &Path = match self.out_file {
            | Some(ref p) => p.as_ref(),
            | None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "out_file is not set",
                ));
            },
        };

        // check file size for buffer capacity
        let input_size: usize = if let Some(file) = fs::read_dir(in_dir)?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_file())
            .map(|entry| entry.path())
            .next()
        {
            fs::metadata(file)?.len() as usize
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No files found in in_dir",
            ));
        };

        let buffer_capacity: usize = input_size.min(self.cap_max);

        // delete outpath target if exists
        if out_file.exists() {
            if out_file.is_dir() {
                fs::remove_dir_all(out_file)?;
            } else {
                fs::remove_file(out_file)?;
            }
        }

        // create outpath
        if let Some(parent) = out_file.parent() {
            fs::create_dir_all(parent)?;
        }

        let output: fs::File = fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(out_file)?;

        // writer
        let mut writer: io::BufWriter<fs::File> =
            io::BufWriter::with_capacity(buffer_capacity, output);

        // get inputs
        let mut entries: Vec<PathBuf> = fs::read_dir(in_dir)?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_file())
            .map(|entry| entry.path())
            .collect();

        entries.sort_by_key(|entry| {
            entry
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .parse::<usize>()
                .unwrap()
        });

        // merge
        for entry in entries {
            let input: fs::File =
                fs::OpenOptions::new().read(true).open(&entry)?;

            let mut reader: io::BufReader<fs::File> =
                io::BufReader::with_capacity(buffer_capacity, input);

            let mut buffer: Vec<u8> = vec![0; buffer_capacity];

            loop {
                let read: usize = reader.read(&mut buffer)?;

                if read == 0 {
                    break;
                }

                writer.write_all(&buffer[..read])?;
            }
        }

        writer.flush()?;

        Ok(true)
    }
}

impl Default for Merge {
    fn default() -> Self {
        Self::new()
    }
}
