pub mod dsl;
pub mod helpers;
pub mod metadata;

use crate::metadata::Metadata;
use libxml::parser::Parser;

/// Result of parsing a TEI-XML file
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportResult {
    /// The DSL content extracted from the body
    pub dsl: String,
    /// Metadata extracted from teiHeader (if present)
    pub metadata: Option<Metadata>,
}

/// Parses TEI-XML content and converts it to Saga-Scribe DSL, also extracting metadata
pub fn parse(xml_content: &str) -> Result<ImportResult, String> {
    let parser = Parser::default();

    // Attempt to parse
    let doc = parser
        .parse_string(xml_content)
        .map_err(|e| format!("Failed to parse XML: {}", e))?;

    let root = doc
        .get_root_element()
        .ok_or("No root element found")?;

    // Extract metadata from teiHeader
    let metadata = metadata::extract_metadata(&root);

    // Find the <body> element specifically to avoid importing header metadata
    let body = dsl::find_body(&root).ok_or("No <body> element found in XML")?;

    let mut output = String::new();
    // Only process children of body
    dsl::process_children(&body, &mut output)?;

    // Trim output to avoid massive trailing/leading whitespace from XML structure
    let trimmed = output.trim();

    Ok(ImportResult {
        dsl: trimmed.to_string(),
        metadata,
    })
}

/// Legacy function for backward compatibility - just returns DSL
#[allow(dead_code)]
pub fn parse_dsl_only(xml_content: &str) -> Result<String, String> {
    parse(xml_content).map(|r| r.dsl)
}
