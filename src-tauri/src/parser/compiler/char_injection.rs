use crate::parser::compiler::base::Compiler;

impl Compiler<'_> {
    pub(crate) fn inject_character_tags(&self, xml: &str, word_index: u32) -> String {
        use crate::annotations::{AnnotationType, AnnotationValue, MenotaObservationType};
        
        let Some(ann_set) = self.annotations else {
            return xml.to_string();
        };

        let mut char_anns = Vec::new();
        for ann in ann_set.for_word(word_index) {
            if let (
                AnnotationType::Paleographic,
                crate::annotations::AnnotationTarget::Character { char_start, char_end, .. },
                AnnotationValue::MenotaPaleographic { observation_type: MenotaObservationType::Character, char_type: Some(ctype), .. }
            ) = (&ann.annotation_type, &ann.target, &ann.value) {
                char_anns.push((*char_start, *char_end, ctype));
            }
        }

        if char_anns.is_empty() {
            return xml.to_string();
        }

        char_anns.sort_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));

        let mut result = String::with_capacity(xml.len() + char_anns.len() * 30);
        let mut text_idx = 0;
        let mut chars = xml.chars().peekable();
        
        while let Some(c) = chars.next() {
            if c == '<' {
                result.push(c);
                for tc in chars.by_ref() {
                    result.push(tc);
                    if tc == '>' { break; }
                }
                continue;
            }

            if c == '&' {
                let mut entity = String::from("&");
                while let Some(&ec) = chars.peek() {
                    chars.next();
                    entity.push(ec);
                    if ec == ';' { break; }
                }
                
                self.process_char_injection(&mut result, &entity, text_idx, &char_anns);
                text_idx += 1;
                continue;
            }

            self.process_char_injection(&mut result, &c.to_string(), text_idx, &char_anns);
            text_idx += 1;
        }

        let mut unclosed: Vec<_> = char_anns.iter()
            .filter(|(start, end, _)| *start < text_idx && *end >= text_idx)
            .collect();
        
        unclosed.sort_by(|a, b| b.0.cmp(&a.0));

        for _ in unclosed {
            result.push_str("</c>");
        }

        result
    }

    fn process_char_injection(
        &self, 
        result: &mut String, 
        content: &str, 
        text_idx: u32, 
        anns: &[(u32, u32, &crate::annotations::MenotaCharType)]
    ) {
        use crate::annotations::MenotaCharType;
        
        for (start, _end, ctype) in anns {
            if *start == text_idx {
                let type_str = match ctype {
                    MenotaCharType::Initial => "initial",
                    MenotaCharType::Capital => "capital",
                    MenotaCharType::Rubric => "rubric",
                    MenotaCharType::Colored => "colored",
                };
                result.push_str(&format!("<c type=\"{}\">", type_str));
            }
        }

        result.push_str(content);

        for (_start, end, _) in anns.iter().rev() {
            if *end == text_idx {
                result.push_str("</c>");
            }
        }
    }
}
