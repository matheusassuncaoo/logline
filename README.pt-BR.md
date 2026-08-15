# LogLine

> Um whiteboard desktop offline-first para ideias, mapas, fluxos e decisões locais.

[English](README.en.md) | [Showcase do projeto](SHOWCASE.md) | [Como contribuir](CONTRIBUTING.md)

## Status Do Projeto

O LogLine `1.0.0` é a primeira release local estável. Ele permanece focado em um fluxo prático de whiteboard offline-first.

O produto atual tem um fluxo local funcional: criar workspaces, criar e abrir boards, editá-los em um canvas SVG e salvar os dados localmente. O projeto ainda está em fase inicial, portanto formatos persistidos e interações podem evoluir antes de uma release estável.

## Por Que LogLine

Muitas ferramentas de pensamento visual começam com contas, sincronização em nuvem ou colaboração sempre conectada. O LogLine parte de outra premissa: um whiteboard útil deve funcionar localmente primeiro.

- Nenhuma conta é necessária para o fluxo local.
- Dados de workspaces e boards ficam no dispositivo.
- A edição principal não depende de conexão com a internet.
- O app desktop usa Tauri, React, TypeScript e Rust.

## Recursos

### Workspaces E Boards

- Criar e listar workspaces locais.
- Abrir um workspace e gerenciar sua lista de boards.
- Criar, listar e abrir boards.
- Persistir metadados de workspace e board em JSON local.

### Canvas MVP

- Canvas baseado em SVG com pan e zoom.
- Sticky notes, texto, formas, frames, conectores e desenho livre.
- Seleção e seleção por área.
- Arrastar, redimensionar e rotacionar.
- Duplicar, excluir, trazer para frente, agrupar e desagrupar.
- Atalhos de teclado para ações comuns de edição.
- Undo e redo durante a sessão.
- Autosave local com debounce.

### Assets Locais E Exportação

- Assets locais de imagem para boards.
- Exportação de boards em SVG e PNG.
- Exportação portátil de workspaces.
- Escrita atômica de boards e recuperação por journal na camada de persistência Rust.

Importe arquivos portáteis de workspace `.logline` pela tela inicial de workspaces.

## Fases Do Projeto

| Fase | Escopo | Status |
| --- | --- | --- |
| 0 | Fundação desktop e local-first | Implementada |
| 1 | Workspaces e boards | MVP implementado |
| 2 | Editor de canvas | MVP implementado |
| 3 | Assets, importação/exportação e recuperação | MVP local implementado |
| 4 | Cobertura de testes, acessibilidade, empacotamento e refinamento de release | Planejada |

As fases descrevem a direção do projeto, não promessas de release.

## Tecnologias

| Camada | Tecnologia |
| --- | --- |
| Shell desktop | Tauri 2 |
| Backend | Rust 2021 |
| Interface | React 18 e TypeScript |
| Ferramentas frontend | Vite |
| Estado do cliente | Zustand |
| Canvas | SVG |
| Estilos | CSS Modules |
| Dados locais | JSON gerenciado pelo backend Rust |

## Requisitos

- Node.js compatível com Vite e TypeScript.
- npm.
- Toolchain Rust.
- Dependências de sistema do Tauri para o seu sistema operacional.

Consulte os [pré-requisitos do Tauri](https://tauri.app/start/prerequisites/) para detalhes de instalação por sistema operacional.

## Como Começar

Clone o repositório e instale as dependências:

```sh
npm install
```

Inicie apenas o frontend:

```sh
npm run dev
```

Inicie o aplicativo desktop:

```sh
npm run tauri dev
```

Gere o build do frontend:

```sh
npm run build
```

Gere o build do aplicativo desktop:

```sh
npm run tauri build
```

## Testes

Execute os testes do frontend:

```sh
npm test
```

O Vitest cobre atualmente o comportamento de exportação SVG. Os testes Rust de persistência cobrem operações do ciclo de vida dos boards, recuperação por journal, deduplicação de assets e importação/exportação de workspaces portáteis.

Execute os testes Rust a partir de `src-tauri`:

```sh
cargo test
```

## Atalhos De Teclado

| Atalho | Ação |
| --- | --- |
| `V` | Ferramenta de seleção |
| `N` | Adicionar sticky note |
| `T` | Adicionar texto |
| `R` | Adicionar forma |
| `Ctrl/Cmd + Z` | Desfazer |
| `Ctrl/Cmd + Shift + Z` | Refazer |
| `Ctrl/Cmd + D` | Duplicar seleção |
| `Delete` ou `Backspace` | Excluir seleção |
| `Space` + arrastar | Mover o canvas |

Os atalhos não se aplicam enquanto você digita em um campo de texto ou textarea.

## Estrutura Do Projeto

```text
.
├── src/
│   ├── app/                  Shell da aplicação e entrada de workspace
│   ├── features/canvas/      Canvas SVG, ferramentas e estilos do canvas
│   ├── features/workspace/   Tela de workspace e fluxo de boards
│   ├── lib/                  Tipos compartilhados e API Tauri
│   ├── stores/               Estado Zustand
│   └── styles/               Estilos globais e tokens de design
├── src-tauri/
│   └── src/
│       ├── domain.rs         Tipos persistidos do domínio
│       ├── lib.rs            Comandos Tauri e configuração do app
│       └── persistence.rs    Dados locais, assets, exportação e recuperação
├── CONTRIBUTING.md
└── LICENSE
```

## Dados Locais E Privacidade

O LogLine foi projetado para armazenar dados localmente. O backend Rust usa o diretório de dados locais da aplicação fornecido pelo Tauri e mantém uma pasta `workspaces` nesse local.

Os dados incluem arquivos JSON de boards. A camada de persistência também gerencia assets locais e journals usados para recuperar gravações de boards interrompidas.

Não versione dados gerados, `node_modules`, `dist`, `src-tauri/target`, `.env` ou arquivos `.env.*`.

## Como Contribuir

Contribuições são bem-vindas, especialmente nas fases atuais de produto e qualidade:

- Melhorias de interação no canvas.
- Acessibilidade e comportamento responsivo.
- Ampliação da cobertura de testes automatizados.
- Fluxos visíveis de recuperação e backup.
- Empacotamento e automação de releases.
- Documentação e relatos de bugs.

Leia [CONTRIBUTING.md](CONTRIBUTING.md) e [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) antes de participar.

## Suporte E Segurança

- Leia [SUPPORT.md](SUPPORT.md) para dúvidas, bugs e sugestões.
- Leia [SECURITY.md](SECURITY.md) para reportar uma vulnerabilidade de forma privada.

## Marca

LogLine é o nome oficial do projeto. Leia [TRADEMARK.md](TRADEMARK.md) para as regras de forks, builds modificados e uso da marca.

## Licença

Copyright (c) 2026 Matheus Assunção da Cunha.

O LogLine é licenciado sob a [Licença MIT](LICENSE).
