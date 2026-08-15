export type WorkspaceSummary = {
  id: string;
  name: string;
  createdAt: number;
  updatedAt: number;
  boardCount: number;
};

export type CanvasElementKind = "sticky-note" | "text" | "shape" | "connector" | "frame" | "freehand" | "image";

export type CanvasElement = {
  id: string;
  kind: CanvasElementKind;
  x: number;
  y: number;
  width: number;
  height: number;
  rotation: number;
  content: string;
  color: string;
  groupId?: string;
};

export type BoardSummary = {
  id: string;
  name: string;
  updatedAt: number;
};

export type AssetData = {
  id: string;
  dataUrl: string;
};

export type AppPreferences = {
  theme: "system" | "light" | "dark";
};

export type Board = {
  id: string;
  workspaceId: string;
  name: string;
  schemaVersion: number;
  createdAt: number;
  updatedAt: number;
  elements: Record<string, CanvasElement>;
  elementOrder: string[];
};
