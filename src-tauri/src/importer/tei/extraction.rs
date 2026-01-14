use crate::importer::tei::helpers;
use crate::importer::tei::segments::Segment;
use libxml::tree::{Node, NodeType};
use std::collections::HashMap;

pub struct Extractor {
    next_id: usize,
}

impl Extractor {
    pub fn new() -> Self {
        Self { next_id: 0 }
    }

    fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn extract_segments(&mut self, node: &Node) -> Vec<Segment> {
        let mut segments = Vec::new();
        self.process_node(node, &mut segments);
        segments
    }

    fn process_node(&mut self, node: &Node, segments: &mut Vec<Segment>) {
        match node.get_type() {
            Some(NodeType::ElementNode) => {
                let local_name = helpers::local_name(node);
                match local_name.as_str() {
                    "w" => {
                        segments.push(self.extract_word(node));
                    }
                    "pc" => {
                        segments.push(self.extract_punctuation(node));
                    }
                    "lb" => {
                        segments.push(Segment::LineBreak {
                            id: self.next_id(),
                            attributes: self.extract_attributes(node),
                        });
                    }
                    "pb" => {
                        segments.push(Segment::PageBreak {
                            id: self.next_id(),
                            attributes: self.extract_attributes(node),
                        });
                    }
                    "handShift" => {
                        segments.push(Segment::HandShift {
                            id: self.next_id(),
                            attributes: self.extract_attributes(node),
                        });
                    }
                    "choice" => {
                        segments.push(self.extract_choice(node));
                    }
                    _ => {
                        // Structural element
                        let is_body = local_name == "body";

                        if !is_body {
                            segments.push(Segment::Structural {
                                id: self.next_id(),
                                xml: self.open_tag(node),
                            });
                        }

                        let mut child = node.get_first_child();
                        while let Some(c) = child {
                            self.process_node(&c, segments);
                            child = c.get_next_sibling();
                        }

                        if !is_body {
                            segments.push(Segment::Structural {
                                id: self.next_id(),
                                xml: format!("</{}>", node.get_name()),
                            });
                        }
                    }
                }
            }
            Some(NodeType::TextNode) => {
                let content = node.get_content();
                if content.trim().is_empty() {
                    segments.push(Segment::Whitespace {
                        id: self.next_id(),
                        content,
                    });
                } else {
                    segments.push(Segment::Word {
                        id: self.next_id(),
                        original_xml: helpers::escape_xml_text(&content),
                        attributes: HashMap::new(),
                        dsl_content: content.trim().to_string(),
                        has_inline_lb: false,
                    });
                }
            }
            Some(NodeType::CommentNode) => {
                segments.push(Segment::Structural {
                    id: self.next_id(),
                    xml: format!("<!--{}-->", node.get_content()),
                });
            }
            _ => {}
        }
    }

    fn open_tag(&self, node: &Node) -> String {
        let name = node.get_name();
        let mut tag = format!("<{}", name);
        for (key, value) in node.get_attributes() {
            tag.push_str(&format!(
                " {}=\"{}\"",
                key,
                helpers::escape_xml_attr(&value)
            ));
        }
        tag.push('>');
        tag
    }

    fn extract_attributes(&self, node: &Node) -> HashMap<String, String> {
        node.get_attributes().into_iter().collect()
    }

    fn extract_word(&mut self, node: &Node) -> Segment {
        let attributes = self.extract_attributes(node);
        let original_xml = helpers::serialize_node(node);

        let mut facs_node = None;
        let mut child = node.get_first_child();
        while let Some(c) = child {
            if c.get_type() == Some(NodeType::ElementNode) {
                let name = helpers::local_name(&c);
                if name == "facs" {
                    facs_node = Some(c);
                    break;
                }
            }
            child = c.get_next_sibling();
        }

        let mut dsl_content = String::new();
        let mut has_inline_lb = false;

        if let Some(facs) = facs_node {
            Self::node_to_dsl(&facs, &mut dsl_content, &mut has_inline_lb);
        } else {
            Self::node_to_dsl(node, &mut dsl_content, &mut has_inline_lb);
        }

        Segment::Word {
            id: self.next_id(),
            original_xml,
            attributes,
            dsl_content,
            has_inline_lb,
        }
    }

    fn extract_punctuation(&mut self, node: &Node) -> Segment {
        let original_xml = helpers::serialize_node(node);

        // Look for me:facs child (or direct content)
        let mut facs_node = None;
        let mut child = node.get_first_child();
        while let Some(c) = child {
            if c.get_type() == Some(NodeType::ElementNode) {
                let name = helpers::local_name(&c);
                if name == "facs" {
                    facs_node = Some(c);
                    break;
                }
            }
            child = c.get_next_sibling();
        }

        let mut dsl_content = String::new();
        let mut has_inline_lb = false;

        if let Some(facs) = facs_node {
            Self::node_to_dsl(&facs, &mut dsl_content, &mut has_inline_lb);
        } else {
            Self::node_to_dsl(node, &mut dsl_content, &mut has_inline_lb);
        }

        Segment::Punctuation {
            id: self.next_id(),
            original_xml,
            dsl_content,
        }
    }

