use crate::importer::tei;
use log::info;
use std::fs;
use std::path::Path;

/// Import a file and convert it to DSL format.
///
/// This command is async, which means Tauri executes it on a separate async task
/// (not the main thread), preventing UI blocking. The actual file I/O and parsing
/// runs on a blocking thread pool via spawn_blocking.
#[tauri::command(async)]
pub async fn import_file(path: String) -> Result<String, String> {
    info!("[Import] import_file command called with path: {}", path);

    // spawn_blocking moves the CPU-bound work to a thread pool,
    // while the async command itself runs off the main thread
    let result = tauri::async_runtime::spawn_blocking(move || {
        info!("[Import] spawn_blocking task started");
        let start = std::time::Instant::now();

        let path_obj = Path::new(&path);
        let extension = path_obj
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        info!("[Import] Reading file...");
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        info!("[Import] File read, {} bytes", content.len());

        let result = match extension.as_str() {
            "xml" | "tei" => {
                info!("[Import] Parsing as XML/TEI...");
                tei::parse(&content)
            }
            _ => {
                info!("[Import] Treating as plain text");
                Ok(content)
            }
        };

        info!("[Import] Processing complete in {:?}", start.elapsed());
        result
    })
    .await
    .map_err(|e| format!("Import task failed: {}", e))?;

    info!("[Import] Command returning result");
    result
}
