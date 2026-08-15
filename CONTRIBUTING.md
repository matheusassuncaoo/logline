# Contributing to LogLine

Thank you for contributing to LogLine. This guide defines how to propose, implement, verify, and review changes.

Obrigado por contribuir com o LogLine. Este guia define como propor, implementar, verificar e revisar mudanças.

LogLine is an early-stage offline-first desktop whiteboard. Contributions must preserve the local-first direction, protect local data, and describe product behavior honestly.

O LogLine é um whiteboard desktop offline-first em fase inicial. As contribuições devem preservar a direção local-first, proteger os dados locais e descrever o comportamento do produto com honestidade.

## Contribution Principles

- Solve a clear user, product, reliability, accessibility, or maintenance problem.
- Keep one pull request focused on one coherent outcome.
- Prefer simple, explicit code over clever abstractions.
- Preserve offline-first behavior; do not add cloud, telemetry, accounts, sync, or network dependencies without prior discussion.
- Treat local data and persisted schemas as part of the product contract.
- Do not claim a feature is shipped until its user-facing flow is implemented and verified.
- Do not commit generated files, credentials, local workspaces, or private data.

## Princípios De Contribuição

- Resolva um problema claro de usuário, produto, confiabilidade, acessibilidade ou manutenção.
- Mantenha um pull request focado em um resultado coerente.
- Prefira código simples e explícito a abstrações desnecessárias.
- Preserve o comportamento offline-first; não adicione nuvem, telemetria, contas, sincronização ou dependências de rede sem discussão prévia.
- Trate dados locais e schemas persistidos como parte do contrato do produto.
- Não declare uma funcionalidade como pronta até que seu fluxo de usuário esteja implementado e verificado.
- Não versione arquivos gerados, credenciais, workspaces locais ou dados privados.

## Project Phases

| Phase | Scope | Status |
| --- | --- | --- |
| 0 | Desktop and local-first foundation | Implemented |
| 1 | Workspaces and boards | MVP implemented |
| 2 | Canvas editor | MVP implemented |
| 3 | Assets, import/export, and recovery | Local MVP implemented |
| 4 | Test coverage, accessibility, packaging, and release polish | Planned |

When proposing work, state the phase it supports and why it matters now.

## Fases Do Projeto

| Fase | Escopo | Status |
| --- | --- | --- |
| 0 | Fundação desktop e local-first | Implementada |
| 1 | Workspaces e boards | MVP implementado |
| 2 | Editor de canvas | MVP implementado |
| 3 | Assets, importação/exportação e recuperação | MVP local implementado |
| 4 | Cobertura de testes, acessibilidade, empacotamento e refinamento de release | Planejada |

Ao propor uma mudança, informe qual fase ela apoia e por que ela é importante agora.

## Before You Start

1. Read `README.md`, `AGENTS.md`, `CODE_OF_CONDUCT.md`, and `SECURITY.md`.
2. Check existing issues and pull requests when a public repository is available.
3. Define the problem before selecting a technical solution.
4. Open an issue or discussion before a broad feature, architecture change, persistence change, or dependency addition.
5. Keep an initial contribution small when you are new to the project.

Read `TRADEMARK.md` before publishing a fork, modified build, screenshot collection, or app listing.

## System Analysis Checklist

Use this checklist before implementation. It applies to maintainers, contributors, and coding agents.

- What user or system problem does this change solve?
- Who is affected, and what is the current workflow?
- What is the expected workflow after the change?
- What are the acceptance criteria that prove the change is complete?
- Which states must be handled: empty, loading, success, validation error, storage error, and recovery?
- Does it affect desktop, frontend, Rust backend, persisted data, import/export, or packaging?
- What happens without a network connection?
- Does it affect existing workspaces or boards? Is migration or recovery required?
- What are the failure modes, and how are they communicated to the user?
- What manual verification is required if automated tests do not cover the change?
- Does the change require documentation, screenshots, a changelog entry, or a versioning decision?

## Checklist De Análise De Sistemas

Use este checklist antes de implementar. Ele vale para mantenedores, contribuidores e agentes de código.

