# CLAUDE.md

Guidance for Claude Code and similar coding assistants working in this repository.

Orientacoes para Claude Code e assistentes semelhantes trabalhando neste repositorio.

## Summary

LogLine is an official product name. It is an early-stage offline-first desktop whiteboard foundation.

Current implementation:

- Tauri desktop app.
- React workspace landing screen.
- Zustand workspace store.
- Rust commands exposed through Tauri.
- Local JSON persistence for workspaces and boards.

Not fully implemented yet:

- Board editor UI.
- Interactive canvas.
- Element rendering/editing.
- Export/import flows.
- Automated tests.

## Commands

```sh
npm install
npm run dev
npm run tauri dev
npm run build
npm run tauri build
```

## Important Files

- `src/app/App.tsx`: current UI shell for workspaces.
- `src/app/App.module.css`: app shell styling.
- `src/lib/types.ts`: frontend domain types.
- `src/lib/tauri.ts`: frontend API calls to Tauri commands.
- `src/stores/workspaceStore.ts`: workspace state management.
- `src-tauri/src/domain.rs`: Rust domain types.
- `src-tauri/src/lib.rs`: Tauri commands.
- `src-tauri/src/persistence.rs`: local storage implementation.
- `src-tauri/tauri.conf.json`: Tauri app config.

## Coding Guidance

- Keep TypeScript strict-compatible.
- Keep Rust code simple and explicit.
- Preserve Tauri v2 APIs.
- Do not introduce a large abstraction unless multiple call sites need it.
- Prefer CSS Modules for app-specific styling.
- Keep shared frontend types synchronized with Rust serialization names.
- Be careful with persisted schemas and `schemaVersion` fields.

## Product Guidance

- Treat LogLine as local-first by default.
- Do not add network dependencies for core data without explicit direction.
- Avoid implying cloud sync or collaboration exists.
- Keep UX language focused on local workspaces, boards, maps, flows, and decisions.

## Before Finishing

Check the relevant items:

- Did the frontend still build with `npm run build`?
- Did Tauri command names remain aligned between Rust and TypeScript?
- Did persisted data validation remain intact?
- Did documentation describe implemented behavior honestly?
- Were generated files avoided?
