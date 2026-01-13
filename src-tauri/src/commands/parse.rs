use crate::annotations::AnnotationSet;
use crate::entities::EntityRegistry;
use crate::errors::{Result, SagaError};
use crate::normalizer::LevelDictionary;
use crate::parser::{Compiler, CompilerConfig, LemmaMapping};
use std::collections::HashMap;

/// Compile DSL input to TEI-XML.
///
/// This command is async to avoid blocking the UI during compilation.
/// The actual compilation runs on a blocking thread pool.
#[allow(clippy::too_many_arguments)]
#[tauri::command(async, rename_all = "camelCase")]
pub async fn compile_dsl(
    input: String,
    template_header: String,
    template_footer: String,
    word_wrap: Option<bool>,
    auto_line_numbers: Option<bool>,
    multi_level: Option<bool>,
    wrap_pages: Option<bool>,
    entities_json: Option<String>,
    normalizer_json: Option<String>,
    entity_mappings_json: Option<String>,
    custom_mappings: Option<HashMap<String, String>>,
    lemma_mappings_json: Option<String>,
    annotations_json: Option<String>,
) -> Result<String> {
    // Move all the work to a blocking thread pool to avoid blocking the UI
    tauri::async_runtime::spawn_blocking(move || -> Result<String> {
        // Load entities if provided
        let mut registry = EntityRegistry::new();
        if let Some(json) = entities_json {
            registry.load_from_str(&json).map_err(SagaError::Validation)?;
        }

        // Load level dictionary if provided
        let mut dictionary = match normalizer_json {
            Some(json) => Some(LevelDictionary::load(&json).map_err(SagaError::Validation)?),
            None => None,
        };

        // Load entity base letter mappings if provided
        if let Some(ref mut dict) = dictionary {
            if let Some(ref json) = entity_mappings_json {
                dict.load_entity_mappings(json).map_err(SagaError::Validation)?;
            }
            // Apply custom mappings (overrides base mappings)
            if let Some(custom) = custom_mappings {
                dict.add_entity_mappings(custom);
            }
        }

        // Parse lemma mappings if provided (keyed by word INDEX)
        let lemma_mappings: HashMap<u32, LemmaMapping> = match lemma_mappings_json {
            Some(json) => serde_json::from_str(&json)
                .map_err(SagaError::Serde)?,
            None => HashMap::new(),
        };

        // Parse annotations if provided
        let annotations: Option<AnnotationSet> = match annotations_json {
            Some(json) => Some(
                serde_json::from_str(&json)
                    .map_err(SagaError::Serde)?,
            ),
            None => None,
        };

        // Configure compiler
        let config = CompilerConfig {
            word_wrap: word_wrap.unwrap_or(false),
            auto_line_numbers: auto_line_numbers.unwrap_or(false),
            multi_level: multi_level.unwrap_or(false),
            wrap_pages: wrap_pages.unwrap_or(false),
        };

        let mut compiler = Compiler::new()
            .with_entities(&registry)
            .with_config(config)
            .with_lemma_mappings(lemma_mappings);

        // Add annotations if available
        if let Some(ref ann) = annotations {
            compiler = compiler.with_annotations(ann);
        }

        // Add dictionary if available
        if let Some(ref dict) = dictionary {
            compiler = compiler.with_dictionary(dict);
        }

        let body = compiler.compile(&input).map_err(SagaError::Parser)?;
        
        // Build result, only adding newlines between non-empty parts
        // This prevents XML starting with \n when header is empty
        let mut result = String::new();
        if !template_header.is_empty() {
            result.push_str(&template_header);
            result.push('\n');
        }
        result.push_str(&body);
        if !template_footer.is_empty() {
            result.push('\n');
            result.push_str(&template_footer);
        }
        Ok(result)
    })
    .await
    .map_err(|e| SagaError::Internal(format!("Compilation task failed: {}", e)))?
}
