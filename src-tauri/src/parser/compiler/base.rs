use crate::parser::ast::Node;
use crate::annotations::AnnotationSet;
use crate::entities::EntityRegistry;
use crate::normalizer::LevelDictionary;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for the compiler
#[derive(Debug, Clone, Default)]
pub struct CompilerConfig {
    pub word_wrap: bool,
    pub auto_line_numbers: bool,
    pub multi_level: bool,
    /// Wrap page content in <p> tags (TEI requires content in structural elements)
    pub wrap_pages: bool,
}

/// A lemma mapping for a wordform
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LemmaMapping {
    pub lemma: String,
    pub msa: String,
    #[serde(default)]
    pub normalized: Option<String>,
}

/// Compiles DSL input into TEI-XML
pub struct Compiler<'a> {
    pub(crate) entities: Option<&'a EntityRegistry>,
    pub(crate) dictionary: Option<&'a LevelDictionary>,
    /// Lemma mappings by word INDEX (for confirmed instances only)
    pub(crate) lemma_mappings: HashMap<u32, LemmaMapping>,
    /// Full annotation set (for non-lemma annotations)
    pub(crate) annotations: Option<&'a AnnotationSet>,
    pub(crate) config: CompilerConfig,
    pub(crate) line_number: u32,
    /// Current word index counter
    pub(crate) word_index: u32,
    /// Whether we're currently inside a page <p> wrapper
    pub(crate) in_page_paragraph: bool,
}

impl<'a> Compiler<'a> {
    pub fn new() -> Self {
        Self {
            entities: None,
            dictionary: None,
            lemma_mappings: HashMap::new(),
            annotations: None,
            config: CompilerConfig::default(),
            line_number: 0,
            word_index: 0,
            in_page_paragraph: false,
        }
    }

    pub fn with_entities(mut self, registry: &'a EntityRegistry) -> Self {
        self.entities = Some(registry);
        self
    }

    pub fn with_dictionary(mut self, dictionary: &'a LevelDictionary) -> Self {
        self.dictionary = Some(dictionary);
        self
    }

    /// Set lemma mappings by word INDEX (for confirmed word instances)
    pub fn with_lemma_mappings(mut self, mappings: HashMap<u32, LemmaMapping>) -> Self {
        self.lemma_mappings = mappings;
        self
    }

    /// Set full annotation set (for non-lemma annotations)
    pub fn with_annotations(mut self, annotations: &'a AnnotationSet) -> Self {
        self.annotations = Some(annotations);
        self
    }

    pub fn with_config(mut self, config: CompilerConfig) -> Self {
        self.config = config;
        self
    }

    pub fn compile(&mut self, input: &str) -> Result<String, String> {
        let mut lexer = crate::parser::lexer::Lexer::new(input);
        let doc = lexer.parse()?;

        let nodes = if self.config.word_wrap {
            let tokenizer = crate::parser::wordtokenizer::WordTokenizer::new();
            tokenizer.tokenize(doc.nodes)
        } else {
            doc.nodes
        };

        // Reset counters for each compilation
        self.line_number = 0;
        self.word_index = 0;
        self.in_page_paragraph = false;

        let mut output = String::new();
        for node in &nodes {
            output.push_str(&self.node_to_xml(node));
        }

        // Close any open page paragraph at the end
        if self.config.wrap_pages && self.in_page_paragraph {
            output.push_str("</p>\n");
        }

        Ok(output)
    }

    pub(crate) fn node_to_xml(&mut self, node: &Node) -> String {
        match node {
            Node::Text(text) => crate::utils::escape_xml(text),
            Node::LineBreak(n) => {
                self.line_number += 1;
                match n {
                    Some(num) => format!("<lb n=\"{}\"/>\n", crate::utils::escape_xml(num)),
                    None if self.config.auto_line_numbers => {
                        format!("<lb n=\"{}\"/>\n", self.line_number)
                    }
                    None => "<lb/>\n".to_string(),
                }
            }
            Node::PageBreak(n) => {
                let mut result = String::new();
                if self.config.wrap_pages {
                    // Close previous page paragraph if open
                    if self.in_page_paragraph {
                        result.push_str("</p>\n");
                    }
                    // Output page break
                    result.push_str(&format!("<pb n=\"{}\"/>\n", crate::utils::escape_xml(n)));
                    // Start new page paragraph
                    result.push_str("<p>\n");
                    self.in_page_paragraph = true;
                } else {
                    result.push_str(&format!("<pb n=\"{}\"/>\n", crate::utils::escape_xml(n)));
                }
                result
            }
            Node::Abbreviation { abbr, expansion } => {
                format!(
                    "<choice><abbr>{}</abbr><expan>{}</expan></choice>",
                    crate::utils::escape_xml(abbr),
                    crate::utils::escape_xml(expansion)
                )
            }
            Node::Gap { quantity, supplied } => {
                let gap_xml = match quantity {
                    Some(n) => {
                        format!("<gap reason=\"illegible\" quantity=\"{}\" unit=\"chars\"/>", n)
                    }
                    None => "<gap reason=\"illegible\"/>".to_string(),
                };
                // If there's supplied text, output both gap and supplied
                match supplied {
                    Some(text) => format!("{}<supplied>{}</supplied>", gap_xml, crate::utils::escape_xml(text)),
                    None => gap_xml,
                }
            }
            Node::Supplied(text) => format!("<supplied>{}</supplied>", crate::utils::escape_xml(text)),
            Node::Deletion(text) => format!("<del>{}</del>", crate::utils::escape_xml(text)),
            Node::Addition(text) => format!("<add>{}</add>", crate::utils::escape_xml(text)),
            Node::Note(text) => format!("<note>{}</note>", crate::utils::escape_xml(text)),
            Node::Unclear(text) => format!("<unclear>{}</unclear>", crate::utils::escape_xml(text)),
            Node::Entity(name) => self.compile_entity(name),
            Node::WordContinuation => String::new(), // Consumed by word tokenizer
            Node::WordBoundary => String::new(),     // Consumed by word tokenizer
            Node::CompoundJoin => " ".to_string(),   // Space in single-level mode
            Node::Word(children) => self.compile_word(children),
            Node::Punctuation(children) => self.compile_punctuation(children),
        }
    }

    pub(crate) fn compile_entity(&self, name: &str) -> String {
        // Output as XML entity reference &name;
        format!("&{};", name)
    }

    pub(crate) fn normalize_text(&self, text: &str) -> String {
        if let Some(dict) = self.dictionary {
            dict.normalize_text(text)
        } else {
            text.to_string()
        }
    }
}

impl Default for Compiler<'_> {
    fn default() -> Self {
        Self::new()
    }
}
