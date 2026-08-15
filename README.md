# LogLine

LogLine is an early-stage offline-first desktop whiteboard for organizing local workspaces, boards, flows, maps, and decisions.

LogLine e um whiteboard desktop offline-first em fase inicial para organizar workspaces, boards, fluxos, mapas e decisoes localmente.

## Status

This project is in `0.1.0` and should be treated as an MVP. The product now includes a functional board/canvas MVP, but it is not a complete production-grade whiteboard editor yet.

Este projeto esta na versao `0.1.0` e deve ser tratado como MVP. O produto ja inclui um MVP funcional de board/canvas, mas ainda nao e um editor de whiteboard completo em nivel de producao.

## Current Features

- Desktop shell powered by Tauri 2.
- Local workspace creation and listing.
- Workspace view with board creation, board listing, and board opening.
- SVG canvas MVP for local board editing.
- Canvas elements: sticky notes, text, shapes, frames, connectors, and freehand drawings.
- Selection, marquee selection, drag, resize, rotate, duplicate, delete, bring-to-front, group, and ungroup.
- Keyboard shortcuts for common editing actions.
- Undo/redo history inside the active board session.
- Debounced local autosave for boards.
- Local JSON persistence for workspaces and boards.
- Atomic board writes with journal recovery support in the Rust persistence layer.
- Local image assets for boards.
- SVG, PNG, and portable workspace export.

## Recursos Atuais

- Shell desktop com Tauri 2.
- Criacao e listagem de workspaces locais.
- Tela de workspace com criacao, listagem e abertura de boards.
- MVP de canvas SVG para edicao local de boards.
- Elementos de canvas: sticky notes, texto, formas, frames, conectores e desenho livre.
- Selecao, selecao por area, arrastar, redimensionar, rotacionar, duplicar, excluir, trazer para frente, agrupar e desagrupar.
- Atalhos de teclado para acoes comuns de edicao.
- Undo/redo dentro da sessao do board ativo.
- Autosave local com debounce para boards.
- Persistencia local em JSON para workspaces e boards.
- Escrita atomica de boards com suporte a recuperacao por journal na camada Rust.
- Assets locais de imagem para boards.
- Exportacao de SVG, PNG e workspace portatil.

## Project Phases

These phases describe the current direction. They are not release promises.

Estas fases descrevem a direcao atual. Elas nao sao promessas de release.

- Phase 0: Desktop/local-first foundation. Implemented.
- Phase 1: Workspaces and boards. Implemented as MVP.
- Phase 2: Canvas MVP. Implemented as MVP.
- Phase 3: Assets, export, and recovery flows. Local image and export MVP implemented; import UI is still planned.
- Phase 4: Quality, tests, accessibility, packaging, and release polish. Planned.

## Stack

- Tauri 2
- Rust 2021
- React 18
- TypeScript
- Vite
- Zustand
- CSS Modules
- SVG canvas rendering

## Requirements

- Node.js compatible with the Vite/TypeScript toolchain.
- npm.
- Rust toolchain.
- Tauri system dependencies for your operating system.

For Tauri setup details, use the official documentation: https://tauri.app/

## Development

Install dependencies:

```sh
npm install
```

Run the Vite frontend:

```sh
npm run dev
```

Run the desktop app in development mode:

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

## Project Structure

```text
.
├── src/                         React frontend
│   ├── app/                     Application shell and workspace entry
│   ├── features/canvas/         SVG canvas MVP and canvas styling
│   ├── features/workspace/      Workspace view, board list, board creation
│   ├── lib/                     Shared frontend types and Tauri API wrapper
│   ├── stores/                  Zustand stores
│   └── styles/                  Global styles and design tokens
├── src-tauri/                   Tauri/Rust backend
│   ├── src/domain.rs            Workspace, board, asset, and canvas domain types
│   ├── src/lib.rs               Tauri commands and app setup
│   └── src/persistence.rs       Local JSON persistence, journaling, assets, import/export groundwork
└── package.json                 Frontend scripts and dependencies
```

## Tauri Commands

Currently exposed to the frontend:

- `list_workspaces`
- `create_workspace`
- `create_board`
- `list_boards`
- `open_board`
- `save_board`
- `add_asset`
- `read_asset`
- `export_workspace`
- `import_workspace`

The interface currently supports adding local image assets and exporting boards or workspaces. Import is exposed through Tauri but does not yet have a dedicated user interface.

## Local Data

LogLine is designed around local-first storage. The Rust backend stores workspace data under the application's local data directory, inside a `workspaces` folder managed by Tauri.

O LogLine foi pensado para armazenamento local-first. O backend Rust salva os dados no diretorio local da aplicacao, dentro de uma pasta `workspaces` gerenciada pelo Tauri.

Current local structure includes board JSON files and groundwork for workspace assets and journals.

A estrutura local atual inclui arquivos JSON de boards e base para assets e journals de workspace.

Do not commit generated local data, builds, dependency folders, or environment files.

Nao versionar dados locais gerados, builds, dependencias ou arquivos de ambiente.

## Contributing

Contributions are welcome. Please read:

- `CONTRIBUTING.md`
- `CODE_OF_CONDUCT.md`
- `TRADEMARK.md`

Contribuicoes sao bem-vindas. Leia os documentos acima antes de abrir issues, PRs ou distribuir versoes modificadas.

## License

LogLine is distributed under the MIT License. See `LICENSE` for the full text.

O LogLine e distribuido sob a Licenca MIT. Consulte `LICENSE` para o texto completo.
