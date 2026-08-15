# Contributing to LogLine

Thanks for your interest in contributing to LogLine.

Obrigado pelo interesse em contribuir com o LogLine.

LogLine is an early-stage offline-first desktop whiteboard. Contributions should keep the project practical, local-first, and honest about what is already implemented.

O LogLine e um whiteboard desktop offline-first em fase inicial. Contribuicoes devem manter o projeto pratico, local-first e honesto sobre o que ja esta implementado.

## Ways to Contribute

- Report bugs.
- Improve documentation.
- Improve accessibility, layout, and responsiveness.
- Add tests and verification workflows.
- Improve local persistence safety.
- Build the board editor and canvas features.
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
- Do not commit generated folders or local secrets.

## Persistence Guidelines

LogLine stores local data through the Tauri/Rust backend.

When changing persisted structures:

- Consider `schemaVersion` fields.
- Validate IDs and names before writing.
- Keep atomic writes where possible.
- Avoid data loss during migrations.
- Document any breaking local data change clearly.

## Pull Requests

Good pull requests include:

- A clear description of the problem and solution.
- Screenshots or recordings for UI changes when useful.
- Notes about verification commands run.
- Notes about any command that could not be run.
- Documentation updates when behavior changes.

## Commit Style

Use concise commit messages that describe the change:

```text
Add workspace empty state
Fix board persistence validation
Document Tauri development setup
```

## Bug Reports

Useful bug reports include:

- What happened.
- What you expected.
- Steps to reproduce.
- Operating system.
- Whether it happens in `npm run dev`, `npm run tauri dev`, or a packaged build.
- Logs or screenshots when relevant.

## Feature Requests

Useful feature requests include:

- The workflow you want to support.
- Why it matters for an offline-first whiteboard.
- Whether it affects frontend, backend, persistence, or packaging.
- Any risks around local data or compatibility.

## Tests

There is no dedicated automated test suite configured yet. Until one is added, use the most relevant build/check commands and describe manual verification in PRs.

Ainda nao existe uma suite automatizada de testes configurada. Ate que ela exista, use os comandos de build/check mais relevantes e descreva a verificacao manual nos PRs.
