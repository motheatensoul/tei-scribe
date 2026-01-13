use libxml::tree::Node;
use super::helpers::*;
use crate::metadata::{
    Availability, DateRange, History, Language, Metadata, MsContents, MsIdentifier, Person,
    PhysDesc, PublicationStmt, RespStmt, TitleStmt,
};

pub fn extract_metadata(root: &Node) -> Option<Metadata> {
    let header = find_element(root, "teiHeader")?;

    let mut metadata = Metadata::default();

    // Extract fileDesc
    if let Some(file_desc) = find_element(&header, "fileDesc") {
        // titleStmt
        if let Some(title_stmt) = find_element(&file_desc, "titleStmt") {
            metadata.title_stmt = extract_title_stmt(&title_stmt);
        }

        // publicationStmt
        if let Some(pub_stmt) = find_element(&file_desc, "publicationStmt") {
            metadata.publication_stmt = extract_publication_stmt(&pub_stmt);
        }

        // sourceDesc -> msDesc
        if let Some(source_desc) = find_element(&file_desc, "sourceDesc") {
            if let Some(ms_desc) = find_element(&source_desc, "msDesc") {
                // msIdentifier
                if let Some(ms_id) = find_element(&ms_desc, "msIdentifier") {
                    metadata.ms_identifier = extract_ms_identifier(&ms_id);
                }

                // msContents
                if let Some(ms_contents) = find_element(&ms_desc, "msContents") {
                    metadata.ms_contents = extract_ms_contents(&ms_contents);
                }

                // physDesc
                if let Some(phys_desc) = find_element(&ms_desc, "physDesc") {
                    metadata.phys_desc = extract_phys_desc(&phys_desc);
                }

                // history
                if let Some(history) = find_element(&ms_desc, "history") {
                    metadata.history = extract_history(&history);
                }
            }
        }
    }

    // Extract profileDesc -> langUsage
    if let Some(profile_desc) = find_element(&header, "profileDesc") {
        if let Some(lang_usage) = find_element(&profile_desc, "langUsage") {
            metadata.languages = extract_languages(&lang_usage);
        }
    }

    // Extract encodingDesc -> editorialDecl -> normalization
    if let Some(encoding_desc) = find_element(&header, "encodingDesc") {
        if let Some(editorial_decl) = find_element(&encoding_desc, "editorialDecl") {
            if let Some(normalization) = find_element(&editorial_decl, "normalization") {
                if let Some(p) = find_element(&normalization, "p") {
                    metadata.encoding_notes = Some(get_text_content(&p));
                }
            }
        }
    }

    // Only return metadata if we found something meaningful
    if metadata.title_stmt.title.is_some()
        || metadata.ms_identifier.repository.is_some()
        || metadata.ms_identifier.idno.is_some()
        || !metadata.languages.is_empty()
    {
        Some(metadata)
    } else {
        None
    }
}

fn extract_title_stmt(node: &Node) -> TitleStmt {
    let mut stmt = TitleStmt::default();

    // Title (first one without type attribute, or just first)
    for child in element_children(node) {
        if child.get_name() == "title" {
            let type_attr = child.get_property("type");
            if type_attr.is_none() || type_attr.as_deref() == Some("main") {
                if stmt.title.is_none() {
                    stmt.title = Some(get_text_content(&child));
                }
            } else if type_attr.as_deref() == Some("sub") {
                stmt.subtitle = Some(get_text_content(&child));
            }
        }
    }

    // Author
    if let Some(author) = find_element(node, "author") {
        stmt.author = Some(get_text_content(&author));
    }

    // Editors
    for child in element_children(node) {
        if child.get_name() == "editor" {
            let name = get_text_content(&child);
            if !name.is_empty() {
                let mut person = Person {
                    name,
                    identifier: None,
                    identifier_type: None,
                };
                // Check for ref attribute (e.g., "orcid:0000-0000-0000-0000")
                if let Some(ref_attr) = child.get_property("ref") {
                    if let Some((id_type, id)) = ref_attr.split_once(':') {
                        person.identifier_type = Some(id_type.to_string());
                        person.identifier = Some(id.to_string());
                    }
                }
                stmt.editors.push(person);
            }
        }
    }

    // respStmt
    for child in element_children(node) {
        if child.get_name() == "respStmt" {
            let resp = find_element(&child, "resp").map(|n| get_text_content(&n));
            let name = find_element(&child, "name").map(|n| get_text_content(&n));
            if let (Some(resp), Some(name)) = (resp, name) {
                if !resp.is_empty() && !name.is_empty() {
                    stmt.resp_stmts.push(RespStmt { resp, name });
                }
            }
        }
    }

    stmt
}

fn extract_publication_stmt(node: &Node) -> PublicationStmt {
    let mut stmt = PublicationStmt::default();

    if let Some(publisher) = find_element(node, "publisher") {
        stmt.publisher = Some(get_text_content(&publisher));
    }

    if let Some(pub_place) = find_element(node, "pubPlace") {
        stmt.pub_place = Some(get_text_content(&pub_place));
    }

    if let Some(date) = find_element(node, "date") {
        // Prefer @when attribute, fall back to text content
        stmt.date = date
            .get_property("when")
            .or_else(|| Some(get_text_content(&date)));
    }

    if let Some(distributor) = find_element(node, "distributor") {
        stmt.distributor = Some(get_text_content(&distributor));
    }

    // idno
    if let Some(idno) = find_element(node, "idno") {
        stmt.idno = Some(get_text_content(&idno));
        stmt.idno_type = idno.get_property("type");
    }

    // availability
    if let Some(avail) = find_element(node, "availability") {
        let availability = Availability {
            status: avail.get_property("status"),
            license: find_element(&avail, "licence").map(|l| get_text_content(&l)),
            license_url: find_element(&avail, "licence").and_then(|l| l.get_property("target")),
            note: find_element(&avail, "p").map(|p| get_text_content(&p)),
        };

        stmt.availability = Some(availability);
    }

    stmt
}

