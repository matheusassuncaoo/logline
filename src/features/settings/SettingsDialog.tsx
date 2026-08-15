import * as Dialog from "@radix-ui/react-dialog";
import { Check, Monitor, Moon, Settings, Sun, X } from "lucide-react";
import type { AppPreferences } from "../../lib/types";
import styles from "./SettingsDialog.module.css";

const themes = [
  { id: "system", label: "Sistema", description: "Segue o tema do Windows", icon: Monitor },
  { id: "light", label: "Claro", description: "Interface clara", icon: Sun },
  { id: "dark", label: "Escuro", description: "Interface escura", icon: Moon },
] as const;

export function SettingsDialog({ open, onOpenChange, preferences, onChange }: { open: boolean; onOpenChange: (open: boolean) => void; preferences: AppPreferences; onChange: (preferences: AppPreferences) => void }) {
  return (
    <Dialog.Root open={open} onOpenChange={onOpenChange}>
      <Dialog.Portal>
        <Dialog.Overlay className={styles.overlay} />
        <Dialog.Content className={styles.content} aria-describedby={undefined}>
          <div className={styles.header}>
            <div><Dialog.Title>Configuracoes</Dialog.Title><Dialog.Description>Preferencias armazenadas apenas neste dispositivo.</Dialog.Description></div>
            <Dialog.Close className={styles.close} aria-label="Fechar configuracoes"><X size={18} /></Dialog.Close>
          </div>
          <section className={styles.section} aria-labelledby="theme-heading">
            <div className={styles.sectionHeading}><Settings size={16} /><h2 id="theme-heading">Aparencia</h2></div>
            <p>Escolha como a interface do LogLine deve aparecer.</p>
            <div className={styles.options} role="radiogroup" aria-label="Tema">
              {themes.map(({ id, label, description, icon: Icon }) => {
                const active = preferences.theme === id;
                return <button key={id} className={active ? styles.optionActive : styles.option} type="button" role="radio" aria-checked={active} onClick={() => onChange({ ...preferences, theme: id })}><Icon size={18} /><span><strong>{label}</strong><small>{description}</small></span>{active && <Check className={styles.check} size={17} />}</button>;
              })}
            </div>
          </section>
        </Dialog.Content>
      </Dialog.Portal>
    </Dialog.Root>
  );
}
