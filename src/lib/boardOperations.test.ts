import { describe, expect, it } from "vitest";
import { applyBoardOperation, createBoardOperation, normalizeElementOrder } from "./boardOperations";
import type { Board, CanvasElement } from "./types";

const element = (id: string): CanvasElement => ({ id, kind: "shape", x: 0, y: 0, width: 100, height: 100, rotation: 0, content: "", color: "#dff2e9" });
const board = (elements: Record<string, CanvasElement>, elementOrder: string[]): Board => ({ id: "board", workspaceId: "workspace", name: "Board", schemaVersion: 1, createdAt: 0, updatedAt: 0, elements, elementOrder });

describe("board order integrity", () => {
  it("repairs duplicated, stale and missing element IDs", () => {
    const result = normalizeElementOrder(board({ a: element("a"), b: element("b") }, ["a", "a", "missing"]));
    expect(result.elementOrder).toEqual(["a", "b"]);
  });

  it("keeps order valid across undo and redo", () => {
    const before = board({ a: element("a") }, ["a"]);
    const after = board({ a: element("a"), arrow: { ...element("arrow"), kind: "connector" } }, ["a", "arrow", "arrow"]);
    const operation = createBoardOperation(before, after)!;
    const redone = applyBoardOperation(before, operation, "redo");
    const undone = applyBoardOperation(redone, operation, "undo");
    expect(redone.elementOrder).toEqual(["a", "arrow"]);
    expect(undone.elementOrder).toEqual(["a"]);
  });
});
