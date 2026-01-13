use libxml::tree::{Node, NodeType};
pub fn find_body(node: &Node) -> Option<Node> {
    if node.get_name() == "body" {
        return Some(node.clone());
    }

    let mut child = node.get_first_child();
    while let Some(c) = child {
        if c.get_type() == Some(NodeType::ElementNode) {
            if let Some(found) = find_body(&c) {
                return Some(found);
            }
        }
        child = c.get_next_sibling();
    }
    None
}

pub fn process_node(node: &Node, output: &mut String) -> Result<(), String> {
    match node.get_type() {
        Some(NodeType::ElementNode) => {
            let name = node.get_name();
            match name.as_str() {
                "lb" => {
                    if !output.ends_with('\n') {
                        output.push('\n');
                    }
                    output.push_str("//");
                    if let Some(n) = node.get_property("n") {
                        output.push_str(&n);
                    }
                }
                "pb" => {
                    if !output.ends_with('\n') {
                        output.push('\n');
                    }
                    output.push_str("///");
                    if let Some(n) = node.get_property("n") {
                        output.push_str(&n);
                    }
                }
                "gap" => {
                    output.push_str("[...]");
                }
                "supplied" => {
                    // Extract text from all children (including <ex> which is normally skipped)
                    // This ensures <supplied><ex>text</ex></supplied> becomes <text> not <>
                    let mut content = String::new();
                    extract_text(node, &mut content)?;
                    if !content.trim().is_empty() {
                        output.push('<');
                        output.push_str(&content);
                        output.push('>');
                    }
                }
                "del" => {
                    output.push_str("-{");
                    process_children(node, output)?;
                    output.push_str("}-");
                }
                "add" => {
                    output.push_str("+{");
                    process_children(node, output)?;
                    output.push_str("}+");
                }
                "choice" => {
                    let mut abbr_text = String::new();
                    let mut expan_text = String::new();
                    let mut found_abbr = false;
                    let mut found_expan = false;

                    let mut child = node.get_first_child();
                    while let Some(c) = child {
                        if c.get_type() == Some(NodeType::ElementNode) {
                            let c_name = c.get_name();
                            if c_name == "abbr" {
                                found_abbr = true;
                                extract_text(&c, &mut abbr_text)?;
                            } else if c_name == "expan" {
                                found_expan = true;
                                extract_text(&c, &mut expan_text)?;
                            }
                        }
                        child = c.get_next_sibling();
                    }

                    if found_abbr && found_expan {
                        output.push_str(".abbr[");
                        output.push_str(&abbr_text);
                        output.push_str("]{");
                        output.push_str(&expan_text);
                        output.push('}');
                    } else {
                        process_children(node, output)?;
                    }
                }
                "note" => {
                    output.push_str("^{");
                    process_children(node, output)?;
                    output.push('}');
                }
                "unclear" => {
                    output.push_str("?{");
                    process_children(node, output)?;
                    output.push_str("}?");
                }
                "facs" | "me:facs" => {
                    process_children(node, output)?;
                }
                "dipl" | "norm" | "me:dipl" | "me:norm" => {
                    // Skip to avoid duplication
                }
                "am" => {
                    process_children(node, output)?;
                }
                "ex" => {
                    // Skip (we use facs level)
                }
                "TEI" | "teiHeader" | "text" | "body" | "div" | "p" => {
                    process_children(node, output)?;
                }
                _ => {
                    process_children(node, output)?;
                }
            }
        }
        Some(NodeType::TextNode) => {
            let content = node.get_content();
            if content.trim().is_empty() {
                if !content.contains('\n') {
                    output.push(' ');
                }
            } else {
                let normalized: String = content.split_whitespace().collect::<Vec<&str>>().join(" ");
                let starts_ws = content.chars().next().map(|c| c.is_whitespace()).unwrap_or(false);
                let ends_ws = content.chars().last().map(|c| c.is_whitespace()).unwrap_or(false);

                if starts_ws && !output.ends_with(' ') && !output.ends_with('\n') {
                    output.push(' ');
                }
                output.push_str(&normalized);
                if ends_ws {
                    output.push(' ');
                }
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn process_children(node: &Node, output: &mut String) -> Result<(), String> {
    let mut child = node.get_first_child();
    while let Some(c) = child {
        process_node(&c, output)?;
        child = c.get_next_sibling();
    }
    Ok(())
}

pub fn extract_text(node: &Node, output: &mut String) -> Result<(), String> {
    match node.get_type() {
        Some(NodeType::TextNode) => {
            let content = node.get_content();
            let normalized: String = content.split_whitespace().collect::<Vec<&str>>().join(" ");
            output.push_str(&normalized);
        }
        Some(NodeType::ElementNode) => {
            let mut child = node.get_first_child();
            while let Some(c) = child {
                extract_text(&c, output)?;
                child = c.get_next_sibling();
            }
        }
        _ => {}
    }
    Ok(())
}
