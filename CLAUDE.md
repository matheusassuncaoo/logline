# CLAUDE.md

Guidance for Claude Code and similar coding assistants working in this repository.

Orientacoes para Claude Code e assistentes semelhantes trabalhando neste repositorio.

## Summary

LogLine is an official product name. It is an early-stage offline-first desktop whiteboard MVP.

Current implementation:

- Tauri desktop app.
- React workspace landing screen.
- Workspace view with board creation, listing, and opening.
- SVG canvas MVP with editing tools.
- Sticky notes, text, shapes, frames, connectors, and freehand drawings.
- Element selection, drag, resize, rotate, duplicate, delete, bring-to-front, group, and ungroup.
- Undo/redo for active board sessions.
- Debounced local board autosave.
- Zustand workspace store.
- Rust commands exposed through Tauri.
- Local JSON persistence for workspaces and boards.
- Rust persistence groundwork for journals, assets, and portable workspace import/export.

Not fully implemented yet:

- Production-grade board editor polish.
- Image asset UI wired end-to-end.
- User-facing import/export UI.
- Visible backup/recovery flows.
- Automated tests.
- Accessibility pass.
- Release packaging and signed builds.

## Product Phases

- Phase 0: Desktop/local-first foundation. Implemented.
- Phase 1: Workspaces and boards. Implemented as MVP.
- Phase 2: Canvas MVP. Implemented as MVP.
- Phase 3: Assets, import/export, and recovery flows. Backend groundwork exists; UI is still planned.
- Phase 4: Quality, tests, accessibility, packaging, and release polish. Planned.

## Commands

```sh
npm install
npm run dev
npm run tauri dev
npm run build
npm run tauri build
```

## Important Files

- `src/app/App.tsx`: workspace landing and selected workspace entry.
- `src/app/App.module.css`: app shell styling.
- `src/features/workspace/WorkspaceView.tsx`: board list, board creation, board opening, autosave coordination, undo/redo state.
- `src/features/workspace/WorkspaceView.module.css`: workspace view layout.
- `src/features/canvas/Canvas.tsx`: SVG canvas MVP, tools, interactions, element rendering.
- `src/features/canvas/Canvas.module.css`: canvas and element styling.
- `src/lib/types.ts`: frontend domain types.
- `src/lib/tauri.ts`: frontend API calls to Tauri commands.
- `src/stores/workspaceStore.ts`: workspace state management.
- `src-tauri/src/domain.rs`: Rust domain types.
- `src-tauri/src/lib.rs`: Tauri commands.
- `src-tauri/src/persistence.rs`: local storage, journal recovery, assets, and import/export groundwork.
- `src-tauri/tauri.conf.json`: Tauri app config.

## Coding Guidance

- Keep TypeScript strict-compatible.
- Keep Rust code simple and explicit.
- Preserve Tauri v2 APIs.
- Do not introduce a large abstraction unless multiple call sites need it.
- Prefer CSS Modules for app-specific styling.
- Keep shared frontend types synchronized with Rust serialization names.
- Be careful with persisted schemas and `schemaVersion` fields.
- Do not treat backend-only helpers as shipped user features until UI and commands are wired.

## Canvas Guidance

- Preserve the commit boundary used by undo/redo.
- Keep text editing usable; keyboard shortcuts should avoid text inputs.
- Keep panning, zooming, marquee selection, dragging, resizing, and rotation behavior intact when changing interactions.
- Avoid large rewrites of `Canvas.tsx` unless the change is explicitly about canvas architecture.

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
- Did canvas history/autosave behavior remain intact?
- Did documentation describe implemented behavior honestly?
- Were generated files avoided?
