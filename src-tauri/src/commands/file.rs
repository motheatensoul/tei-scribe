use crate::annotations::AnnotationSet;
use crate::errors::Result;
use crate::metadata::Metadata;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileContent {
    pub path: String,
    pub content: String,
}

#[tauri::command]
pub fn open_file(path: String) -> Result<FileContent> {
    let content = fs::read_to_string(&path)?;
    Ok(FileContent { path, content })
}

#[tauri::command]
pub fn load_text_file(path: String) -> Result<String> {
    Ok(fs::read_to_string(&path)?)
}

#[tauri::command]
pub fn save_file(path: String, content: String) -> Result<()> {
    fs::write(&path, &content)?;
    Ok(())
}

#[tauri::command]
pub fn export_tei(path: String, tei_content: String) -> Result<()> {
    let path = PathBuf::from(path);
    fs::write(&path, &tei_content)?;
    Ok(())
}

#[tauri::command]
pub fn export_html(path: String, html_content: String) -> Result<()> {
    let path = PathBuf::from(path);
    fs::write(&path, &html_content)?;
    Ok(())
}

// Project archive format (.teis)
// A ZIP file containing:
// - source.dsl: The DSL source text
// - output.xml: The compiled TEI-XML
// - confirmations.json: Word index -> lemma mapping
// - manifest.json: Project metadata

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectManifest {
    pub version: String,
    pub template_id: String,
    pub created: String,
    pub modified: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LemmaConfirmation {
    pub lemma: String,
    pub msa: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normalized: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectData {
    pub source: String,
    pub output: String,
    /// Legacy lemma confirmations (for backward compat, derived from annotations)
    pub confirmations: HashMap<u32, LemmaConfirmation>,
    pub manifest: ProjectManifest,
    /// Optional manuscript metadata (new in v1.1)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    /// Full annotation set (new in v1.2)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<AnnotationSet>,
}

#[tauri::command]
pub fn save_project(
    path: String,
    source: String,
    output: String,
    confirmations_json: String,
    template_id: String,
    metadata_json: Option<String>,
    annotations_json: Option<String>,
) -> Result<()> {
    let path = PathBuf::from(&path);
    let file = File::create(&path)?;
    let mut zip = ZipWriter::new(file);
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o644);

    // Write source.dsl
    zip.start_file("source.dsl", options)?;
    zip.write_all(source.as_bytes())?;

    // Write output.xml
    zip.start_file("output.xml", options)?;
    zip.write_all(output.as_bytes())?;

    // Write confirmations.json (backward compat for older app versions)
    zip.start_file("confirmations.json", options)?;
    zip.write_all(confirmations_json.as_bytes())?;

    // Write annotations.json (new in v1.2, full annotation set)
    if let Some(ref ann_json) = annotations_json {
        zip.start_file("annotations.json", options)?;
        zip.write_all(ann_json.as_bytes())?;
    }

    // Write metadata.json if provided
    if let Some(ref meta_json) = metadata_json {
        zip.start_file("metadata.json", options)?;
        zip.write_all(meta_json.as_bytes())?;
    }

    // Create and write manifest.json
    let now = chrono_lite_now();
    let manifest = ProjectManifest {
        version: "1.2".to_string(),
        template_id,
        created: now.clone(),
        modified: now,
    };
    let manifest_json = serde_json::to_string_pretty(&manifest)?;
    zip.start_file("manifest.json", options)?;
    zip.write_all(manifest_json.as_bytes())?;

    zip.finish()?;

    Ok(())
}

#[tauri::command]
pub fn open_project(path: String) -> Result<ProjectData> {
    let path = PathBuf::from(&path);
    let file = File::open(&path)?;
    let mut archive = ZipArchive::new(file)?;

    // Read source.dsl
    let source = read_zip_file(&mut archive, "source.dsl")?;

    // Read output.xml
    let output = read_zip_file(&mut archive, "output.xml")?;

    // Read confirmations.json (always present for backward compat)
    let confirmations_str = read_zip_file(&mut archive, "confirmations.json")?;
    let confirmations: HashMap<u32, LemmaConfirmation> = serde_json::from_str(&confirmations_str)?;

    // Read manifest.json
    let manifest_str = read_zip_file(&mut archive, "manifest.json")?;
    let manifest: ProjectManifest = serde_json::from_str(&manifest_str)?;

    // Read metadata.json (optional, new in v1.1)
    let metadata: Option<Metadata> = match read_zip_file(&mut archive, "metadata.json") {
        Ok(meta_str) => serde_json::from_str(&meta_str).ok(),
        Err(_) => None, // File doesn't exist in older projects
    };

    // Read annotations.json (optional, new in v1.2)
    let annotations: Option<AnnotationSet> = match read_zip_file(&mut archive, "annotations.json") {
        Ok(ann_str) => serde_json::from_str(&ann_str).ok(),
        Err(_) => None, // File doesn't exist in older projects
    };

    Ok(ProjectData {
        source,
        output,
        confirmations,
        manifest,
        metadata,
        annotations,
    })
}

fn read_zip_file<R: Read + std::io::Seek>(
    archive: &mut ZipArchive<R>,
    name: &str,
) -> Result<String> {
    let mut file = archive.by_name(name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Simple ISO 8601 timestamp without external crate
fn chrono_lite_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();
    // Convert to rough ISO format (not perfect but good enough)
    let days_since_epoch = secs / 86400;
    let time_of_day = secs % 86400;
    let hours = time_of_day / 3600;
    let minutes = (time_of_day % 3600) / 60;
    let seconds = time_of_day % 60;

    // Approximate year/month/day calculation
    let mut year = 1970;
    let mut remaining_days = days_since_epoch as i64;

    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }

    let days_in_months: [i64; 12] = if is_leap_year(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 1;
    for days in days_in_months.iter() {
        if remaining_days < *days {
            break;
        }
        remaining_days -= days;
        month += 1;
    }
    let day = remaining_days + 1;

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hours, minutes, seconds
    )
}

fn is_leap_year(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
