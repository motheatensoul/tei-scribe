use crate::parser::ast::Node;
use crate::parser::compiler::base::Compiler;

impl Compiler<'_> {
    pub(crate) fn compile_word_single(&mut self, children: &[Node]) -> String {
        let mut content = String::new();
        for child in children {
            content.push_str(&self.node_to_xml(child));
        }
        if content.is_empty() {
            String::new()
        } else {
            // Get current word index and increment
            let current_index = self.word_index;
            self.word_index += 1;

            // Lookup by word INDEX (only confirmed instances have mappings)
            let lemma_attrs = self.get_lemma_attributes_by_index(current_index);
            let ann_attrs = self.get_annotation_attributes(current_index);
            let notes = self.get_note_elements(current_index);

            format!("<w{}{}>{}{}</w>\n", lemma_attrs, ann_attrs, content, notes)
        }
    }

    pub(crate) fn compile_punctuation_single(&mut self, children: &[Node]) -> String {
        let mut content = String::new();
        for child in children {
            content.push_str(&self.node_to_xml(child));
        }
        if content.is_empty() {
            String::new()
        } else {
            format!("<pc>{}</pc>\n", content)
        }
    }
}
