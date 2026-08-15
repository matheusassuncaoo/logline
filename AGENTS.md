# AGENTS.md

This file guides AI coding agents working on LogLine.

Este arquivo orienta agentes de IA que trabalham no LogLine.

## Project Context

LogLine is the official project name.

LogLine is an early-stage offline-first desktop whiteboard built with Tauri, Rust, React, TypeScript, Vite, and Zustand.

The current product is not a complete canvas editor yet. The implemented foundation includes a desktop shell, workspace creation/listing, board domain types, and local JSON persistence for workspaces and boards.

O LogLine ainda nao e um editor de canvas completo. A base implementada inclui o shell desktop, criacao/listagem de workspaces, tipos de dominio para boards e persistencia local em JSON.

## Working Rules

- Prefer small, focused changes.
- Do not claim features exist unless they are implemented in the repository.
- Preserve the local-first/offline-first direction.
- Keep frontend types aligned with Rust domain structs.
- Keep Tauri command names stable unless the related frontend calls are updated in the same change.
- Do not add compatibility layers without a concrete need.
- Do not commit generated folders such as `node_modules`, `dist`, or `src-tauri/target`.
- Do not commit `.env` files or secrets.

## Architecture Notes

- Frontend entry: `src/main.tsx`.
- App shell: `src/app/App.tsx`.
- Frontend Tauri wrapper: `src/lib/tauri.ts`.
- Frontend shared types: `src/lib/types.ts`.
- Workspace state: `src/stores/workspaceStore.ts`.
- Rust commands and setup: `src-tauri/src/lib.rs`.
- Rust domain types: `src-tauri/src/domain.rs`.
- Local persistence: `src-tauri/src/persistence.rs`.

## Persistence Rules

- Workspace and board data are stored locally through the Rust backend.
- Board data is serialized as JSON.
- Writes should remain atomic where possible.
- Validate IDs and names before writing data.
- Respect schema version fields when changing persisted structures.

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
