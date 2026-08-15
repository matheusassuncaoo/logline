# AGENTS.md

This file guides AI coding agents working on LogLine.

Este arquivo orienta agentes de IA que trabalham no LogLine.

## Project Context

LogLine is the official project name.

LogLine is an early-stage offline-first desktop whiteboard built with Tauri, Rust, React, TypeScript, Vite, and Zustand.

The current product includes a functional canvas MVP, but it is not a complete production-grade whiteboard editor yet. The implemented foundation includes the desktop shell, workspace creation/listing, board creation/listing/opening, SVG canvas editing, local autosave, and local JSON persistence for workspaces and boards.

O LogLine já inclui um MVP funcional de canvas, mas ainda não é um editor de whiteboard completo em nível de produção. A base implementada inclui shell desktop, criação/listagem de workspaces, criação/listagem/abertura de boards, edição em canvas SVG, autosave local e persistência local em JSON.

## Project Phases

- Phase 0: Desktop/local-first foundation. Implemented.
- Phase 1: Workspaces and boards. Implemented as MVP.
- Phase 2: Canvas MVP. Implemented as MVP.
- Phase 3: Assets, import/export, and recovery flows. Local MVP implemented.
- Phase 4: Quality, tests, accessibility, packaging, and release polish. Planned.

## Working Rules

- Prefer small, focused changes.
- Do not claim features exist unless they are implemented in the repository.
- Preserve the local-first/offline-first direction.
- Keep frontend types aligned with Rust domain structs.
- Keep Tauri command names stable unless the related frontend calls are updated in the same change.
- Do not document backend-only groundwork as an end-user feature until it has frontend/Tauri command support.
- Do not add compatibility layers without a concrete need.
- Do not commit generated folders such as `node_modules`, `dist`, or `src-tauri/target`.
- Do not commit `.env` files or secrets.

## Architecture Notes

- Frontend entry: `src/main.tsx`.
- App shell and workspace landing: `src/app/App.tsx`.
- Workspace board UI: `src/features/workspace/WorkspaceView.tsx`.
- Canvas MVP: `src/features/canvas/Canvas.tsx`.
- Canvas styles: `src/features/canvas/Canvas.module.css`.
- Frontend Tauri wrapper: `src/lib/tauri.ts`.
- Frontend shared types: `src/lib/types.ts`.
- Workspace state: `src/stores/workspaceStore.ts`.
- Rust commands and setup: `src-tauri/src/lib.rs`.
- Rust domain types: `src-tauri/src/domain.rs`.
- Local persistence: `src-tauri/src/persistence.rs`.

## Canvas Rules

- Canvas interactions currently live mostly in `Canvas.tsx`; keep changes focused unless a split clearly improves maintainability.
- Preserve selection, undo/redo, autosave, and commit behavior when changing element operations.
- Do not break existing element kinds: `sticky-note`, `text`, `shape`, `connector`, `frame`, `freehand`, and the reserved `image` kind.
- Keep image additions connected to local asset persistence and the active board state.
- Keyboard shortcuts should not interfere with text inputs.

## Persistence Rules

- Workspace and board data are stored locally through the Rust backend.
- Board data is serialized as JSON.
- Writes should remain atomic where possible.
- Journal recovery exists in the persistence layer and should not be removed casually.
- Validate IDs and names before writing data.
- Respect schema version fields when changing persisted structures.
- Image assets, workspace import, and export commands are exposed through the frontend.

## Commands

Install dependencies:

```sh
npm install
```

Run frontend dev server:

```sh
npm run dev
```

Run Tauri app:

```sh
npm run tauri dev
```

Build frontend:

```sh
npm run build
```

Build desktop app:

```sh
npm run tauri build
```

## Verification

Before completing a code change, run the most relevant available command:

- `npm run build` for frontend/type changes.
- `npm run tauri build` for full desktop build changes, when feasible.
- Rust checks from `src-tauri` when backend-only changes are made.

If a command cannot be run, explain why in the final response.

## Documentation Style

- Prefer bilingual PT/EN documentation for public project docs.
- Use clear open source language.
- Be honest about MVP status.
- Avoid marketing claims that are not backed by the current code.
