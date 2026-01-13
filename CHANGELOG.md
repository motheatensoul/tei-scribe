# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-01-13

### Added
- Created shared frontend utilities in `src/lib/utils/` for entity resolution and XSLT transformations.
- Introduced `src/lib/types/` for shared TypeScript interfaces (entities, template, metadata).
- Added `CHANGELOG.md` and `CONTRIBUTING.md`.

### Changed
- **Major Refactor**: Migrated all frontend stores to Svelte 5 runes pattern (`.svelte.ts`).
- **Architecture**: Decomposed large backend files into modular structures:
    - `src-tauri/src/metadata/` split into `types.rs` and `generator.rs`.
    - `src-tauri/src/importer/` modularized with a dedicated `tei/` submodule.
    - `src-tauri/src/parser/compiler.rs` split into `compiler/` submodule with single/multi-level specialized files.
- Centralized XML escaping in `src-tauri/src/utils/mod.rs`.
- Updated project version to 0.4.0 across all configuration files.
- Improved shell scripts in `package.json` for better compatibility.

### Removed
- Deleted unused `lemmatizationHistory.ts` store.
- Removed empty `lib/index.ts` barrel file.
- Cleaned up redundant `.ts` store files replaced by `.svelte.ts` versions.

## [0.3.0] - 2025-12-20

### Added
- Manuscript metadata editor.
- Advanced TEI header generation.
- Support for custom entity sets.
- XSLT-based rendering in the preview pane.

## [0.2.0] - 2025-11-15

### Added
- MENOTA multi-level transcription support.
- Lemmatization interface with ONP dictionary integration.
- XML schema validation (RelaxNG).
- PDF and Standalone HTML export.

## [0.1.0] - 2025-10-01

### Added
- Initial release with basic DSL editor and TEI-XML export.
- Syntax highlighting for the custom DSL.
- Built-in MENOTA entity set.
