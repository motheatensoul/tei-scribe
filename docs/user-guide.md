# Saga-Scribe User Guide

This guide explains how to use Saga-Scribe for manuscript transcription.

## Getting Started

Saga-Scribe provides a split-pane interface:
- **Left pane**: DSL Editor for entering your transcription
- **Right pane**: Preview showing rendered text or raw XML output

### Basic Workflow

1. Select a template from the toolbar dropdown
2. Type your transcription using the DSL syntax
3. Click words in the rendered view to add lemma annotations
4. Save your project with Ctrl+S
5. Export the TEI-XML when ready for publication

### Project Files

Saga-Scribe uses `.teis` project archives (ZIP format) that bundle:
- Your DSL source text
- Compiled TEI-XML output
- Per-word lemma confirmations
- Project metadata

This means you never lose your lemmatization work when saving and reopening projects.

## Importing Files

Saga-Scribe can import transcriptions from other formats, converting them to DSL for editing.

### Import from TEI-XML

1. Click **File > Import** or use the import button in the toolbar
2. Select a `.xml` file containing TEI-encoded text
3. The importer extracts text content from the `<body>` element
4. Line breaks (`<lb/>`) are converted to `//` syntax
5. Page breaks (`<pb/>`) are converted to `///` syntax with folio numbers

**Note**: Complex TEI markup is simplified during import. Review the result and add DSL markup as needed.

### Import from Plain Text

1. Click **File > Import** or use the import button in the toolbar
2. Select a `.txt` file
3. The text is imported directly into the editor
4. Add DSL markup manually for line breaks, page breaks, and editorial marks

### Import Behavior

- Importing replaces the current editor content
- A confirmation dialog appears if you have unsaved changes
- Session lemmatization data is cleared on import (start fresh with the new text)

## Settings

Click the ⚙️ (gear) icon in the toolbar to open the Settings dialog.

### Appearance Settings

- **Theme**: Choose between Light, Dark, or System color schemes
  - **System**: Automatically follows your operating system's theme preference
    - On Linux: Uses xdg-desktop-portal (works with GNOME, KDE, XFCE, etc.)
    - On macOS/Windows: Uses native system APIs
  - **Light**: Always use the light theme (caramellatte)
  - **Dark**: Always use the dark theme (coffee)
- **Editor Font Size**: Adjust the font size in the DSL editor (10-24px)

### Editor Settings

- **Auto-preview**: Toggle automatic preview updates as you type
- **Preview Delay**: Set the debounce delay (100-2000ms) before preview updates
  - Only visible when auto-preview is enabled
  - Lower values update faster but may impact performance on large documents

### Default Template

- **Active Template**: Select which template to use by default for new projects
- Changes to the active template take effect immediately

Settings are automatically saved when you click "Save Changes". Click "Reset to Defaults" to restore all settings to their original values.

## Help System

