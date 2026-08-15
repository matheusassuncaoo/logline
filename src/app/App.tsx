import { FormEvent, useEffect, useRef, useState } from "react";
import { ArrowUpRight, Plus, Search, Settings, Upload } from "lucide-react";
import { workspaceApi } from "../lib/tauri";
import { SettingsDialog } from "../features/settings/SettingsDialog";
import { WorkspaceView } from "../features/workspace/WorkspaceView";
import type { AppPreferences, WorkspaceSummary } from "../lib/types";
import { useWorkspaceStore } from "../stores/workspaceStore";
import styles from "./App.module.css";

export function App() {
  const { workspaces, isLoading, error, load, create } = useWorkspaceStore();
  const [name, setName] = useState("");
  const [isCreating, setIsCreating] = useState(false);
  const [selectedWorkspace, setSelectedWorkspace] = useState<WorkspaceSummary | null>(null);
  const [importError, setImportError] = useState<string | null>(null);
  const [preferences, setPreferences] = useState<AppPreferences>({ theme: "system" });
  const [settingsOpen, setSettingsOpen] = useState(false);
  const importInput = useRef<HTMLInputElement>(null);

  useEffect(() => {
    void load();
  }, [load]);

  useEffect(() => {
    void workspaceApi.getPreferences().then(setPreferences);
  }, []);

  useEffect(() => {
    const media = window.matchMedia("(prefers-color-scheme: dark)");
    const apply = () => { document.documentElement.dataset.theme = preferences.theme === "system" ? (media.matches ? "dark" : "light") : preferences.theme; };
    apply();
    media.addEventListener("change", apply);
    return () => media.removeEventListener("change", apply);
  }, [preferences.theme]);

  async function updatePreferences(next: AppPreferences) {
    setPreferences(next);
    try { setPreferences(await workspaceApi.savePreferences(next)); } catch { /* The selected theme remains usable for this session. */ }
  }

  async function handleSubmit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault();
    if (!name.trim()) return;

    setIsCreating(true);
    try {
      await create(name.trim());
      setName("");
    } finally {
      setIsCreating(false);
    }
  }

  async function importWorkspace(file: File) {
    try {
      setImportError(null);
      const workspace = await workspaceApi.importWorkspace(new Uint8Array(await file.arrayBuffer()));
      await load();
      setSelectedWorkspace(workspace);
    } catch (reason) { setImportError(String(reason)); }
  }

  if (selectedWorkspace) return <><WorkspaceView workspace={selectedWorkspace} onBack={() => setSelectedWorkspace(null)} onOpenSettings={() => setSettingsOpen(true)} /><SettingsDialog open={settingsOpen} onOpenChange={setSettingsOpen} preferences={preferences} onChange={(next) => void updatePreferences(next)} /></>;

  return (
    <main className={styles.shell}>
      <header className={styles.header}>
        <div className={styles.brand}>LogLine</div>
        <div className={styles.headerActions}><button type="button" onClick={() => setSettingsOpen(true)}><Settings size={15} /> Configuracoes</button><button type="button" onClick={() => importInput.current?.click()}><Upload size={15} /> Importar</button><div className={styles.status}><span /> Local-first workspace</div></div>
      </header>
      <input ref={importInput} className={styles.hiddenInput} type="file" accept=".logline,application/zip" onChange={(event) => { const file = event.target.files?.[0]; if (file) void importWorkspace(file); event.currentTarget.value = ""; }} />
      <section className={styles.content}>
        <div className={styles.intro}>
          <p className={styles.eyebrow}>Whiteboards sem dependência de rede</p>
          <h1>Organize ideias onde elas acontecem.</h1>
          <p>Crie um workspace local para começar a estruturar seus fluxos, mapas e decisões.</p>
        </div>
        <form className={styles.createForm} onSubmit={handleSubmit}>
          <label htmlFor="workspace-name">Novo workspace</label>
          <div className={styles.inputRow}>
            <input id="workspace-name" value={name} onChange={(event) => setName(event.target.value)} placeholder="Ex.: Produto 2026" maxLength={120} />
            <button type="submit" disabled={isCreating || !name.trim()}><Plus size={18} /> Criar</button>
          </div>
        </form>
        <div className={styles.sectionHeading}>
          <h2>Seus workspaces</h2>
          <Search size={18} aria-hidden="true" />
        </div>
        {isLoading && <p className={styles.muted}>Abrindo armazenamento local...</p>}
        {error && <p className={styles.error}>{error}</p>}
        {importError && <p className={styles.error}>{importError}</p>}
        {!isLoading && !error && workspaces.length === 0 && (
          <div className={styles.empty}>Seu primeiro board está a um workspace de distância.</div>
        )}
        <div className={styles.grid}>
          {workspaces.map((workspace) => (
            <button className={styles.workspace} key={workspace.id} type="button" onClick={() => setSelectedWorkspace(workspace)}>
              <span className={styles.workspaceMark}>{workspace.name.slice(0, 1).toUpperCase()}</span>
              <span className={styles.workspaceName}>{workspace.name}</span>
              <span className={styles.workspaceMeta}>{workspace.boardCount} {workspace.boardCount === 1 ? "board" : "boards"}</span>
              <ArrowUpRight className={styles.openIcon} size={18} aria-hidden="true" />
            </button>
          ))}
        </div>
      </section>
      <SettingsDialog open={settingsOpen} onOpenChange={setSettingsOpen} preferences={preferences} onChange={(next) => void updatePreferences(next)} />
    </main>
  );
}
