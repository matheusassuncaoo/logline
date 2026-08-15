import { FormEvent, useEffect, useState } from "react";
import { ArrowUpRight, Plus, Search } from "lucide-react";
import { WorkspaceView } from "../features/workspace/WorkspaceView";
import type { WorkspaceSummary } from "../lib/types";
import { useWorkspaceStore } from "../stores/workspaceStore";
import styles from "./App.module.css";

export function App() {
  const { workspaces, isLoading, error, load, create } = useWorkspaceStore();
  const [name, setName] = useState("");
  const [isCreating, setIsCreating] = useState(false);
  const [selectedWorkspace, setSelectedWorkspace] = useState<WorkspaceSummary | null>(null);

  useEffect(() => {
    void load();
  }, [load]);

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

  if (selectedWorkspace) return <WorkspaceView workspace={selectedWorkspace} onBack={() => setSelectedWorkspace(null)} />;

  return (
    <main className={styles.shell}>
      <header className={styles.header}>
        <div className={styles.brand}>LogLine</div>
        <div className={styles.status}><span /> Local-first workspace</div>
      </header>
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
    </main>
  );
}