- Qual problema de usuário ou de sistema esta mudança resolve?
- Quem é afetado e qual é o fluxo atual?
- Qual é o fluxo esperado após a mudança?
- Quais são os critérios de aceite que comprovam que a mudança está pronta?
- Quais estados precisam ser tratados: vazio, carregando, sucesso, erro de validação, erro de armazenamento e recuperação?
- Ela afeta desktop, frontend, backend Rust, dados persistidos, importação/exportação ou empacotamento?
- O que acontece sem conexão com a internet?
- Ela afeta workspaces ou boards existentes? É necessária migração ou recuperação?
- Quais são os modos de falha e como eles são comunicados ao usuário?
- Qual verificação manual é necessária se testes automatizados não cobrirem a mudança?
- A mudança exige documentação, screenshots, changelog ou decisão de versionamento?

## Local Setup

Install dependencies:

```sh
npm install
```

Run the frontend only:

```sh
npm run dev
```

Run the desktop application:

```sh
npm run tauri dev
```

Build the frontend:

```sh
npm run build
```

Build the desktop application when the change needs desktop-level verification:

```sh
npm run tauri build
```

## Workflow

1. Create or identify the issue that explains the problem and acceptance criteria.
2. Create a branch from the current default branch.
3. Implement the smallest correct change.
4. Verify the affected behavior and run the relevant checks.
5. Update documentation and changelog information when user-visible behavior changes.
6. Open a pull request using the repository template.
7. Respond to review feedback with focused commits and keep the PR scope stable.

Suggested branch names:

```text
feature/workspace-import-feedback
fix/canvas-resize-history
docs/contribution-guidelines
refactor/workspace-store-errors
test/board-persistence-validation
chore/update-tauri-config
```

## Commit Guidelines

Use concise Conventional Commit-style messages:

```text
feat: add workspace import feedback
fix: preserve undo history after resize
docs: expand contribution guidelines
refactor: simplify workspace error handling
test: cover board persistence validation
chore: update Tauri configuration
```

Commit rules:

- Use the imperative mood: `add`, `fix`, `document`, not `added` or `fixes`.
- Keep each commit focused on one intention.
- Separate refactors from behavior changes when practical.
- Do not mix unrelated formatting, dependency updates, and product changes.
- Do not commit `node_modules`, `dist`, `src-tauri/target`, `.env`, `.env.*`, local workspaces, exports, or generated artifacts.
- Explain non-obvious data migrations or compatibility decisions in the commit body and pull request.

## Regras De Commit

- Use mensagens curtas no imperativo: `feat: adicionar feedback de importação` ou `fix: preservar histórico após redimensionamento`.
- Mantenha cada commit focado em uma intenção.
- Separe refactors de mudanças de comportamento quando for prático.
- Não misture formatação não relacionada, atualização de dependências e mudanças de produto.
- Não versione `node_modules`, `dist`, `src-tauri/target`, `.env`, `.env.*`, workspaces locais, exports ou artefatos gerados.
- Explique migrações de dados e decisões de compatibilidade não óbvias no corpo do commit e no pull request.

## Pull Requests

A pull request must make review easy. Include:

- Problem: what was wrong or missing?
- Solution: what changed and why?
- Scope: what is intentionally included and excluded?
- Acceptance criteria: how does the reviewer know it works?
- Verification: commands run and manual flows tested.
- Persistence impact: whether data schemas, journals, assets, imports, or exports changed.
- Risks and follow-up work.
- Screenshots or recordings for visual, canvas, or interaction changes.

Keep pull requests small. Split independent refactors, dependency upgrades, and product features into separate pull requests whenever possible.

## Review Expectations

Reviewers should check:

- The change solves the stated problem and meets acceptance criteria.
- Scope is focused and no unrelated behavior changed.
- Errors, loading states, empty states, and keyboard flows are considered.
- UI changes remain usable on desktop and smaller layouts.
- Canvas changes preserve pan, zoom, selection, drag, resize, rotation, undo/redo, and autosave behavior.
- Rust and TypeScript types remain aligned.
- Persistence changes validate input, preserve atomic writes, and avoid local data loss.
- Documentation and changelog reflect user-visible changes.

## Using AI And Coding Agents

AI assistance is allowed, but responsibility remains with the contributor submitting the change.

- Read and understand every changed line before opening a pull request.
- Review generated diffs for regressions, duplicated logic, unsafe assumptions, and invented features.
- Do not send secrets, local workspace data, private customer data, credentials, or proprietary material to external tools.
- Do not use generated text to make unsupported product, security, or performance claims.
- Follow `AGENTS.md` and `CLAUDE.md` when using coding agents in this repository.
- Treat changes to canvas interactions, persistence, import/export, assets, and schemas as high-risk; perform deliberate human review and verification.
- State in the pull request when AI materially generated implementation or documentation, especially when reviewers need additional context.
- Do not use AI to bypass review, inflate contribution volume, or create large changes that cannot be explained and verified.

