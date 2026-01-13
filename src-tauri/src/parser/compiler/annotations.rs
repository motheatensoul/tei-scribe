use crate::parser::compiler::base::Compiler;

impl Compiler<'_> {
    pub(crate) fn get_lemma_attributes_by_index(&self, word_index: u32) -> String {
        if let Some(mapping) = self.lemma_mappings.get(&word_index) {
            format!(
                " lemma=\"{}\" me:msa=\"{}\"",
                crate::utils::escape_xml(&mapping.lemma),
                crate::utils::escape_xml(&mapping.msa)
            )
        } else {
            String::new()
        }
    }

    pub(crate) fn get_stored_normalized_by_index(&self, word_index: u32) -> Option<String> {
        self.lemma_mappings
            .get(&word_index)
            .and_then(|m| m.normalized.clone())
    }

    pub(crate) fn get_annotation_attributes(&self, word_index: u32) -> String {
        use crate::annotations::{AnnotationType, AnnotationValue, MenotaObservationType};

        let Some(ann_set) = self.annotations else {
            return String::new();
        };

        let mut attrs = String::new();
        let mut ana_values = Vec::new();

        for ann in ann_set.for_word(word_index) {
            match (&ann.annotation_type, &ann.value) {
                (AnnotationType::Semantic, AnnotationValue::Semantic { category, subcategory, .. }) => {
                    let ana = if let Some(sub) = subcategory {
                        format!("#{}:{}", category, sub)
                    } else {
                        format!("#{}", category)
                    };
                    ana_values.push(ana);
                }
                (AnnotationType::Paleographic, AnnotationValue::Paleographic { observation_type, certainty, .. }) => {
                    use crate::annotations::PaleographicType;
                    let paleo_type = match observation_type {
                        PaleographicType::Unclear => "unclear",
                        PaleographicType::Damage => "damage",
                        PaleographicType::Erasure => "erasure",
                        PaleographicType::Letterform => "letterform",
                        PaleographicType::Abbreviation => "abbrev-mark",
                        PaleographicType::Correction => "correction",
                        PaleographicType::Addition => "addition",
                        PaleographicType::Decoration => "decoration",
                        PaleographicType::Other => "paleo",
                    };
                    ana_values.push(format!("#paleo:{}", paleo_type));

                    if let Some(cert) = certainty {
                        let cert_val = if *cert >= 0.8 { "high" } else if *cert >= 0.5 { "medium" } else { "low" };
                        attrs.push_str(&format!(" cert=\"{}\"", cert_val));
                    }
                }
                (AnnotationType::Paleographic, AnnotationValue::MenotaPaleographic { 
                    observation_type, 
                    unclear_reason,
                    add_place,
                    add_type,
                    hand,
                    del_rend,
                    supplied_reason,
                    resp,
                    certainty,
                    ..
                }) => {
                    match observation_type {
                        MenotaObservationType::Unclear => {
                            ana_values.push("#unclear".to_string());
                            if let Some(reason) = unclear_reason {
                                attrs.push_str(&format!(" reason=\"{:?}\"", reason).to_lowercase());
                            }
                            if let Some(cert) = certainty {
                                let cert_val = if *cert >= 0.8 { "high" } else if *cert >= 0.5 { "medium" } else { "low" };
                                attrs.push_str(&format!(" cert=\"{}\"", cert_val));
                            }
                        }
                        MenotaObservationType::Addition => {
                            ana_values.push("#addition".to_string());
                            if let Some(place) = add_place {
                                attrs.push_str(&format!(" place=\"{:?}\"", place).to_lowercase().replace("_", "-"));
                            }
                            if let Some(add_t) = add_type {
                                attrs.push_str(&format!(" type=\"{:?}\"", add_t).to_lowercase());
                            }
                            if let Some(h) = hand {
                                attrs.push_str(&format!(" hand=\"{}\"", h));
                            }
                        }
                        MenotaObservationType::Deletion => {
                            ana_values.push("#deletion".to_string());
                            if let Some(rend) = del_rend {
                                attrs.push_str(&format!(" rend=\"{:?}\"", rend).to_lowercase());
                            }
                            if let Some(h) = hand {
                                attrs.push_str(&format!(" hand=\"{}\"", h));
                            }
                        }
                        MenotaObservationType::Supplied => {
                            ana_values.push("#supplied".to_string());
                            if let Some(reason) = supplied_reason {
                                attrs.push_str(&format!(" reason=\"{:?}\"", reason).to_lowercase());
                            }
                            if let Some(r) = resp {
                                attrs.push_str(&format!(" resp=\"{}\"", r));
                            }
                        }
                        MenotaObservationType::Character => {}
                    }
                }
                _ => {}
            }
        }

        if !ana_values.is_empty() {
            attrs.push_str(&format!(" ana=\"{}\"", ana_values.join(" ")));
        }

        attrs
    }

    pub(crate) fn get_note_elements(&self, word_index: u32) -> String {
        use crate::annotations::{AnnotationType, AnnotationValue};

        let Some(ann_set) = self.annotations else {
            return String::new();
        };

        let mut notes = String::new();

        for ann in ann_set.for_word(word_index) {
            if let (AnnotationType::Note, AnnotationValue::Note { text, category }) =
                (&ann.annotation_type, &ann.value)
            {
                let type_attr = if let Some(cat) = category {
                    format!(" type=\"{}\"", crate::utils::escape_xml(cat))
                } else {
                    String::new()
                };
                notes.push_str(&format!(
                    "<note{}>{}</note>",
                    type_attr,
                    crate::utils::escape_xml(text)
                ));
            }
        }

        notes
    }
}
