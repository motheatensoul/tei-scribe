use crate::metadata::types::*;

impl Metadata {
    /// Create empty metadata with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if metadata has any meaningful content
    pub fn is_empty(&self) -> bool {
        self.title_stmt.title.is_none()
            && self.title_stmt.author.is_none()
            && self.title_stmt.editors.is_empty()
            && self.ms_identifier.repository.is_none()
            && self.ms_identifier.idno.is_none()
            && self.languages.is_empty()
    }

    /// Generate TEI XML for the teiHeader based on metadata
    pub fn to_tei_header(&self, include_menota_ns: bool) -> String {
        let mut xml = String::new();

        // XML declaration and TEI root
        xml.push_str(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
        xml.push('\n');

        if include_menota_ns {
            xml.push_str(
                r#"<TEI xmlns="http://www.tei-c.org/ns/1.0" xmlns:me="http://www.menota.org/ns/1.0">"#,
            );
        } else {
            xml.push_str(r#"<TEI xmlns="http://www.tei-c.org/ns/1.0">"#);
        }
        xml.push('\n');

        xml.push_str("  <teiHeader>\n");
        xml.push_str("    <fileDesc>\n");

        // titleStmt
        xml.push_str(&self.title_stmt_to_xml());

        // publicationStmt
        xml.push_str(&self.publication_stmt_to_xml());

        // sourceDesc
        xml.push_str(&self.source_desc_to_xml());

        xml.push_str("    </fileDesc>\n");

        // encodingDesc (if notes present)
        if let Some(ref notes) = self.encoding_notes {
            if !notes.is_empty() {
                xml.push_str("    <encodingDesc>\n");
                xml.push_str("      <editorialDecl>\n");
                xml.push_str("        <normalization>\n");
                xml.push_str(&format!("          <p>{}</p>\n", crate::utils::escape_xml(notes)));
                xml.push_str("        </normalization>\n");
                xml.push_str("      </editorialDecl>\n");
                xml.push_str("    </encodingDesc>\n");
            }
        }

        // profileDesc (languages)
        if !self.languages.is_empty() {
            xml.push_str("    <profileDesc>\n");
            xml.push_str("      <langUsage>\n");
            for lang in &self.languages {
                let mut attrs = format!(r#"ident="{}""#, crate::utils::escape_xml(&lang.ident));
                if let Some(usage) = lang.usage {
                    attrs.push_str(&format!(r#" usage="{}""#, usage));
                }
                let content = lang.name.as_deref().unwrap_or(&lang.ident);
                xml.push_str(&format!(
                    "        <language {}>{}</language>\n",
                    attrs,
                    crate::utils::escape_xml(content)
                ));
            }
            xml.push_str("      </langUsage>\n");
            xml.push_str("    </profileDesc>\n");
        }

        xml.push_str("  </teiHeader>\n");
        xml.push_str("  <text>\n");
        xml.push_str("    <body>\n");

        xml
    }

    fn title_stmt_to_xml(&self) -> String {
        let mut xml = String::from("      <titleStmt>\n");

        // Title (required in TEI, use placeholder if empty)
        let title = self
            .title_stmt
            .title
            .as_deref()
            .filter(|t| !t.is_empty())
            .unwrap_or("Untitled");
        xml.push_str(&format!("        <title>{}</title>\n", crate::utils::escape_xml(title)));

        // Subtitle
        if let Some(ref subtitle) = self.title_stmt.subtitle {
            if !subtitle.is_empty() {
                xml.push_str(&format!(
                    "        <title type=\"sub\">{}</title>\n",
                    crate::utils::escape_xml(subtitle)
                ));
            }
        }

        // Author
        if let Some(ref author) = self.title_stmt.author {
            if !author.is_empty() {
                xml.push_str(&format!("        <author>{}</author>\n", crate::utils::escape_xml(author)));
            }
        }

        // Editors
        for editor in &self.title_stmt.editors {
            if !editor.name.is_empty() {
                let mut elem = format!("<editor>{}</editor>", crate::utils::escape_xml(&editor.name));
                if let (Some(ref id), Some(ref id_type)) =
                    (&editor.identifier, &editor.identifier_type)
                {
                    elem = format!(
                        r#"<editor ref="{}:{}">{}</editor>"#,
                        crate::utils::escape_xml(id_type),
                        crate::utils::escape_xml(id),
                        crate::utils::escape_xml(&editor.name)
                    );
                }
                xml.push_str(&format!("        {}\n", elem));
            }
        }

        // Responsibility statements
        for resp in &self.title_stmt.resp_stmts {
            if !resp.name.is_empty() && !resp.resp.is_empty() {
                xml.push_str("        <respStmt>\n");
                xml.push_str(&format!("          <resp>{}</resp>\n", crate::utils::escape_xml(&resp.resp)));
                xml.push_str(&format!("          <name>{}</name>\n", crate::utils::escape_xml(&resp.name)));
                xml.push_str("        </respStmt>\n");
            }
        }

        xml.push_str("      </titleStmt>\n");
        xml
    }

    fn publication_stmt_to_xml(&self) -> String {
        let mut xml = String::from("      <publicationStmt>\n");
        let ps = &self.publication_stmt;

        // Publisher
        if let Some(ref publisher) = ps.publisher {
            if !publisher.is_empty() {
                xml.push_str(&format!(
                    "        <publisher>{}</publisher>\n",
                    crate::utils::escape_xml(publisher)
                ));
            }
        }

        // Publication place
        if let Some(ref place) = ps.pub_place {
            if !place.is_empty() {
                xml.push_str(&format!(
                    "        <pubPlace>{}</pubPlace>\n",
                    crate::utils::escape_xml(place)
                ));
            }
        }

        // Date
        if let Some(ref date) = ps.date {
            if !date.is_empty() {
                xml.push_str(&format!(
                    "        <date when=\"{}\">{}</date>\n",
                    crate::utils::escape_xml(date),
                    crate::utils::escape_xml(date)
                ));
            }
        }

        // Distributor
        if let Some(ref dist) = ps.distributor {
            if !dist.is_empty() {
                xml.push_str(&format!(
                    "        <distributor>{}</distributor>\n",
                    crate::utils::escape_xml(dist)
                ));
            }
        }

        // Identifier
        if let Some(ref idno) = ps.idno {
            if !idno.is_empty() {
                let type_attr = ps
                    .idno_type
                    .as_ref()
                    .map(|t| format!(r#" type="{}""#, crate::utils::escape_xml(t)))
                    .unwrap_or_default();
                xml.push_str(&format!(
                    "        <idno{}>{}</idno>\n",
                    type_attr,
                    crate::utils::escape_xml(idno)
                ));
            }
        }

        // Availability
        if let Some(ref avail) = ps.availability {
            let status_attr = avail
                .status
                .as_ref()
                .map(|s| format!(r#" status="{}""#, crate::utils::escape_xml(s)))
                .unwrap_or_default();
            xml.push_str(&format!("        <availability{}>\n", status_attr));

            if let Some(ref license) = avail.license {
                if !license.is_empty() {
                    let target_attr = avail
                        .license_url
                        .as_ref()
                        .map(|u| format!(r#" target="{}""#, crate::utils::escape_xml(u)))
                        .unwrap_or_default();
                    xml.push_str(&format!(
                        "          <licence{}>{}</licence>\n",
                        target_attr,
                        crate::utils::escape_xml(license)
                    ));
                }
            }

            if let Some(ref note) = avail.note {
                if !note.is_empty() {
                    xml.push_str(&format!("          <p>{}</p>\n", crate::utils::escape_xml(note)));
                }
            }

            xml.push_str("        </availability>\n");
        }

        // Fallback if nothing was added
        if xml == "      <publicationStmt>\n" {
            xml.push_str("        <p>Unpublished transcription</p>\n");
        }

        xml.push_str("      </publicationStmt>\n");
        xml
    }

    fn source_desc_to_xml(&self) -> String {
        let mut xml = String::from("      <sourceDesc>\n");

        // Check if we have manuscript description fields
        let has_ms_desc = self.ms_identifier.repository.is_some()
            || self.ms_identifier.idno.is_some()
            || self.ms_identifier.settlement.is_some()
            || self.ms_contents.summary.is_some()
            || !self.phys_desc.hands.is_empty()
            || self.history.orig_date.is_some();

        if has_ms_desc {
            xml.push_str("        <msDesc>\n");

            // msIdentifier
            xml.push_str("          <msIdentifier>\n");
            if let Some(ref country) = self.ms_identifier.country {
                if !country.is_empty() {
                    xml.push_str(&format!(
                        "            <country>{}</country>\n",
                        crate::utils::escape_xml(country)
                    ));
                }
            }
            if let Some(ref settlement) = self.ms_identifier.settlement {
                if !settlement.is_empty() {
                    xml.push_str(&format!(
                        "            <settlement>{}</settlement>\n",
                        crate::utils::escape_xml(settlement)
                    ));
                }
            }
            if let Some(ref repository) = self.ms_identifier.repository {
                if !repository.is_empty() {
                    xml.push_str(&format!(
                        "            <repository>{}</repository>\n",
                        crate::utils::escape_xml(repository)
                    ));
                }
            }
            if let Some(ref collection) = self.ms_identifier.collection {
                if !collection.is_empty() {
                    xml.push_str(&format!(
                        "            <collection>{}</collection>\n",
                        crate::utils::escape_xml(collection)
                    ));
                }
            }
            if let Some(ref idno) = self.ms_identifier.idno {
                if !idno.is_empty() {
                    xml.push_str(&format!(
                        "            <idno>{}</idno>\n",
                        crate::utils::escape_xml(idno)
                    ));
                }
            }
            if let Some(ref ms_name) = self.ms_identifier.ms_name {
                if !ms_name.is_empty() {
                    xml.push_str(&format!(
                        "            <msName>{}</msName>\n",
                        crate::utils::escape_xml(ms_name)
                    ));
                }
            }
            for alt in &self.ms_identifier.alt_identifiers {
                xml.push_str(&format!(
                    "            <altIdentifier type=\"{}\">\n              <idno>{}</idno>\n            </altIdentifier>\n",
                    crate::utils::escape_xml(&alt.id_type),
                    crate::utils::escape_xml(&alt.idno)
                ));
            }
            xml.push_str("          </msIdentifier>\n");

            // msContents
            if self.ms_contents.summary.is_some() || self.ms_contents.text_lang.is_some() {
                xml.push_str("          <msContents>\n");
                if let Some(ref summary) = self.ms_contents.summary {
                    if !summary.is_empty() {
                        xml.push_str(&format!(
                            "            <summary>{}</summary>\n",
                            crate::utils::escape_xml(summary)
                        ));
                    }
                }
                if let Some(ref lang) = self.ms_contents.text_lang {
                    if !lang.is_empty() {
                        let note = self
                            .ms_contents
                            .text_lang_note
                            .as_deref()
                            .unwrap_or(lang);
                        xml.push_str(&format!(
                            "            <textLang mainLang=\"{}\">{}</textLang>\n",
                            crate::utils::escape_xml(lang),
                            crate::utils::escape_xml(note)
                        ));
                    }
                }
                xml.push_str("          </msContents>\n");
            }

            // physDesc
            if self.phys_desc.material.is_some()
                || self.phys_desc.extent.is_some()
                || !self.phys_desc.hands.is_empty()
            {
                xml.push_str("          <physDesc>\n");

                if self.phys_desc.material.is_some() || self.phys_desc.extent.is_some() {
                    xml.push_str("            <objectDesc>\n");
                    xml.push_str("              <supportDesc>\n");
                    if let Some(ref material) = self.phys_desc.material {
                        if !material.is_empty() {
                            xml.push_str(&format!(
                                "                <support>{}</support>\n",
                                crate::utils::escape_xml(material)
                            ));
                        }
                    }
                    if let Some(ref extent) = self.phys_desc.extent {
                        if !extent.is_empty() {
                            xml.push_str(&format!(
                                "                <extent>{}</extent>\n",
                                crate::utils::escape_xml(extent)
                            ));
                        }
                    }
                    if let Some(ref foliation) = self.phys_desc.foliation {
                        if !foliation.is_empty() {
                            xml.push_str(&format!(
                                "                <foliation>{}</foliation>\n",
                                crate::utils::escape_xml(foliation)
                            ));
                        }
                    }
                    if let Some(ref condition) = self.phys_desc.condition {
                        if !condition.is_empty() {
                            xml.push_str(&format!(
                                "                <condition>{}</condition>\n",
                                crate::utils::escape_xml(condition)
                            ));
                        }
                    }
                    xml.push_str("              </supportDesc>\n");
                    if let Some(ref layout) = self.phys_desc.layout {
                        if !layout.is_empty() {
                            xml.push_str("              <layoutDesc>\n");
                            xml.push_str(&format!(
                                "                <layout>{}</layout>\n",
                                crate::utils::escape_xml(layout)
                            ));
                            xml.push_str("              </layoutDesc>\n");
                        }
                    }
                    xml.push_str("            </objectDesc>\n");
                }

                if !self.phys_desc.hands.is_empty() {
                    xml.push_str("            <handDesc>\n");
                    for hand in &self.phys_desc.hands {
                        let mut attrs = String::new();
                        if let Some(ref id) = hand.id {
                            attrs.push_str(&format!(r#" xml:id="{}""#, crate::utils::escape_xml(id)));
                        }
                        if let Some(ref script) = hand.script {
                            attrs.push_str(&format!(r#" script="{}""#, crate::utils::escape_xml(script)));
                        }
                        if let Some(ref scope) = hand.scope {
                            attrs.push_str(&format!(r#" scope="{}""#, crate::utils::escape_xml(scope)));
                        }
                        let desc = hand.description.as_deref().unwrap_or("");
                        xml.push_str(&format!(
                            "              <handNote{}>{}</handNote>\n",
                            attrs,
                            crate::utils::escape_xml(desc)
                        ));
                    }
                    xml.push_str("            </handDesc>\n");
                }

                xml.push_str("          </physDesc>\n");
            }

            // history
            if self.history.orig_date.is_some()
                || self.history.orig_place.is_some()
                || self.history.provenance.is_some()
            {
                xml.push_str("          <history>\n");

                if self.history.orig_date.is_some() || self.history.orig_place.is_some() {
                    xml.push_str("            <origin>\n");
                    if let Some(ref date) = self.history.orig_date {
                        let mut attrs = String::new();
                        if let Some(ref when) = date.when {
                            attrs.push_str(&format!(r#" when="{}""#, crate::utils::escape_xml(when)));
                        }
                        if let Some(ref nb) = date.not_before {
                            attrs.push_str(&format!(r#" notBefore="{}""#, crate::utils::escape_xml(nb)));
                        }
                        if let Some(ref na) = date.not_after {
                            attrs.push_str(&format!(r#" notAfter="{}""#, crate::utils::escape_xml(na)));
                        }
                        let display = date.display.as_deref().unwrap_or("");
                        xml.push_str(&format!(
                            "              <origDate{}>{}</origDate>\n",
                            attrs,
                            crate::utils::escape_xml(display)
                        ));
                    }
                    if let Some(ref place) = self.history.orig_place {
                        if !place.is_empty() {
                            xml.push_str(&format!(
                                "              <origPlace>{}</origPlace>\n",
                                crate::utils::escape_xml(place)
                            ));
                        }
                    }
                    xml.push_str("            </origin>\n");
                }

                if let Some(ref prov) = self.history.provenance {
                    if !prov.is_empty() {
                        xml.push_str(&format!(
                            "            <provenance>{}</provenance>\n",
                            crate::utils::escape_xml(prov)
                        ));
                    }
                }

                if let Some(ref acq) = self.history.acquisition {
                    if !acq.is_empty() {
                        xml.push_str(&format!(
                            "            <acquisition>{}</acquisition>\n",
                            crate::utils::escape_xml(acq)
                        ));
                    }
                }

                xml.push_str("          </history>\n");
            }

            xml.push_str("        </msDesc>\n");
        } else {
            // Fallback for empty source description
            xml.push_str("        <p>No source description available.</p>\n");
        }

        xml.push_str("      </sourceDesc>\n");
        xml
    }

    /// Generate just the TEI footer (closing tags)
    pub fn tei_footer() -> String {
        String::from(
            r#"    </body>
  </text>
</TEI>"#,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_metadata() {
        let meta = Metadata::new();
        assert!(meta.is_empty());
    }

    #[test]
    fn test_metadata_with_title() {
        let mut meta = Metadata::new();
        meta.title_stmt.title = Some("Test Title".to_string());
        assert!(!meta.is_empty());
    }

    #[test]
    fn test_basic_tei_header_generation() {
        let mut meta = Metadata::new();
        meta.title_stmt.title = Some("Njáls saga".to_string());
        meta.title_stmt.author = Some("Anonymous".to_string());
        meta.ms_identifier.settlement = Some("Reykjavík".to_string());
        meta.ms_identifier.repository = Some("Stofnun Árna Magnússonar".to_string());
        meta.ms_identifier.idno = Some("GKS 2870 4to".to_string());

        let header = meta.to_tei_header(false);

        assert!(header.contains("<title>Njáls saga</title>"));
        assert!(header.contains("<author>Anonymous</author>"));
        assert!(header.contains("<settlement>Reykjavík</settlement>"));
        assert!(header.contains("<repository>Stofnun Árna Magnússonar</repository>"));
        assert!(header.contains("<idno>GKS 2870 4to</idno>"));
    }

    #[test]
    fn test_menota_namespace() {
        let meta = Metadata::new();
        let header = meta.to_tei_header(true);
        assert!(header.contains(r#"xmlns:me="http://www.menota.org/ns/1.0""#));
    }

    #[test]
    fn test_escape_xml_chars() {
        let mut meta = Metadata::new();
        meta.title_stmt.title = Some("Title with <special> & \"chars\"".to_string());

        let header = meta.to_tei_header(false);
        assert!(header.contains("&lt;special&gt;"));
        assert!(header.contains("&amp;"));
        assert!(header.contains("&quot;chars&quot;"));
    }

    #[test]
    fn test_language_declaration() {
        let mut meta = Metadata::new();
        meta.title_stmt.title = Some("Test".to_string());
        meta.languages.push(Language {
            ident: "non".to_string(),
            usage: Some(95),
            name: Some("Old Norse".to_string()),
        });
        meta.languages.push(Language {
            ident: "lat".to_string(),
            usage: Some(5),
            name: Some("Latin".to_string()),
        });

        let header = meta.to_tei_header(false);
        assert!(header.contains(r#"<language ident="non" usage="95">Old Norse</language>"#));
        assert!(header.contains(r#"<language ident="lat" usage="5">Latin</language>"#));
    }

    #[test]
    fn test_serialization_roundtrip() {
        let mut meta = Metadata::new();
        meta.title_stmt.title = Some("Test".to_string());
        meta.ms_identifier.idno = Some("MS 123".to_string());

        let json = serde_json::to_string(&meta).unwrap();
        let restored: Metadata = serde_json::from_str(&json).unwrap();

        assert_eq!(meta, restored);
    }
}