Press **F1** or click the **?** button in the toolbar to open the Help dialog. You can also use **Ctrl+/** (or **Cmd+/** on macOS).

## Template Editor

Click the **⚙** (gear) icon in the editor header to open the Template Editor. Templates define the TEI-XML structure that wraps your transcription.

### Built-in Templates

Saga-Scribe includes two built-in templates:

- **TEI P5**: Standard TEI P5 document structure with basic header
- **Menota**: MENOTA handbook compatible structure with multi-level transcription support

Built-in templates cannot be modified or deleted, but you can clone them to create custom versions.

### Creating a Custom Template

1. Click **+ New Template** in the Template Editor
2. Fill in the template details:
   - **Name**: Display name for the template
   - **Description**: Brief explanation of the template's purpose
   - **Header XML**: TEI-XML content before your transcription (up to and including `<body>`)
   - **Footer XML**: TEI-XML content after your transcription (closing tags)
3. Configure options:
   - **Word wrapping**: Wrap words in `<w>` tags
   - **Auto line numbers**: Add `n="..."` attributes to `<lb/>` elements
   - **Multi-level (MENOTA)**: Enable facs/dipl/norm transcription levels
4. Click **Create Template**

### Editing Templates

- Click the **✏️** icon to edit a custom template
- Click the **🔧** icon on a built-in template to clone and customize it
- Click the **📋** icon to duplicate any template
- Click the **🗑️** icon to delete a custom template

### Template Options Explained

| Option | Effect |
|--------|--------|
| Word wrapping | Each word is wrapped in `<w>` tags, punctuation in `<pc>` tags |
| Auto line numbers | Line breaks (`//`) automatically get sequential numbers |
| Multi-level | Words contain `<me:facs>`, `<me:dipl>`, `<me:norm>` child elements |


The Help dialog contains three tabs:

### Keyboard Shortcuts

A complete reference of all keyboard shortcuts for:
- **Application shortcuts**: Save, Open, Undo/Redo lemmatization
- **Editor shortcuts**: Standard CodeMirror editing commands

| Shortcut | Action |
|----------|--------|
| Ctrl+S | Save project |
| Ctrl+O | Open project |
| Ctrl+Shift+Z | Undo lemmatization |
| Ctrl+Shift+Y | Redo lemmatization |
| F1 | Open help |
| Escape | Close dialog |

### DSL Syntax

A quick reference table showing all DSL syntax elements and their TEI-XML output. Useful when you need to look up the correct notation for a specific element.

### About

Version information, feature summary, and license details.

## DSL Syntax

The Domain-Specific Language (DSL) uses short notation that compiles to TEI-XML.

### Line and Page Breaks

```
//           Line break (auto-numbered if enabled)
//5          Line break with explicit number
///1v        Page break with folio number
```

### Editorial Marks

```
[...]        Gap/lacuna (illegible text)
[...3]       Gap of approximately 3 characters
[...3<ab>]   Gap with supplied reading
<abc>        Supplied/reconstructed text
-{abc}-      Deleted text
+{abc}+      Added text
?{abc}?      Unclear reading
^{note}      Marginal note
```

### Abbreviations

```
.abbr[abbreviation]{expansion}
```

Example: `.abbr[d.]{deus}` produces an abbreviation mark for "d." expanding to "deus".

### Word Boundaries

```
|            Explicit word boundary
~//          Word continues across line break
~///1v       Word continues across page break
```

### Special Characters

Use entity references for medieval characters:

```
:rrot:       Rotunda r
:thorn:      Thorn (þ)
:eth:        Eth (ð)
:aelig:      Ligature ae (æ)
```

## Entity Browser

Click the "ꝥ" button in the editor header to open the Entity Browser.

### Searching Entities

- Type in the search box to filter by name or description
- Use the category dropdown to browse by type
- Click an entity to select it
- Click "+" to insert the entity at cursor position

### Custom Entity Mappings

When you select an entity, you can customize how it normalizes:

1. Select an entity from the list
2. Edit the "Translation" field to set the diplomatic normalization
3. Press Enter or click away to save
4. Click "Reset" to restore the default mapping

Custom mappings appear with an "custom" badge and affect how entities render in the diplomatic and normalized levels.

## Lemmatizer

Click any word in the rendered text view to open the Lemmatizer.

### Adding Lemma Mappings

1. Click a word in the rendered text
2. Search for the lemma in the ONP dictionary
3. Select the correct headword from results
4. Select the word class (noun, verb, adjective, etc.)
5. Fill in the relevant morphological fields that appear
6. Click "Save"

### Editing Existing Mappings

When you click an existing mapping from the list:
- The form fields populate with the saved values
- You can modify the word class, morphology, or normalized form
- Click "Save" to update the mapping

### Viewing Lemma Info

- **Confirmed words** (green border): This specific word instance has a confirmed lemma
- **Suggested words** (faded border): The dictionary knows this wordform but it hasn't been confirmed for this instance
- Hover over a word to see its lemma and analysis in the tooltip

### MENOTA Morphological Analysis Format (me:msa)

The Lemmatizer uses the MENOTA me:msa format for morphological tagging. Analyses are stored as space-separated name tokens.

#### Word Classes

| Code | Meaning |
|------|---------|
| xNC | Common noun |
| xNP | Proper noun |
| xAJ | Adjective |
| xVB | Verb |
| xPE | Personal pronoun |
| xPR | Reflexive pronoun |
| xPQ | Interrogative pronoun |
| xPI | Indefinite pronoun |
| xDD | Demonstrative determiner |
| xDQ | Quantifying determiner |
| xDP | Possessive determiner |
| xAV | Adverb |
| xAQ | Interrogative adverb |
| xAP | Preposition |
| xCC | Coordinating conjunction |
| xCS | Subordinating conjunction |
| xIT | Interjection |
| xIM | Infinitive marker |

#### Morphological Categories

| Category | Values |
|----------|--------|
| Case | cN (nom), cG (gen), cD (dat), cA (acc) |
| Number | nS (singular), nD (dual), nP (plural) |
| Gender | gM (masc), gF (fem), gN (neut) |
| Definiteness | sI (indefinite), sD (definite) |
| Grade | rP (positive), rC (comparative), rS (superlative) |
| Person | p1, p2, p3 |
| Tense | tPS (present), tPT (preterite) |
| Mood | mIN (indicative), mSU (subjunctive), mIP (imperative) |
| Voice | vA (active), vR (reflexive/middle) |
| Finiteness | fF (finite), fP (participle), fS (supine), fI (infinitive) |

#### Examples

- `xNC cN nS gF` = common noun, nominative, singular, feminine
- `xVB fF p3 nS tPT mIN vA` = verb, finite, 3rd person, singular, preterite, indicative, active
- `xAJ rC cA nP gM` = adjective, comparative, accusative, plural, masculine

### TEI Output

When you save lemma mappings, the TEI output will include `lemma` and `me:msa` attributes on word elements:

```xml
<w lemma="kona" me:msa="xNC cN nP gF">
  <me:facs>konur</me:facs>
  <me:dipl>konur</me:dipl>
  <me:norm>konur</me:norm>
</w>
```

## Templates

Templates define the TEI-XML header and footer that wrap your content.

### Built-in Templates

- **TEI P5**: Standard TEI with minimal header
- **MENOTA**: Full MENOTA-compliant template with multi-level support

### Template Options

- **Word wrapping**: Wrap words in `<w>` and punctuation in `<pc>` tags
- **Auto line numbers**: Automatically number line breaks
- **Multi-level output**: Generate MENOTA `<me:facs>`, `<me:dipl>`, `<me:norm>` levels

### Creating Custom Templates

1. Click the gear icon in the editor header
2. Click "+ Create Template"
3. Fill in the template fields:
   - Name and description
   - Header XML (before content)
   - Footer XML (after content)
   - Options checkboxes
4. Click "Save"

## Multi-Level Transcription

When using a MENOTA template with multi-level output enabled, each word generates three levels:

- **Facsimile (`me:facs`)**: Exact manuscript appearance with entity references
- **Diplomatic (`me:dipl`)**: Resolved entities, expanded abbreviations
- **Normalized (`me:norm`)**: Standardized spelling

### Example Output

Input:
```
:rrot:egn
```

Output:
```xml
<w>
  <me:facs>&rrot;egn</me:facs>
  <me:dipl>regn</me:dipl>
  <me:norm>regn</me:norm>
</w>
```

## Preview Modes

Toggle between views using the buttons in the preview header:

- **Text**: Rendered view with clickable words for lemmatization
- **XSLT**: Apply custom XSLT stylesheets to transform the XML output
- **XML**: Raw TEI-XML output

### Page Navigation (Text View)

For manuscripts with multiple pages (defined by `///` page breaks), the Text view shows a page navigation bar:

- Use **First/Prev/Next/Last** buttons to navigate between pages
- Use the **dropdown menu** to jump directly to any page
- The status bar shows total word count, token count, and page count

Page navigation enables smooth browsing of large manuscripts without performance issues, as only the visible pages are rendered at a time.

### XSLT Preview Mode

The XSLT tab allows you to apply custom stylesheets to transform your TEI-XML:

1. Click the **XSLT** tab in the preview header
2. Use the **Select XSLT File** button to load a stylesheet (.xsl or .xslt)
3. The XML is automatically transformed and displayed as HTML
4. The selected stylesheet persists during your session

This is useful for:
- Previewing how your TEI will render in different publication formats
- Testing transformations before export
- Applying project-specific display conventions

**Note**: XSLT transformation uses the browser's built-in XSLTProcessor and supports XSLT 1.0.

## Keyboard Shortcuts

### Project

| Shortcut | Action |
|----------|--------|
| Ctrl+S / Cmd+S | Save project (.teis archive) |
| Ctrl+O / Cmd+O | Open project or DSL file |

### Editor

| Shortcut | Action |
|----------|--------|
| Ctrl+F / Cmd+F | Find in editor |
| Ctrl+H / Cmd+H | Find and replace |
| Ctrl+G / Cmd+G | Find next match |
| Ctrl+Shift+G / Cmd+Shift+G | Find previous match |
| Ctrl+Z / Cmd+Z | Undo |
| Ctrl+Y / Cmd+Y | Redo |
| Tab | Indent |
| Shift+Tab | Unindent |

### Folding (Collapsible Sections)

| Shortcut | Action |
|----------|--------|
| Ctrl+Shift+[ / Cmd+Shift+[ | Fold section at cursor |
| Ctrl+Shift+] / Cmd+Shift+] | Unfold section at cursor |

You can also click the fold markers in the gutter (▶/▼) next to page breaks (`///`) to collapse or expand sections.

### Lemmatization

| Shortcut | Action |
|----------|--------|
| Ctrl+Shift+Z / Cmd+Shift+Z | Undo lemmatization |
| Ctrl+Shift+Y / Cmd+Shift+Y | Redo lemmatization |

## Error Panel

Click the "☰" button to view the application log. This shows:
- Loading status for dictionaries and entities
- Errors during parsing or compilation
- Debug information

## Tips

1. **Start simple**: Use basic syntax first, add markup incrementally
2. **Use templates**: Let templates handle the XML boilerplate
3. **Check the preview**: Watch the XML output as you type
4. **Build your dictionary**: Lemmatize words as you encounter them
5. **Customize entities**: Set up diplomatic normalizations for your text's conventions
