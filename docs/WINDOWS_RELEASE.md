# Windows Release / Lancamento Windows

## Build

Use a Windows machine with the Microsoft C++ Build Tools and WebView2 installed:

```sh
npm install
npm run tauri build
```

Tauri generates NSIS and MSI installers below `src-tauri/target/release/bundle/`. They are build artifacts and intentionally ignored by Git.

## Verification / Verificacao

1. Install with a generated package on a clean Windows account.
2. Create a workspace, board and image asset.
3. Close and reopen LogLine and confirm the board is recovered.
4. Export a `.logline` file, import it, and confirm boards and assets are present.
5. Review rotating local logs under the application log directory managed by Tauri.

## Future Updates / Atualizacoes Futuras

The updater is intentionally not configured in this offline-first MVP. Before enabling it, define a signed release endpoint and configure Tauri's updater public key and endpoint in `tauri.conf.json`. Never ship a placeholder endpoint or signing key.