fn extract_ms_identifier(node: &Node) -> MsIdentifier {
    let mut id = MsIdentifier::default();

    if let Some(country) = find_element(node, "country") {
        id.country = Some(get_text_content(&country));
    }

    if let Some(settlement) = find_element(node, "settlement") {
        id.settlement = Some(get_text_content(&settlement));
    }

    if let Some(repository) = find_element(node, "repository") {
        id.repository = Some(get_text_content(&repository));
    }

    if let Some(collection) = find_element(node, "collection") {
        id.collection = Some(get_text_content(&collection));
    }

    if let Some(idno) = find_element(node, "idno") {
        id.idno = Some(get_text_content(&idno));
    }

    if let Some(ms_name) = find_element(node, "msName") {
        id.ms_name = Some(get_text_content(&ms_name));
    }

    // altIdentifier elements
    for child in element_children(node) {
        if child.get_name() == "altIdentifier" {
            if let Some(idno) = find_element(&child, "idno") {
                let id_type = child.get_property("type").unwrap_or_default();
                let idno_text = get_text_content(&idno);
                if !idno_text.is_empty() {
                    id.alt_identifiers.push(crate::metadata::AltIdentifier {
                        id_type,
                        idno: idno_text,
                    });
                }
            }
        }
    }

    id
}

fn extract_ms_contents(node: &Node) -> MsContents {
    let mut contents = MsContents::default();

    if let Some(summary) = find_element(node, "summary") {
        contents.summary = Some(get_text_content(&summary));
    }

    if let Some(text_lang) = find_element(node, "textLang") {
        contents.text_lang = text_lang.get_property("mainLang");
        let note = get_text_content(&text_lang);
        if !note.is_empty() && contents.text_lang.as_deref() != Some(&note) {
            contents.text_lang_note = Some(note);
        }
    }

    contents
}

fn extract_phys_desc(node: &Node) -> PhysDesc {
    let mut desc = PhysDesc::default();

    // objectDesc -> supportDesc
    if let Some(object_desc) = find_element(node, "objectDesc") {
        if let Some(support_desc) = find_element(&object_desc, "supportDesc") {
            if let Some(support) = find_element(&support_desc, "support") {
                desc.material = Some(get_text_content(&support));
            }
            if let Some(extent) = find_element(&support_desc, "extent") {
                desc.extent = Some(get_text_content(&extent));
            }
            if let Some(foliation) = find_element(&support_desc, "foliation") {
                desc.foliation = Some(get_text_content(&foliation));
            }
            if let Some(condition) = find_element(&support_desc, "condition") {
                desc.condition = Some(get_text_content(&condition));
            }
        }
        if let Some(layout_desc) = find_element(&object_desc, "layoutDesc") {
            if let Some(layout) = find_element(&layout_desc, "layout") {
                desc.layout = Some(get_text_content(&layout));
            }
        }
    }

    // handDesc
    if let Some(hand_desc) = find_element(node, "handDesc") {
        for child in element_children(&hand_desc) {
            if child.get_name() == "handNote" {
                let hand = crate::metadata::HandNote {
                    id: child.get_property("xml:id").or_else(|| child.get_property("id")),
                    script: child.get_property("script"),
                    scope: child.get_property("scope"),
                    description: Some(get_text_content(&child)),
                };
                desc.hands.push(hand);
            }
        }
    }

    desc
}

fn extract_history(node: &Node) -> History {
    let mut history = History::default();

    if let Some(origin) = find_element(node, "origin") {
        if let Some(orig_date) = find_element(&origin, "origDate") {
            let date_range = DateRange {
                display: Some(get_text_content(&orig_date)),
                when: orig_date.get_property("when"),
                not_before: orig_date.get_property("notBefore"),
                not_after: orig_date.get_property("notAfter"),
            };
            history.orig_date = Some(date_range);
        }

        if let Some(orig_place) = find_element(&origin, "origPlace") {
            history.orig_place = Some(get_text_content(&orig_place));
        }
    }

    if let Some(provenance) = find_element(node, "provenance") {
        history.provenance = Some(get_text_content(&provenance));
    }

    if let Some(acquisition) = find_element(node, "acquisition") {
        history.acquisition = Some(get_text_content(&acquisition));
    }

    history
}

fn extract_languages(node: &Node) -> Vec<Language> {
    let mut languages = Vec::new();

    for child in element_children(node) {
        if child.get_name() == "language" {
            if let Some(ident) = child.get_property("ident") {
                let usage = child
                    .get_property("usage")
                    .and_then(|s| s.parse::<u8>().ok());
                let name = get_text_content(&child);
                languages.push(Language {
                    ident,
                    usage,
                    name: if name.is_empty() { None } else { Some(name) },
                });
            }
        }
    }

    languages
}
