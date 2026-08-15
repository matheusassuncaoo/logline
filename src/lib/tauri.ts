import { invoke } from "@tauri-apps/api/core";
import type { AssetData, Board, BoardSummary, WorkspaceSummary } from "./types";

export const workspaceApi = {
  list: () => invoke<WorkspaceSummary[]>("list_workspaces"),
  create: (name: string) => invoke<WorkspaceSummary>("create_workspace", { name }),
  createBoard: (workspaceId: string, name: string) =>
    invoke<Board>("create_board", { workspaceId, name }),
  renameBoard: (workspaceId: string, boardId: string, name: string) =>
    invoke<Board>("rename_board", { workspaceId, boardId, name }),
  duplicateBoard: (workspaceId: string, boardId: string, name: string) =>
    invoke<Board>("duplicate_board", { workspaceId, boardId, name }),
  deleteBoard: (workspaceId: string, boardId: string) =>
    invoke<void>("delete_board", { workspaceId, boardId }),
  listBoards: (workspaceId: string) => invoke<BoardSummary[]>("list_boards", { workspaceId }),
  openBoard: (workspaceId: string, boardId: string) =>
    invoke<Board>("open_board", { workspaceId, boardId }),
  saveBoard: (board: Board) => invoke<Board>("save_board", { board }),
  addAsset: (workspaceId: string, file: File, bytes: Uint8Array) =>
    invoke<AssetData>("add_asset", { workspaceId, fileName: file.name, mimeType: file.type, bytes: Array.from(bytes) }),
  readAsset: (workspaceId: string, assetId: string) => invoke<AssetData>("read_asset", { workspaceId, assetId }),
  exportWorkspace: (workspaceId: string) => invoke<number[]>("export_workspace", { workspaceId }),
  importWorkspace: (bytes: Uint8Array) => invoke<WorkspaceSummary>("import_workspace", { bytes: Array.from(bytes) }),
};
