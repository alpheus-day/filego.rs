use std::fs::FileType;

use async_std::{
    fs::{self, DirEntry, ReadDir},
    io::{self, ReadExt as _, WriteExt as _},
    path::{Path, PathBuf},
    stream::StreamExt,
};

use crate::merge::{Merge, MergeError};

/// Trait for running the merge process.
pub trait MergeAsyncExt {
    /// Run the check process asynchronously.
    fn run_async(
        &self
    ) -> impl std::future::Future<Output = Result<bool, MergeError>> + Send;
}

impl MergeAsyncExt for Merge {
    async fn run_async(&self) -> Result<bool, MergeError> {
        let in_dir: &Path = match self.in_dir {
            | Some(ref p) => {
                let p: &Path = p.as_ref();

                // if in_dir not exists
                if !p.exists().await {
                    return Err(MergeError::InDirNotFound);
                }

                // if in_dir not a directory
                if !p.is_dir().await {
                    return Err(MergeError::InDirNotDir);
                }

                p
            },
            | None => return Err(MergeError::InDirNotSet),
        };

        let out_file: &Path = match self.out_file {
            | Some(ref p) => p.as_ref(),
            | None => return Err(MergeError::OutFileNotSet),
        };

        // check file size for buffer capacity
        let input_size: usize = {
            let mut entries: ReadDir = match fs::read_dir(in_dir).await {
                | Ok(entries) => entries,
                | Err(_) => return Err(MergeError::InDirNotRead),
            };

            let dir_entry: DirEntry = match entries.next().await.transpose() {
                | Ok(Some(entry)) => entry,
                | Ok(None) => return Err(MergeError::InDirNoFile),
                | Err(_) => return Err(MergeError::InDirNotRead),
            };

            let file_type: FileType = match dir_entry.file_type().await {
                | Ok(file_type) => file_type,
                | Err(_) => return Err(MergeError::InDirNotRead),
            };

            if file_type.is_file() {
                match fs::metadata(dir_entry.path()).await {
                    | Ok(metadata) => metadata.len() as usize,
                    | Err(_) => return Err(MergeError::InDirNotRead),
                }
            } else {
                return Err(MergeError::InDirNoFile);
            }
        };

        let buffer_capacity: usize = input_size.min(self.cap_max);

        // delete outpath target if exists
        if out_file.exists().await {
            if out_file.is_dir().await {
                if fs::remove_dir_all(&out_file).await.is_err() {
                    return Err(MergeError::OutFileNotRemoved);
                }
            } else if fs::remove_file(&out_file).await.is_err() {
                return Err(MergeError::OutFileNotRemoved);
            }
        }

        // create outpath
        if let Some(parent) = out_file.parent() {
            if fs::create_dir_all(parent).await.is_err() {
                return Err(MergeError::OutDirNotCreated);
            }
        }

        let output: fs::File = match fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(out_file)
            .await
        {
            | Ok(file) => file,
            | Err(_) => return Err(MergeError::OutFileNotOpened),
        };

        // writer
        let mut writer: io::BufWriter<fs::File> =
            io::BufWriter::with_capacity(buffer_capacity, output);

        // get inputs
        let mut entries: Vec<PathBuf> = Vec::new();

        let mut read_dir: ReadDir = match fs::read_dir(in_dir).await {
            | Ok(read_dir) => read_dir,
            | Err(_) => return Err(MergeError::InDirNotRead),
        };

        while let Some(ref entry) = read_dir
            .next()
            .await
            .transpose()
            .map_err(|_| MergeError::InDirNotRead)?
        {
            let file_type: FileType = match entry.file_type().await {
                | Ok(file_type) => file_type,
                | Err(_) => return Err(MergeError::InDirNotRead),
            };

            if file_type.is_file() {
                entries.push(entry.path());
            }
        }

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
                match fs::OpenOptions::new().read(true).open(&entry).await {
                    | Ok(file) => file,
                    | Err(_) => return Err(MergeError::InFileNotOpened),
                };

            let mut reader: io::BufReader<fs::File> =
                io::BufReader::with_capacity(buffer_capacity, input);

            let mut buffer: Vec<u8> = vec![0; buffer_capacity];

            loop {
                let read: usize = match reader.read(&mut buffer).await {
                    | Ok(read) => read,
                    | Err(_) => return Err(MergeError::InFileNotRead),
                };

                if read == 0 {
                    break;
                }

                if writer.write(&buffer[..read]).await.is_err() {
                    return Err(MergeError::OutFileNotWritten);
                }
            }
        }

        if writer.flush().await.is_err() {
            return Err(MergeError::OutFileNotWritten);
        }

        Ok(true)
    }
}
