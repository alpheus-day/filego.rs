use async_std::{
    fs::{self, File},
    io::{self, BufWriter, ReadExt as _, WriteExt as _},
    path::{Path, PathBuf},
};

use crate::split::{Split, SplitError, SplitResult};

/// Trait for running the split process.
pub trait SplitAsyncExt {
    /// Run the split process asynchronously.
    fn run_async(
        &self
    ) -> impl std::future::Future<Output = Result<SplitResult, SplitError>> + Send;
}

impl SplitAsyncExt for Split {
    async fn run_async(&self) -> Result<SplitResult, SplitError> {
        let in_file: &Path = match self.in_file {
            | Some(ref p) => {
                let p: &Path = p.as_ref();

                // if in_file not exists
                if !p.exists().await {
                    return Err(SplitError::InFileNotFound);
                }

                // if in_file not a file
                if !p.is_file().await {
                    return Err(SplitError::InFileNotFile);
                }

                p
            },
            | None => return Err(SplitError::InFileNotSet),
        };

        let out_dir: &Path = match self.out_dir {
            | Some(ref p) => {
                let p: &Path = p.as_ref();

                // if out_dir not exists
                if !p.exists().await {
                    if fs::create_dir_all(p).await.is_err() {
                        return Err(SplitError::OutDirNotDir);
                    }
                } else {
                    // if out_dir not a directory
                    if p.is_file().await {
                        return Err(SplitError::OutDirNotDir);
                    }
                }

                p
            },
            | None => return Err(SplitError::OutDirNotSet),
        };

        let chunk_size: usize = self.chunk_size;

        let buffer_capacity: usize = chunk_size.min(self.cap_max);

        let input: fs::File =
            match fs::OpenOptions::new().read(true).open(in_file).await {
                | Ok(f) => f,
                | Err(_) => return Err(SplitError::InFileNotOpened),
            };

        let file_size: usize = match input.metadata().await {
            | Ok(m) => m.len() as usize,
            | Err(_) => return Err(SplitError::InFileNotRead),
        };

        let mut reader: io::BufReader<fs::File> =
            io::BufReader::with_capacity(buffer_capacity, input);

        let mut buffer: Vec<u8> = vec![0; chunk_size];

        let mut total_chunks: usize = 0;

        loop {
            let mut offset: usize = 0;

            while offset < chunk_size {
                let bytes_read: usize =
                    match reader.read(&mut buffer[offset..]).await {
                        | Ok(n) => n,
                        | Err(_) => return Err(SplitError::InFileNotRead),
                    };

                if bytes_read == 0 {
                    break;
                }

                offset += bytes_read;
            }

            if offset == 0 {
                break;
            }

            let output_path: PathBuf = out_dir.join(total_chunks.to_string());

            let output: File = match fs::OpenOptions::new()
                .create(true)
                .truncate(true)
                .write(true)
                .open(output_path)
                .await
            {
                | Ok(f) => f,
                | Err(_) => return Err(SplitError::OutFileNotOpened),
            };

            let mut writer: BufWriter<File> =
                io::BufWriter::with_capacity(buffer_capacity, output);

            if writer.write_all(&buffer[..offset]).await.is_err() {
                return Err(SplitError::OutFileNotWritten);
            }

            if writer.flush().await.is_err() {
                return Err(SplitError::OutFileNotWritten);
            }

            total_chunks += 1;
        }

        Ok(SplitResult { file_size, total_chunks })
    }
}
