# LogLine Showcase

LogLine is an offline-first desktop whiteboard for people who want to organize ideas, flows, maps, and decisions without depending on a network connection.

LogLine e um whiteboard desktop offline-first para pessoas que querem organizar ideias, fluxos, mapas e decisoes sem depender de conexao com a rede.

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

O MVP atual ja inclui o primeiro ciclo utilizavel de edicao:

- Criar workspaces locais.
- Criar, listar e abrir boards dentro de um workspace.
- Editar boards em um canvas SVG.
- Adicionar sticky notes, texto, formas, frames, conectores e desenho livre.
- Selecionar, mover, redimensionar, rotacionar, duplicar, excluir, agrupar e reordenar elementos.
- Usar undo/redo durante uma sessao de board ativa.
- Salvar alteracoes localmente com autosave.

## Product Phases

- Phase 0: Desktop/local-first foundation. Tauri shell, Rust persistence, workspace schema, board schema, and atomic writes.
- Phase 1: Workspaces and boards. Workspace landing screen, workspace view, board creation, board listing, and board opening.
- Phase 2: Canvas MVP. SVG canvas, editing tools, element operations, undo/redo, and local autosave.
- Phase 3: Assets, import/export, and recovery. Backend groundwork exists; user-facing flows are still planned.
- Phase 4: Quality and release readiness. Tests, accessibility, responsive polish, packaging, signed builds, and release automation.

## Fases Do Produto

- Fase 0: Fundacao desktop/local-first. Shell Tauri, persistencia Rust, schema de workspace, schema de board e escrita atomica.
- Fase 1: Workspaces e boards. Tela inicial de workspaces, tela de workspace, criacao de boards, listagem de boards e abertura de boards.
- Fase 2: Canvas MVP. Canvas SVG, ferramentas de edicao, operacoes em elementos, undo/redo e autosave local.
- Fase 3: Assets, importacao/exportacao e recuperacao. A base backend existe; os fluxos visuais para usuario ainda estao planejados.
- Fase 4: Qualidade e preparo para release. Testes, acessibilidade, refinamento responsivo, empacotamento, builds assinados e automacao de releases.

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

Estas sao direcoes de produto, nao promessas de funcionalidades prontas.

- Image asset UI.
- Workspace import/export UI for portable `.logline` files.
- Visible recovery and backup flows.
- Search inside boards.
- More precise connector behavior.
- Better text editing and element inspectors.
- Keyboard shortcut reference.
- Accessibility improvements.
- Automated tests.
- Packaged releases.

## Portfolio Summary

LogLine demonstrates a pragmatic desktop application architecture with React and Tauri, using Rust for local persistence and TypeScript for the user interface. The current codebase now includes a functional SVG canvas MVP instead of only a persistence foundation.

O LogLine demonstra uma arquitetura desktop pragmatica com React e Tauri, usando Rust para persistencia local e TypeScript para a interface. O codigo atual ja inclui um MVP funcional de canvas SVG, nao apenas uma base de persistencia.

## Short Description

EN: LogLine is an offline-first desktop whiteboard MVP for organizing local workspaces, boards, flows, maps, and decisions.

PT: LogLine e um MVP de whiteboard desktop offline-first para organizar workspaces, boards, fluxos, mapas e decisoes localmente.
