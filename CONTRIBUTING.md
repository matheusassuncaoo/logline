# Contributing to LogLine

Thanks for your interest in contributing to LogLine.

Obrigado pelo interesse em contribuir com o LogLine.

LogLine is an early-stage offline-first desktop whiteboard MVP. Contributions should keep the project practical, local-first, and honest about what is already implemented.

O LogLine é um MVP de whiteboard desktop offline-first em fase inicial. Contribuições devem manter o projeto prático, local-first e honesto sobre o que já está implementado.

## Project Phases

- Phase 0: Desktop/local-first foundation. Implemented.
- Phase 1: Workspaces and boards. Implemented as MVP.
- Phase 2: Canvas MVP. Implemented as MVP.
- Phase 3: Assets, import/export, and recovery flows. Local MVP implemented.
- Phase 4: Quality, tests, accessibility, packaging, and release polish. Planned.

## Ways to Contribute

- Report bugs.
- Improve documentation.
- Improve accessibility, layout, and responsiveness.
- Add tests and verification workflows.
- Improve canvas editing behavior.
- Improve local persistence safety.
- Improve image asset behavior and board export.
- Improve workspace import validation and feedback.
- Improve packaging and release readiness.
- Refactor carefully when it removes real complexity.

## Before You Start

- Read `README.md`.
- Read `CODE_OF_CONDUCT.md`.
- Read `TRADEMARK.md` if you plan to publish forks, builds, screenshots, or modified versions.
- Check existing issues or discussions when available.

## Local Setup

Install dependencies:

```sh
npm install
```

Run the frontend dev server:

```sh
npm run dev
```

Run the desktop app:

```sh
npm run tauri dev
```

Build the frontend:

```sh
npm run build
```

Build the desktop app:

```sh
npm run tauri build
```

## Development Guidelines

- Keep changes small and focused.
- Prefer clear code over clever code.
- Preserve TypeScript strictness.
- Keep Rust validation and persistence errors explicit.
- Keep frontend and backend domain types aligned.
- Do not introduce cloud, sync, telemetry, or network behavior without a clear issue or maintainer decision.
- Do not document backend-only helpers as complete user-facing features.
- Do not commit generated folders or local secrets.

## Canvas Guidelines

The canvas MVP is currently centered in `src/features/canvas/Canvas.tsx`.

When changing canvas behavior:

- Preserve selection, marquee selection, panning, zooming, dragging, resizing, and rotation.
- Preserve undo/redo commit boundaries.
- Preserve local autosave behavior in `WorkspaceView.tsx`.
- Avoid keyboard shortcuts that interfere with text inputs.
- Keep existing element kinds working: `sticky-note`, `text`, `shape`, `connector`, `frame`, `freehand`, and reserved `image`.
- Add focused tests or manual verification notes when tests are not available.

## Persistence Guidelines

LogLine stores local data through the Tauri/Rust backend.

When changing persisted structures:

- Consider `schemaVersion` fields.
- Validate IDs and names before writing.
- Keep atomic writes where possible.
- Preserve journal recovery unless a safer replacement is implemented.
- Avoid data loss during migrations.
- Document any breaking local data change clearly.

Local image assets, workspace import, and export are implemented user-facing flows.

## Pull Requests

Good pull requests include:

- A clear description of the problem and solution.
- Screenshots or recordings for UI/canvas changes when useful.
- Notes about verification commands run.
- Notes about any command that could not be run.
- Documentation updates when behavior changes.
- Manual verification details for interactions that do not have automated tests yet.

## Commit Style

Use concise commit messages that describe the change:

```text
Add board list empty state
Fix canvas resize commit behavior
Document import export phase
```

## Bug Reports

Useful bug reports include:

- What happened.
- What you expected.
- Steps to reproduce.
- Operating system.
- Whether it happens in `npm run dev`, `npm run tauri dev`, or a packaged build.
- Whether local data was newly created or migrated from an earlier build.
- Logs, screenshots, or recordings when relevant.

## Feature Requests

Useful feature requests include:

- The workflow you want to support.
- Why it matters for an offline-first whiteboard.
- Whether it affects frontend, backend, persistence, canvas editing, or packaging.
- Any risks around local data or compatibility.
- How the feature fits Phase 3 or Phase 4 when relevant.

## Tests

There is no dedicated automated test suite configured yet. Until one is added, use the most relevant build/check commands and describe manual verification in PRs.

Ainda não existe uma suíte automatizada de testes configurada. Até que ela exista, use os comandos de build/check mais relevantes e descreva a verificação manual nos PRs.
