# LogLine

> An offline-first desktop whiteboard for local ideas, maps, flows, and decisions.

[Portugues (Brasil)](README.pt-BR.md) | [Project showcase](SHOWCASE.md) | [Contributing](CONTRIBUTING.md)

## Project Status

LogLine `1.0.0` is the first stable local release. It remains focused on a practical offline-first whiteboard workflow.

The current product has a functional local workflow: create workspaces, create and open boards, edit them in an SVG canvas, and save data locally. The project remains early-stage, so persisted formats and interactions may evolve before a stable release.

## Why LogLine

Many visual thinking tools start with accounts, cloud synchronization, or always-on collaboration. LogLine starts from a different premise: a useful whiteboard should work locally first.

- No account is required for the local workflow.
- Workspace and board data stay on the device.
- Core editing does not depend on a network connection.
- The desktop app uses Tauri, React, TypeScript, and Rust.

## Features

### Workspaces And Boards

- Create and list local workspaces.
- Open a workspace and manage its board list.
- Create, list, and open boards.
- Persist workspace and board metadata as local JSON.

### Canvas MVP

- SVG-based canvas with pan and zoom.
- Sticky notes, text, shapes, frames, connectors, and freehand drawings.
- Selection and marquee selection.
- Drag, resize, and rotation.
- Duplicate, delete, bring-to-front, group, and ungroup operations.
- Keyboard shortcuts for common editing actions.
- Session-based undo and redo.
- Debounced local autosave.

### Local Assets And Export

- Local image assets for boards.
- Board export in SVG and PNG formats.
- Portable workspace export.
- Atomic board writes and journal-based recovery in the Rust persistence layer.

Import portable `.logline` workspace files through the workspace landing screen.

## Project Phases

| Phase | Scope | Status |
| --- | --- | --- |
| 0 | Desktop and local-first foundation | Implemented |
| 1 | Workspaces and boards | MVP implemented |
| 2 | Canvas editor | MVP implemented |
| 3 | Assets, import/export, and recovery | Local MVP implemented |
| 4 | Test coverage, accessibility, packaging, and release polish | Planned |

These phases describe direction, not release promises.

## Technology

| Layer | Technology |
| --- | --- |
| Desktop shell | Tauri 2 |
| Backend | Rust 2021 |
| Interface | React 18 and TypeScript |
| Frontend tooling | Vite |
| Client state | Zustand |
| Canvas | SVG |
| Styling | CSS Modules |
| Local data | JSON managed by the Rust backend |

## Requirements

- Node.js compatible with Vite and TypeScript.
- npm.
- Rust toolchain.
- Tauri system dependencies for your operating system.

See the [Tauri prerequisites](https://tauri.app/start/prerequisites/) for operating-system setup details.

## Getting Started

Clone the repository and install dependencies:

```sh
npm install
```

Start the frontend only:

```sh
npm run dev
```

Start the desktop application:

```sh
npm run tauri dev
```

Build the frontend:

```sh
npm run build
```

Build the desktop application:

```sh
npm run tauri build
```

## Testing

Run frontend tests:

```sh
npm test
```

Vitest currently covers SVG export behavior. Rust persistence tests cover board lifecycle operations, journal recovery, asset deduplication, and portable workspace import/export.

Run Rust tests from `src-tauri`:

```sh
cargo test
```

## Keyboard Shortcuts

| Shortcut | Action |
| --- | --- |
| `V` | Select tool |
| `N` | Add sticky note |
| `T` | Add text |
| `R` | Add shape |
| `Ctrl/Cmd + Z` | Undo |
| `Ctrl/Cmd + Shift + Z` | Redo |
| `Ctrl/Cmd + D` | Duplicate selection |
| `Delete` or `Backspace` | Delete selection |
| `Space` + drag | Pan canvas |

Shortcuts do not apply while typing in a text input or textarea.

## Project Structure

```text
.
├── src/
│   ├── app/                  Application shell and workspace entry
│   ├── features/canvas/      SVG canvas, tools, and canvas styling
│   ├── features/workspace/   Workspace view and board workflow
│   ├── lib/                  Shared types and Tauri API wrapper
│   ├── stores/               Zustand state
│   └── styles/               Global styles and design tokens
├── src-tauri/
│   └── src/
│       ├── domain.rs         Persisted domain types
│       ├── lib.rs            Tauri commands and app setup
│       └── persistence.rs    Local data, assets, export, recovery
├── CONTRIBUTING.md
└── LICENSE
```

## Local Data And Privacy

LogLine is designed to store data locally. The Rust backend uses the application local-data directory provided by Tauri and maintains a `workspaces` directory there.

Workspace data includes JSON board files. The persistence layer also manages local assets and journals used to recover interrupted board writes.

Do not commit generated data, `node_modules`, `dist`, `src-tauri/target`, `.env`, or `.env.*` files.

## Contributing

Contributions are welcome, especially in the current quality and product phases:

- Canvas interaction improvements.
- Accessibility and responsive behavior.
- Expanded automated test coverage.
- Visible recovery and backup flows.
- Packaging and release automation.
- Documentation and bug reports.

Read [CONTRIBUTING.md](CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) before participating.

## Support And Security

- Read [SUPPORT.md](SUPPORT.md) for questions, bug reports, and feature requests.
- Read [SECURITY.md](SECURITY.md) to report a security issue privately.

## Trademark

LogLine is the official project name. See [TRADEMARK.md](TRADEMARK.md) for rules covering forks, modified builds, and brand use.

## License

Copyright (c) 2026 Matheus Assunção da Cunha.

LogLine is licensed under the [MIT License](LICENSE).
