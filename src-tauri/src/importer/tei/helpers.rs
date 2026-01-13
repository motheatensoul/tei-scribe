use libxml::tree::Node;

/// Find first child element with given name
pub fn find_element(parent: &Node, name: &str) -> Option<Node> {
    element_children(parent).into_iter().find(|child| child.get_name() == name)
}

/// Iterator for element children only
pub fn element_children(parent: &Node) -> Vec<Node> {
    let mut children = Vec::new();
    let mut current = parent.get_first_child();
    while let Some(node) = current {
        if node.get_type() == Some(libxml::tree::NodeType::ElementNode) {
            children.push(node.clone());
        }
        current = node.get_next_sibling();
    }
    children
}

/// Get text content of a node and its descendants
pub fn get_text_content(node: &Node) -> String {
    node.get_content().trim().to_string()
}
