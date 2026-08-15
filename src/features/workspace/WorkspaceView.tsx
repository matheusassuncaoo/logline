import { FormEvent, useEffect, useRef, useState } from "react";
import { ArrowLeft, Check, Copy, Download, FilePlus2, ImagePlus, PanelLeft, Pencil, Plus, Settings, Trash2, X } from "lucide-react";
import { nanoid } from "nanoid";
import { Canvas } from "../canvas/Canvas";
import { applyBoardOperation, createBoardOperation, type BoardOperation } from "../../lib/boardOperations";
import { normalizeElementOrder } from "../../lib/boardOperations";
import { download, downloadPng, downloadSvg } from "../../lib/export";
import { workspaceApi } from "../../lib/tauri";
import type { Board, BoardSummary, CanvasElement, WorkspaceSummary } from "../../lib/types";
import styles from "./WorkspaceView.module.css";

export function WorkspaceView({ workspace, onBack, onOpenSettings }: { workspace: WorkspaceSummary; onBack: () => void; onOpenSettings: () => void }) {
  const [boards, setBoards] = useState<BoardSummary[]>([]);
  const [board, setBoard] = useState<Board | null>(null);
  const [name, setName] = useState("");
  const [rename, setRename] = useState("");
  const [isRenaming, setIsRenaming] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [historyVersion, setHistoryVersion] = useState(0);
  const [assets, setAssets] = useState<Record<string, string>>({});
  const [isLoadingBoards, setIsLoadingBoards] = useState(true);
  const saveTimer = useRef<number | null>(null);
  const imageInput = useRef<HTMLInputElement>(null);
  const history = useRef<{ past: BoardOperation[]; future: BoardOperation[] }>({ past: [], future: [] });

  useEffect(() => {
    setIsLoadingBoards(true);
    void workspaceApi.listBoards(workspace.id).then(setBoards).catch((reason) => setError(String(reason))).finally(() => setIsLoadingBoards(false));
  }, [workspace.id]);

  useEffect(() => {
    if (!board) return;
    if (saveTimer.current) window.clearTimeout(saveTimer.current);
    saveTimer.current = window.setTimeout(() => {
      void workspaceApi.saveBoard(board).catch((reason) => setError(String(reason)));
    }, 500);
    return () => { if (saveTimer.current) window.clearTimeout(saveTimer.current); };
  }, [board]);

  useEffect(() => {
    if (!board) { setAssets({}); return; }
    const imageIds = [...new Set(Object.values(board.elements).filter((element) => element.kind === "image").map((element) => element.content))];
    void Promise.all(imageIds.map((id) => workspaceApi.readAsset(workspace.id, id))).then((items) => setAssets(Object.fromEntries(items.map((asset) => [asset.id, asset.dataUrl])))).catch((reason) => setError(String(reason)));
  }, [board?.id, workspace.id]);

  async function createBoard(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (!name.trim()) return;
    try {
      const next = await workspaceApi.createBoard(workspace.id, name.trim());
      setBoards((current) => [{ id: next.id, name: next.name, updatedAt: next.updatedAt }, ...current]);
      setBoard(next);
      history.current = { past: [], future: [] };
      setHistoryVersion((version) => version + 1);
      setName("");
    } catch (reason) { setError(String(reason)); }
  }

  async function openBoard(id: string) {
    try {
      setBoard(await workspaceApi.openBoard(workspace.id, id));
      setIsRenaming(false);
      history.current = { past: [], future: [] };
      setHistoryVersion((version) => version + 1);
    } catch (reason) { setError(String(reason)); }
  }

  async function renameBoard(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (!board || !rename.trim()) return;
    try {
      const next = await workspaceApi.renameBoard(workspace.id, board.id, rename.trim());
      setBoard(next);
      setBoards((current) => current.map((item) => item.id === next.id ? { ...item, name: next.name, updatedAt: next.updatedAt } : item));
      setIsRenaming(false);
    } catch (reason) { setError(String(reason)); }
  }

  async function duplicateBoard() {
    if (!board) return;
    try {
      const name = `${board.name.slice(0, 112)} (cópia)`;
      const next = await workspaceApi.duplicateBoard(workspace.id, board.id, name);
      setBoards((current) => [{ id: next.id, name: next.name, updatedAt: next.updatedAt }, ...current]);
      setBoard(next);
      history.current = { past: [], future: [] };
      setHistoryVersion((version) => version + 1);
    } catch (reason) { setError(String(reason)); }
  }

  async function deleteBoard() {
    if (!board || !window.confirm(`Remover o board "${board.name}"?`)) return;
    try {
      await workspaceApi.deleteBoard(workspace.id, board.id);
      setBoards((current) => current.filter((item) => item.id !== board.id));
      setBoard(null);
      setIsRenaming(false);
      history.current = { past: [], future: [] };
      setHistoryVersion((version) => version + 1);
    } catch (reason) { setError(String(reason)); }
  }

  function commitBoard(before: Board, next: Board) {
    const normalizedBefore = normalizeElementOrder(before);
    const normalizedNext = normalizeElementOrder(next);
    const operation = createBoardOperation(normalizedBefore, normalizedNext);
    if (!operation) return;
    history.current.past.push(operation);
    history.current.future = [];
    setBoard(normalizedNext);
    setHistoryVersion((version) => version + 1);
  }

  function changeBoard(next: Board) {
    setBoard(normalizeElementOrder(next));
  }

  function undo() {
    if (!board) return;
    const operation = history.current.past.pop();
    if (!operation) return;
    history.current.future.push(operation);
    setBoard(applyBoardOperation(board, operation, "undo"));
    setHistoryVersion((version) => version + 1);
  }

  function redo() {
    if (!board) return;
    const operation = history.current.future.pop();
    if (!operation) return;
    history.current.past.push(operation);
    setBoard(applyBoardOperation(board, operation, "redo"));
    setHistoryVersion((version) => version + 1);
  }

  async function saveNow() {
    if (!board) return;
    if (saveTimer.current) window.clearTimeout(saveTimer.current);
    try { setBoard(await workspaceApi.saveBoard(board)); } catch (reason) { setError(String(reason)); }
  }

  useEffect(() => {
    function handleSave(event: KeyboardEvent) {
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "s") { event.preventDefault(); void saveNow(); }
    }
    window.addEventListener("keydown", handleSave);
    return () => window.removeEventListener("keydown", handleSave);
  });

  async function addImage(file: File) {
    if (!board) return;
    try {
      const asset = await workspaceApi.addAsset(workspace.id, file, new Uint8Array(await file.arrayBuffer()));
      const element: CanvasElement = { id: nanoid(), kind: "image", x: 120, y: 120, width: 360, height: 240, rotation: 0, content: asset.id, color: "" };
      commitBoard(board, { ...board, elements: { ...board.elements, [element.id]: element }, elementOrder: [...board.elementOrder, element.id] });
      setAssets((current) => ({ ...current, [asset.id]: asset.dataUrl }));
    } catch (reason) { setError(String(reason)); }
  }

  async function exportPortable() {
    try {
      const bytes = await workspaceApi.exportWorkspace(workspace.id);
      download(new Blob([new Uint8Array(bytes)], { type: "application/zip" }), `${workspace.name}.logline`);
    } catch (reason) { setError(String(reason)); }
  }

  return (
    <main className={styles.shell}>
      <aside className={styles.sidebar}>
        <button className={styles.back} type="button" onClick={onBack}><ArrowLeft size={17} /> Workspaces</button>
        <div className={styles.workspaceTitle}><span>{workspace.name.slice(0, 1).toUpperCase()}</span><strong>{workspace.name}</strong></div>
        <div className={styles.boardsHeader}><span>Boards</span><FilePlus2 size={16} /></div>
        <form className={styles.newBoard} onSubmit={createBoard}>
          <input aria-label="Nome do novo board" value={name} onChange={(event) => setName(event.target.value)} placeholder="Nome do board" maxLength={120} />
          <button type="submit" disabled={!name.trim()} aria-label="Criar board"><Plus size={16} /></button>
        </form>
        <nav className={styles.boardList} aria-label="Boards">
          {boards.map((item) => <button key={item.id} className={item.id === board?.id ? styles.boardActive : ""} type="button" onClick={() => void openBoard(item.id)}><PanelLeft size={15} /> {item.name}</button>)}
        </nav>
        {isLoadingBoards && <p className={styles.sidebarStatus}>Carregando boards...</p>}
        {!isLoadingBoards && boards.length === 0 && <p className={styles.sidebarStatus}>Nenhum board ainda.</p>}
      </aside>
      <section className={styles.main}>
        <header className={styles.topbar}>
          <div>
            <p>{workspace.name}</p>
            {isRenaming && board ? <form className={styles.renameForm} onSubmit={renameBoard}><input autoFocus aria-label="Nome do board" value={rename} onChange={(event) => setRename(event.target.value)} maxLength={120} /><button type="submit" aria-label="Confirmar nome"><Check size={15} /></button><button type="button" aria-label="Cancelar renomeação" onClick={() => setIsRenaming(false)}><X size={15} /></button></form> : <h1>{board?.name ?? "Selecione ou crie um board"}</h1>}
          </div>
          <div className={styles.actions}>
            {board && <><button type="button" onClick={() => { setRename(board.name); setIsRenaming(true); }} title="Renomear board"><Pencil size={16} /></button><button type="button" onClick={() => void duplicateBoard()} title="Duplicar board"><Copy size={16} /></button><button type="button" onClick={() => void deleteBoard()} title="Remover board"><Trash2 size={16} /></button><button type="button" onClick={() => imageInput.current?.click()} title="Adicionar imagem"><ImagePlus size={16} /></button><button type="button" onClick={() => downloadSvg(board, assets)} title="Exportar SVG">SVG</button><button type="button" onClick={() => void downloadPng(board, assets).catch((reason) => setError(String(reason)))} title="Exportar PNG">PNG</button></>}
            <button type="button" onClick={onOpenSettings} title="Configuracoes"><Settings size={16} /></button><button type="button" onClick={() => void exportPortable()} title="Exportar workspace"><Download size={16} /></button>
            <span className={styles.saved}>{board ? "Salvamento local ativo" : ""}</span>
          </div>
        </header>
        <input ref={imageInput} className={styles.hiddenInput} type="file" accept="image/png,image/jpeg,image/gif,image/webp,image/svg+xml" onChange={(event) => { const file = event.target.files?.[0]; if (file) void addImage(file); event.currentTarget.value = ""; }} />
        {error && <p className={styles.error} role="alert">{error}</p>}
        {board ? <Canvas board={board} onChange={changeBoard} onCommit={commitBoard} onUndo={undo} onRedo={redo} canUndo={history.current.past.length > 0} canRedo={history.current.future.length > 0} assets={assets} /> : <div className={styles.blank}>Crie um board para começar.</div>}
      </section>
    </main>
  );
}
