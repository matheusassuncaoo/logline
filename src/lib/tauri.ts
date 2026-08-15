import { invoke } from "@tauri-apps/api/core";
import type { Board, BoardSummary, WorkspaceSummary } from "./types";

export const workspaceApi = {
  list: () => invoke<WorkspaceSummary[]>("list_workspaces"),
  create: (name: string) => invoke<WorkspaceSummary>("create_workspace", { name }),
  createBoard: (workspaceId: string, name: string) =>
    invoke<Board>("create_board", { workspaceId, name }),
  listBoards: (workspaceId: string) => invoke<BoardSummary[]>("list_boards", { workspaceId }),
  openBoard: (workspaceId: string, boardId: string) =>
    invoke<Board>("open_board", { workspaceId, boardId }),
  saveBoard: (board: Board) => invoke<Board>("save_board", { board }),
};