## Uso De IA E Agentes De Código

O uso de IA é permitido, mas a responsabilidade continua sendo de quem envia a contribuição.

- Leia e entenda cada linha alterada antes de abrir um pull request.
- Revise diffs gerados para encontrar regressões, lógica duplicada, suposições inseguras e funcionalidades inventadas.
- Não envie segredos, dados de workspaces locais, dados privados, credenciais ou material proprietário para ferramentas externas.
- Não use texto gerado para fazer alegações de produto, segurança ou desempenho que não tenham sido verificadas.
- Siga `AGENTS.md` e `CLAUDE.md` ao usar agentes de código neste repositório.
- Trate mudanças em interações do canvas, persistência, importação/exportação, assets e schemas como alto risco; faça revisão humana e verificação cuidadosas.
- Informe no pull request quando IA tiver gerado materialmente a implementação ou documentação, principalmente se reviewers precisarem de contexto adicional.
- Não use IA para ignorar revisão, inflar volume de contribuições ou criar mudanças grandes que não possam ser explicadas e verificadas.

## Frontend Guidelines

- Keep TypeScript strict-compatible.
- Follow established React and CSS Module patterns.
- Handle empty, loading, success, and error states explicitly.
- Keep interactive controls keyboard-accessible and labeled.
- Do not break the current desktop layout or smaller viewport behavior.
- Keep UI language consistent with the local-first product direction.

## Canvas Guidelines

The canvas MVP is centered in `src/features/canvas/Canvas.tsx`.

- Preserve selection, marquee selection, panning, zooming, dragging, resizing, and rotation.
- Preserve undo/redo commit boundaries and autosave coordination in `WorkspaceView.tsx`.
- Avoid keyboard shortcuts that interfere with inputs and textareas.
- Keep `sticky-note`, `text`, `shape`, `connector`, `frame`, `freehand`, and `image` behavior compatible.
- Manually verify element creation, editing, selection, movement, deletion, history, save, reload, export, and import when a related change is made.

## Backend And Persistence Guidelines

- Keep Tauri command names aligned with the frontend wrapper.
- Keep frontend and Rust domain types aligned with serialization names.
- Validate IDs, names, files, MIME types, sizes, and archive contents before writing data.
- Preserve atomic writes and journal recovery unless a safer replacement is implemented.
- Respect `schemaVersion` and provide a migration strategy for persisted format changes.
- Prevent data loss, path traversal, invalid archive contents, and unsafe asset handling.
- Return useful errors without exposing unnecessary local paths or private data.

## Verification Matrix

| Change type | Minimum verification |
| --- | --- |
| Documentation only | Review rendered Markdown and links |
| Frontend or TypeScript | `npm run build`; run `npm test` when relevant |
| Rust backend only | `cargo test` from `src-tauri` when relevant |
| Desktop integration | `npm run tauri build` when feasible |
| Canvas interaction | Build, relevant tests, and manual interaction verification |
| Persistence or migration | `cargo test`, build, and manual save, reload, and recovery verification with temporary data |
| Import/export or assets | `npm test`, `cargo test`, build, and valid, invalid, and boundary-size manual cases |

If a relevant command cannot run, state why in the pull request.

## Definition Of Done

A contribution is ready when:

- It solves the stated problem and meets its acceptance criteria.
- The relevant verification was completed or the limitation was documented.
- The implementation is understandable and within scope.
- Local data safety and compatibility were considered.
- User-visible behavior, documentation, and changelog entries are updated when needed.
- No secrets, generated files, or unrelated changes are included.
- The pull request template is completed honestly.

## Reporting Bugs And Requesting Features

Use the issue templates when the project is hosted publicly.

- A bug report needs reproducible steps, expected behavior, actual behavior, environment, and safe diagnostic material.
- A feature request needs the user workflow, problem, proposed outcome, acceptance criteria, local-first impact, and alternatives considered.
- Security issues must not be reported publicly. Follow `SECURITY.md`.

## Tests

Automated tests are available through `npm test` for frontend export behavior and `cargo test` in `src-tauri` for persistence flows. Contributors must add or update relevant tests when practical and document manual verification for interactions not covered by automation.

Testes automatizados estão disponíveis por `npm test` para o comportamento de exportação do frontend e por `cargo test` em `src-tauri` para fluxos de persistência. Contribuidores devem adicionar ou atualizar testes relevantes quando for prático e documentar a verificação manual de interações não cobertas pela automação.
