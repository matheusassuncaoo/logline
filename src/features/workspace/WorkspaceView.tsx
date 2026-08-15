import { ArrowLeft } from "lucide-react";
import type { WorkspaceSummary } from "../../lib/types";
import styles from "./WorkspaceView.module.css";

export function WorkspaceView({ workspace, onBack }: { workspace: WorkspaceSummary; onBack: () => void }) {
  return (
    <main className={styles.shell}>
      <aside className={styles.sidebar}>
        <button className={styles.back} type="button" onClick={onBack}><ArrowLeft size={17} /> Workspaces</button>
        <div className={styles.workspaceTitle}><span>{workspace.name.slice(0, 1).toUpperCase()}</span><strong>{workspace.name}</strong></div>
      </aside>
      <section className={styles.main}>
        <header className={styles.topbar}>
          <div><p>Workspace local</p><h1>{workspace.name}</h1></div>
        </header>
        <div className={styles.blank}>Seu workspace está pronto para receber boards.</div>
      </section>
    </main>
  );
}