    fn extract_choice(&mut self, node: &Node) -> Segment {
        let attributes = self.extract_attributes(node);
        let original_xml = helpers::serialize_node(node);
        let mut dsl_content = String::new();
        let mut has_inline_lb = false;

        Self::node_to_dsl(node, &mut dsl_content, &mut has_inline_lb);

        Segment::Word {
            id: self.next_id(),
            original_xml,
            attributes,
            dsl_content,
            has_inline_lb,
        }
    }

    fn node_to_dsl(node: &Node, output: &mut String, has_inline_lb: &mut bool) {
        let mut child = node.get_first_child();
        while let Some(c) = child {
            match c.get_type() {
                Some(NodeType::TextNode) => {
                    output.push_str(&c.get_content());
                }
                Some(NodeType::ElementNode) => {
                    let name = helpers::local_name(&c);
                    match name.as_str() {
                        "choice" => {
                            let mut abbr_text = String::new();
                            let mut expan_text = String::new();
                            let mut found_abbr = false;
                            let mut found_expan = false;

                            let mut gc = c.get_first_child();
                            while let Some(gcc) = gc {
                                if gcc.get_type() == Some(NodeType::ElementNode) {
                                    let gc_name = helpers::local_name(&gcc);
                                    if gc_name == "abbr" {
                                        found_abbr = true;
                                        Self::node_to_dsl(&gcc, &mut abbr_text, has_inline_lb);
                                    } else if gc_name == "expan" {
                                        found_expan = true;
                                        Self::node_to_dsl(&gcc, &mut expan_text, has_inline_lb);
                                    }
                                }
                                gc = gcc.get_next_sibling();
                            }

                            if found_abbr && found_expan {
                                output.push_str(&format!(".abbr[{}]{{{}}}", abbr_text, expan_text));
                            } else {
                                Self::node_to_dsl(&c, output, has_inline_lb);
                            }
                        }
                        "am" => {
                            let content = c.get_content();
                            if content.starts_with('&') && content.ends_with(';') {
                                output.push(':');
                                output.push_str(&content[1..content.len() - 1]);
                                output.push(':');
                            } else {
                                Self::node_to_dsl(&c, output, has_inline_lb);
                            }
                        }
                        "add" => {
                            output.push_str("+{");
                            Self::node_to_dsl(&c, output, has_inline_lb);
                            output.push_str("}+");
                        }
                        "del" => {
                            output.push_str("-{");
                            Self::node_to_dsl(&c, output, has_inline_lb);
                            output.push_str("}-");
                        }
                        "unclear" => {
                            output.push_str("?{");
                            Self::node_to_dsl(&c, output, has_inline_lb);
                            output.push_str("}?");
                        }
                        "lb" => {
                            *has_inline_lb = true;
                            output.push_str("~//");
                            if let Some(n) = c.get_property("n") {
                                output.push_str(&n);
                            }
                        }
                        "dipl" | "norm" => {
                            // Skip
                        }
                        _ => {
                            Self::node_to_dsl(&c, output, has_inline_lb);
                        }
                    }
                }
                _ => {}
            }
            child = c.get_next_sibling();
        }
    }
}

#[allow(dead_code)]
pub fn extract_segments(body_node: &Node) -> Vec<Segment> {
    let mut extractor = Extractor::new();
    extractor.extract_segments(body_node)
}

pub fn segments_to_dsl(segments: &[Segment]) -> String {
    let mut dsl = String::new();

    for segment in segments {
        match segment {
            Segment::Word { dsl_content, .. } => {
                dsl.push_str(dsl_content);
                dsl.push(' ');
            }
            Segment::Punctuation { dsl_content, .. } => {
                dsl.push_str(dsl_content);
                dsl.push(' ');
            }
            Segment::LineBreak { attributes, .. } => {
                if let Some(n) = attributes.get("n") {
                    dsl.push_str(&format!("//{} ", n));
                } else {
                    dsl.push_str("// ");
                }
            }
            Segment::PageBreak { attributes, .. } => {
                if let Some(n) = attributes.get("n") {
                    dsl.push_str(&format!("///{}\n", n));
                } else {
                    dsl.push_str("///\n");
                }
            }
            _ => {}
        }
    }

    dsl.trim().to_string()
}
