import type { Board, CanvasElement } from "./types";

type BoardPatch = {
  elements: Record<string, CanvasElement | null>;
  elementOrder?: string[];
};

export type BoardOperation = {
  undo: BoardPatch;
  redo: BoardPatch;
};

export function createBoardOperation(before: Board, after: Board): BoardOperation | null {
  const undoElements: Record<string, CanvasElement | null> = {};
  const redoElements: Record<string, CanvasElement | null> = {};
  const ids = new Set([...Object.keys(before.elements), ...Object.keys(after.elements)]);

  for (const id of ids) {
    const previous = before.elements[id];
    const next = after.elements[id];
    if (sameElement(previous, next)) continue;
    undoElements[id] = previous ? { ...previous } : null;
    redoElements[id] = next ? { ...next } : null;
  }

  const elementOrderChanged = !sameOrder(before.elementOrder, after.elementOrder);
  if (!Object.keys(undoElements).length && !elementOrderChanged) return null;

  return {
    undo: { elements: undoElements, ...(elementOrderChanged ? { elementOrder: [...before.elementOrder] } : {}) },
    redo: { elements: redoElements, ...(elementOrderChanged ? { elementOrder: [...after.elementOrder] } : {}) },
  };
}

export function applyBoardOperation(board: Board, operation: BoardOperation, direction: "undo" | "redo"): Board {
  const patch = operation[direction];
  const elements = { ...board.elements };
  for (const [id, element] of Object.entries(patch.elements)) {
    if (element) elements[id] = { ...element };
    else delete elements[id];
  }
  return { ...board, elements, ...(patch.elementOrder ? { elementOrder: [...patch.elementOrder] } : {}) };
}

function sameElement(left: CanvasElement | undefined, right: CanvasElement | undefined) {
  return left === right || (left !== undefined && right !== undefined
    && left.id === right.id
    && left.kind === right.kind
    && left.x === right.x
    && left.y === right.y
    && left.width === right.width
    && left.height === right.height
    && left.rotation === right.rotation
    && left.content === right.content
    && left.color === right.color
    && left.groupId === right.groupId);
}

function sameOrder(left: string[], right: string[]) {
  return left.length === right.length && left.every((id, index) => id === right[index]);
}
