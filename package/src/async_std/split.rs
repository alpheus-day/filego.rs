use async_std::{
    fs::{self, File},
    io::{self, BufWriter, ReadExt as _, WriteExt as _},
    path::{Path, PathBuf},
};

use crate::split::{Split, SplitResult};

/// Trait for running the split process.
pub trait SplitAsyncExt {
    /// Run the split process asynchronously.
    fn run_async(
        &self
    ) -> impl std::future::Future<Output = io::Result<SplitResult>> + Send;
}

impl SplitAsyncExt for Split {
    async fn run_async(&self) -> io::Result<SplitResult> {
        let in_file: &Path = match self.in_file {
            | Some(ref p) => {
                let p: &Path = p.as_ref();

                // if in_file not exists
                if !p.exists().await {
                    return Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        "in_file path not found",
                    ));
                }

                // if in_file not a file
                if !p.is_file().await {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "in_file is not a path to file",
                    ));
                }

                p
            },
            | None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "in_file is not set",
                ));
            },
        };

        let out_dir: &Path = match self.out_dir {
            | Some(ref p) => {
                let p: &Path = p.as_ref();

                // if out_dir not exists
                if !p.exists().await {
                    fs::create_dir_all(&p).await?;
                } else {
                    // if out_dir not a directory
                    if p.is_file().await {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "out_dir is not a directory",
                        ));
                    }
                }

                p
            },
            | None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "out_dir is not set",
                ));
            },
        };

        let chunk_size: usize = self.chunk_size;

        let buffer_capacity: usize = chunk_size.min(self.cap_max);

        let input: fs::File =
            fs::OpenOptions::new().read(true).open(in_file).await?;

        let file_size: usize = input.metadata().await?.len() as usize;

        let mut reader: io::BufReader<fs::File> =
            io::BufReader::with_capacity(buffer_capacity, input);

        let mut buffer: Vec<u8> = vec![0; chunk_size];

        let mut total_chunks: usize = 0;

        loop {
            let mut offset: usize = 0;

            while offset < chunk_size {
                let bytes_read: usize =
                    reader.read(&mut buffer[offset..]).await?;

                if bytes_read == 0 {
                    break;
                }

                offset += bytes_read;
            }

            if offset == 0 {
                break;
            }

            let output_path: PathBuf = out_dir.join(total_chunks.to_string());

            let output: File = fs::OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(output_path)
                .await?;

            let mut writer: BufWriter<File> =
                io::BufWriter::with_capacity(buffer_capacity, output);

            writer.write_all(&buffer[..offset]).await?;

            writer.flush().await?;

            total_chunks += 1;
        }

        Ok(SplitResult { file_size, total_chunks })
    }
}
