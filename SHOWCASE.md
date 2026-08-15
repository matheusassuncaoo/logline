# LogLine Showcase

LogLine is an offline-first desktop whiteboard for people who want to organize ideas, flows, maps, and decisions without depending on a network connection.

LogLine é um whiteboard desktop offline-first para pessoas que querem organizar ideias, fluxos, mapas e decisões sem depender de conexão com a rede.

## What LogLine Is For

- Product planning.
- Decision mapping.
- Architecture sketches.
- Personal knowledge boards.
- Process mapping.
- Offline workshops.
- Local-first research spaces.

## Current MVP

The current MVP already includes the first usable editing loop:

- Create local workspaces.
- Create, list, and open boards inside a workspace.
- Edit boards on an SVG canvas.
- Add sticky notes, text, shapes, frames, connectors, and freehand drawings.
- Select, move, resize, rotate, duplicate, delete, group, and reorder elements.
- Use undo/redo during an active board session.
- Save board changes locally with autosave.

O MVP atual já inclui o primeiro ciclo utilizável de edição:

- Criar workspaces locais.
- Criar, listar e abrir boards dentro de um workspace.
- Editar boards em um canvas SVG.
- Adicionar sticky notes, texto, formas, frames, conectores e desenho livre.
- Selecionar, mover, redimensionar, rotacionar, duplicar, excluir, agrupar e reordenar elementos.
- Usar undo/redo durante uma sessão de board ativa.
- Salvar alterações localmente com autosave.

## Product Phases

- Phase 0: Desktop/local-first foundation. Tauri shell, Rust persistence, workspace schema, board schema, and atomic writes.
- Phase 1: Workspaces and boards. Workspace landing screen, workspace view, board creation, board listing, and board opening.
- Phase 2: Canvas MVP. SVG canvas, editing tools, element operations, undo/redo, and local autosave.
- Phase 3: Assets, import/export, and recovery. Local MVP implemented.
- Phase 4: Quality and release readiness. Expanded test coverage, accessibility, responsive polish, packaging, signed builds, and release automation.

## Fases Do Produto

- Fase 0: Fundação desktop/local-first. Shell Tauri, persistência Rust, schema de workspace, schema de board e escrita atômica.
- Fase 1: Workspaces e boards. Tela inicial de workspaces, tela de workspace, criação de boards, listagem de boards e abertura de boards.
- Fase 2: Canvas MVP. Canvas SVG, ferramentas de edição, operações em elementos, undo/redo e autosave local.
- Fase 3: Assets, importação/exportação e recuperação. MVP local implementado.
- Fase 4: Qualidade e preparo para release. Ampliação da cobertura de testes, acessibilidade, refinamento responsivo, empacotamento, builds assinados e automação de releases.

## Product Principles

- Local-first by default.
- Offline should be a first-class experience.
- Data should stay understandable and portable.
- The interface should be calm, fast, and direct.
- The product should not require an account to be useful.

## Screenshots

Screenshots are not included yet.

When screenshots are added, prefer:

- Workspace landing screen.
- Workspace board list.
- Board editor with the toolbar visible.
- Canvas with notes, shapes, connectors, frames, and freehand drawings.
- Packaged desktop window on supported operating systems.

## Roadmap Ideas

These are product directions, not promises of completed features.

Estas são direções de produto, não promessas de funcionalidades prontas.

- Visible recovery and backup flows.
- Search inside boards.
- More precise connector behavior.
- Better text editing and element inspectors.
- Keyboard shortcut reference.
- Accessibility improvements.
- Expanded automated test coverage.
- Packaged releases.

## Portfolio Summary

LogLine demonstrates a pragmatic desktop application architecture with React and Tauri, using Rust for local persistence and TypeScript for the user interface. The current codebase now includes a functional SVG canvas MVP instead of only a persistence foundation.

O LogLine demonstra uma arquitetura desktop pragmática com React e Tauri, usando Rust para persistência local e TypeScript para a interface. O código atual já inclui um MVP funcional de canvas SVG, não apenas uma base de persistência.

## Short Description

EN: LogLine is an offline-first desktop whiteboard MVP for organizing local workspaces, boards, flows, maps, and decisions.

PT: LogLine é um MVP de whiteboard desktop offline-first para organizar workspaces, boards, fluxos, mapas e decisões localmente.
