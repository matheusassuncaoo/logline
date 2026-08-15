# LogLine

LogLine is an early-stage offline-first desktop whiteboard for organizing workspaces, boards, flows, maps, and decisions locally.

LogLine e um whiteboard desktop offline-first em fase inicial para organizar workspaces, boards, fluxos, mapas e decisoes localmente.

## Status

This project is in `0.1.0` and should be treated as an MVP foundation.

Este projeto esta na versao `0.1.0` e deve ser tratado como uma base MVP.

Currently implemented:

- Desktop shell powered by Tauri.
- React interface for creating and listing local workspaces.
- Local JSON persistence for workspaces and boards.
- Tauri commands for listing workspaces, creating workspaces, creating boards, opening boards, and saving boards.
- Atomic JSON writes on the Rust side.

Implemented today:

- Aplicativo desktop com Tauri.
- Interface React para criar e listar workspaces locais.
- Persistencia local em JSON para workspaces e boards.
- Comandos Tauri para listar workspaces, criar workspaces, criar boards, abrir boards e salvar boards.
- Escrita atomica de JSON no backend Rust.

Planned direction:

- Board editor UI.
- Canvas elements such as sticky notes, text, shapes, connectors, frames, freehand drawings, and images.
- Better navigation between workspaces and boards.
- Export, import, backup, and recovery flows.
- Tests, release automation, and signed desktop builds.

Direcao planejada:

- Interface de edicao de boards.
- Elementos de canvas como notas, textos, formas, conectores, frames, desenhos livres e imagens.
- Navegacao melhor entre workspaces e boards.
- Fluxos de exportacao, importacao, backup e recuperacao.
- Testes, automacao de releases e builds desktop assinados.

## Stack

- Tauri 2
- Rust 2021
- React 18
- TypeScript
- Vite
- Zustand
- CSS Modules

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
├── src/                  React frontend
│   ├── app/              Application shell
│   ├── lib/              Shared frontend types and Tauri API wrapper
│   ├── stores/           Zustand stores
│   └── styles/           Global styles and design tokens
├── src-tauri/            Tauri/Rust backend
│   ├── src/domain.rs     Workspace, board, and canvas domain types
│   ├── src/lib.rs        Tauri commands and app setup
│   └── src/persistence.rs Local JSON persistence
└── package.json          Frontend scripts and dependencies
```

## Local Data

LogLine is designed around local-first storage. The Rust backend stores workspace data under the application's local data directory, inside a `workspaces` folder managed by Tauri.

O LogLine foi pensado para armazenamento local-first. O backend Rust salva os dados no diretorio local da aplicacao, dentro de uma pasta `workspaces` gerenciada pelo Tauri.

Do not commit generated local data, builds, dependency folders, or environment files.

Nao versionar dados locais gerados, builds, dependencias ou arquivos de ambiente.

## Contributing

Contributions are welcome. Please read:

- `CONTRIBUTING.md`
- `CODE_OF_CONDUCT.md`
- `TRADEMARK.md`

Contribuicoes sao bem-vindas. Leia os documentos acima antes de abrir issues, PRs ou distribuir versoes modificadas.

## License

No license file is currently included. Until a license is added, all rights are reserved by the project owner.

Ainda nao existe arquivo de licenca neste repositorio. Ate que uma licenca seja adicionada, todos os direitos ficam reservados ao proprietario do projeto.
