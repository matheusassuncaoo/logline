import { create } from "zustand";
import { workspaceApi } from "../lib/tauri";
import type { WorkspaceSummary } from "../lib/types";

type WorkspaceState = {
  workspaces: WorkspaceSummary[];
  isLoading: boolean;
  error: string | null;
  load: () => Promise<void>;
  create: (name: string) => Promise<void>;
};

export const useWorkspaceStore = create<WorkspaceState>((set) => ({
  workspaces: [],
  isLoading: true,
  error: null,
  load: async () => {
    set({ isLoading: true, error: null });
    try {
      set({ workspaces: await workspaceApi.list(), isLoading: false });
    } catch (error) {
      set({ error: error instanceof Error ? error.message : "Não foi possível abrir os workspaces.", isLoading: false });
    }
  },
  create: async (name) => {
    set({ error: null });
    try {
      const workspace = await workspaceApi.create(name);
      set((state) => ({ workspaces: [workspace, ...state.workspaces] }));
    } catch (error) {
      set({ error: error instanceof Error ? error.message : "Não foi possível criar o workspace." });
    }
  },
}));
