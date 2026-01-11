use crate::importer::tei;
use std::fs;
use std::path::Path;

/// Import a file and convert it to DSL format.
/// This command runs on a background thread to avoid blocking the UI.
#[tauri::command]
pub async fn import_file(path: String) -> Result<String, String> {
    // Run the file I/O and parsing on a blocking thread pool
    // to avoid blocking the async runtime and allow the UI to remain responsive
    tauri::async_runtime::spawn_blocking(move || {
        let path_obj = Path::new(&path);
        let extension = path_obj
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;

        match extension.as_str() {
            "xml" | "tei" => tei::parse(&content),
            _ => Ok(content), // Default to treating as plain text
        }
    })
    .await
    .map_err(|e| format!("Import task failed: {}", e))?
}
