use crate::errors::{Result, SagaError};
use crate::importer::tei::{self, ImportResult};
use std::fs;
use std::path::Path;

/// Import a file and convert it to DSL format, also extracting metadata if available.
///
/// This command is async, which means Tauri executes it on a separate async task
/// (not the main thread), preventing UI blocking. The actual file I/O and parsing
/// runs on a blocking thread pool via spawn_blocking.
#[tauri::command(async)]
pub async fn import_file(path: String) -> Result<ImportResult> {
    // spawn_blocking moves the CPU-bound work to a thread pool,
    // while the async command itself runs off the main thread
    tauri::async_runtime::spawn_blocking(move || -> Result<ImportResult> {
        let path_obj = Path::new(&path);
        let extension = path_obj
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        let content = fs::read_to_string(&path)?;

        match extension.as_str() {
            "xml" | "tei" => tei::parse(&content).map_err(SagaError::Parser),
            // Plain text files - return as DSL with no metadata
            _ => Ok(ImportResult {
                dsl: content,
                metadata: None,
            }),
        }
    })
    .await
    .map_err(|e| SagaError::Internal(format!("Import task failed: {}", e)))?
}
