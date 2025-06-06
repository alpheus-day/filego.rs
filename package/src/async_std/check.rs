use async_std::{
    fs,
    path::{Path, PathBuf},
};

use crate::check::{
    Check, CheckError, CheckResult, CheckResultError, CheckResultErrorType,
};

/// Trait for running the check process.
pub trait CheckAsyncExt {
    /// Run the check process asynchronously.
    fn run_async(
        &self
    ) -> impl std::future::Future<Output = Result<CheckResult, CheckError>> + Send;
}

impl CheckAsyncExt for Check {
    async fn run_async(&self) -> Result<CheckResult, CheckError> {
        let in_dir: &Path = match self.in_dir {
            | Some(ref p) => {
                let p: &Path = p.as_ref();

                // if in_dir not exists
                if !p.exists().await {
                    return Err(CheckError::InDirNotFound);
                }

                // if in_dir not a directory
                if !p.is_dir().await {
                    return Err(CheckError::InDirNotDir);
                }

                p
            },
            | None => return Err(CheckError::InDirNotSet),
        };

        let file_size: usize = match self.file_size {
            | Some(s) => s,
            | None => return Err(CheckError::FileSizeNotSet),
        };

        let total_chunks: usize = match self.total_chunks {
            | Some(s) => s,
            | None => return Err(CheckError::TotalChunksNotSet),
        };

        let mut actual_size: usize = 0;
        let mut missing: Vec<usize> = Vec::new();

        for i in 0..total_chunks {
            let target_file: PathBuf = in_dir.join(i.to_string());

            if !target_file.exists().await || !target_file.is_file().await {
                missing.push(i);
                continue;
            }

            actual_size +=
                match fs::OpenOptions::new().read(true).open(&target_file).await
                {
                    | Ok(f) => match f.metadata().await {
                        | Ok(m) => m.len() as usize,
                        | Err(_) => return Err(CheckError::InFileNotRead),
                    },
                    | Err(_) => return Err(CheckError::InFileNotOpened),
                }
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
