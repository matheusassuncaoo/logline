# Changelog

All notable changes to LogLine are documented in this file.

Todas as mudanças relevantes do LogLine são documentadas neste arquivo.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project follows [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Planned

- Improve accessibility, expand test coverage, packaging, and release automation.
- Continue refining the canvas MVP.

## [1.0.0] - 2026-08-14

### Added

- Stable local-first desktop release with Windows MSI and NSIS installers.
- Theme preferences stored locally with system, light, and dark modes.
- Canvas ordering controls, draft interactions, accessible toolbar semantics, and large-board rendering improvements.
- Validation and normalization for imported, recovered, and persisted board element order.
- Frontend tests for export and board operations, plus Rust persistence coverage for recovery and asset flows.

### Changed

- Hardened asset validation, workspace import limits, content security policy, and local application logging.

## [0.1.0] - 2026

### Added

- Tauri desktop application shell.
- Local-first workspace and board persistence.
- Workspace creation and listing.
- Board creation, listing, and opening.
- SVG canvas MVP.
- Sticky notes, text, shapes, frames, connectors, and freehand drawings.
- Selection, drag, resize, rotation, duplication, deletion, ordering, grouping, and ungrouping.
- Session undo/redo and local autosave.
- Local image assets.
- SVG, PNG, and portable workspace export.
- Portable workspace import.
- Atomic writes and journal recovery in persistence.

### Notes

- `0.1.0` is an MVP release line. APIs, persisted data formats, and canvas behavior may evolve before a stable release.
