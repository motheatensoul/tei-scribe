use serde::{Deserialize, Serialize};

/// Complete manuscript metadata covering TEI P5 header elements.
/// All fields are optional to support incremental editing.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    /// Title statement (titleStmt)
    #[serde(default)]
    pub title_stmt: TitleStmt,

    /// Publication statement (publicationStmt)
    #[serde(default)]
    pub publication_stmt: PublicationStmt,

    /// Source description - manuscript identifier (sourceDesc/msDesc/msIdentifier)
    #[serde(default)]
    pub ms_identifier: MsIdentifier,

    /// Manuscript contents summary (msDesc/msContents)
    #[serde(default)]
    pub ms_contents: MsContents,

    /// Physical description (msDesc/physDesc)
    #[serde(default)]
    pub phys_desc: PhysDesc,

    /// History of the manuscript (msDesc/history)
    #[serde(default)]
    pub history: History,

    /// Language profile (profileDesc/langUsage)
    #[serde(default)]
    pub languages: Vec<Language>,

    /// Encoding description notes (encodingDesc)
    #[serde(default)]
    pub encoding_notes: Option<String>,
}

/// Title statement - who created and is responsible for the digital edition
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TitleStmt {
    /// Main title of the transcription
    pub title: Option<String>,

    /// Subtitle or alternative title
    pub subtitle: Option<String>,

    /// Original author of the text (if known)
    pub author: Option<String>,

    /// Modern editor(s) responsible for the transcription
    #[serde(default)]
    pub editors: Vec<Person>,

    /// Other responsibility statements (transcriber, encoder, etc.)
    #[serde(default)]
    pub resp_stmts: Vec<RespStmt>,
}

/// A person with optional identifiers
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    /// Person's name
    pub name: String,

    /// Optional ORCID, VIAF, or other identifier
    pub identifier: Option<String>,

    /// Identifier type (orcid, viaf, etc.)
    pub identifier_type: Option<String>,
}

/// Responsibility statement - who did what
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RespStmt {
    /// What they did (e.g., "Transcription", "Encoding", "Proofreading")
    pub resp: String,

    /// Who did it
    pub name: String,
}

/// Publication information for the digital edition
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PublicationStmt {
    /// Publisher or institution
    pub publisher: Option<String>,

    /// Place of publication
    pub pub_place: Option<String>,

    /// Publication date (ISO format preferred)
    pub date: Option<String>,

    /// Distributor (if different from publisher)
    pub distributor: Option<String>,

    /// License or availability statement
    pub availability: Option<Availability>,

    /// Identifier for the digital edition (DOI, URN, etc.)
    pub idno: Option<String>,

    /// Type of identifier
    pub idno_type: Option<String>,
}

/// License and availability information
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Availability {
    /// Status: "free", "restricted", "unknown"
    pub status: Option<String>,

    /// License name (e.g., "CC BY 4.0")
    pub license: Option<String>,

    /// License URL
    pub license_url: Option<String>,

    /// Additional availability notes
    pub note: Option<String>,
}

/// Manuscript identifier - where the physical manuscript is held
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MsIdentifier {
    /// Country where manuscript is held
    pub country: Option<String>,

    /// City/settlement
    pub settlement: Option<String>,

    /// Institution/repository name
    pub repository: Option<String>,

    /// Collection name (if applicable)
    pub collection: Option<String>,

    /// Shelfmark/call number
    pub idno: Option<String>,

    /// Alternative identifiers
    #[serde(default)]
    pub alt_identifiers: Vec<AltIdentifier>,

    /// Common name for the manuscript (e.g., "Codex Regius")
    pub ms_name: Option<String>,
}

/// Alternative identifier
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AltIdentifier {
    /// Type of identifier (e.g., "former", "catalog")
    pub id_type: String,

    /// The identifier value
    pub idno: String,
}

/// Summary of manuscript contents
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MsContents {
    /// Brief summary of the manuscript contents
    pub summary: Option<String>,

    /// Primary language of the text (ISO 639 code)
    pub text_lang: Option<String>,

    /// Additional language notes
    pub text_lang_note: Option<String>,
}

/// Physical description of the manuscript
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PhysDesc {
    /// Material (parchment, paper, etc.)
    pub material: Option<String>,

    /// Extent (number of leaves, dimensions)
    pub extent: Option<String>,

    /// Foliation description
    pub foliation: Option<String>,

    /// Layout description (columns, lines per page)
    pub layout: Option<String>,

    /// Description of scribal hands
    #[serde(default)]
    pub hands: Vec<HandNote>,

    /// General condition
    pub condition: Option<String>,
}

/// Description of a scribal hand
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HandNote {
    /// Hand identifier (e.g., "H1", "main")
    pub id: Option<String>,

    /// Script type (e.g., "carolingian", "gothic")
    pub script: Option<String>,

    /// Scope (e.g., "major", "minor", "sole")
    pub scope: Option<String>,

    /// Description
    pub description: Option<String>,
}

/// Historical information about the manuscript
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct History {
    /// When the manuscript was created
    pub orig_date: Option<DateRange>,

    /// Where the manuscript was created
    pub orig_place: Option<String>,

    /// Provenance notes
    pub provenance: Option<String>,

    /// How the repository acquired it
    pub acquisition: Option<String>,
}

/// A date or date range
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DateRange {
    /// Displayed date text (e.g., "ca. 1270-1280")
    pub display: Option<String>,

    /// ISO date for not-before (YYYY or YYYY-MM-DD)
    pub not_before: Option<String>,

    /// ISO date for not-after
    pub not_after: Option<String>,

    /// Single date if known exactly
    pub when: Option<String>,
}

/// Language declaration
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Language {
    /// ISO 639 language code (e.g., "non" for Old Norse)
    pub ident: String,

    /// Percentage of text in this language (optional)
    pub usage: Option<u8>,

    /// Human-readable name
    pub name: Option<String>,
}
