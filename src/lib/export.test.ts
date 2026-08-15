import { describe, expect, it } from "vitest";
import { boardSvg } from "./export";
import type { Board, CanvasElement } from "./types";

function boardWith(count: number): Board {
  const elements: Record<string, CanvasElement> = {};
  const elementOrder: string[] = [];
  for (let index = 0; index < count; index += 1) {
    const id = `note-${index}`;
    elements[id] = { id, kind: "sticky-note", x: (index % 40) * 240, y: Math.floor(index / 40) * 200, width: 210, height: 180, rotation: 0, content: `Note ${index}`, color: "#f7dd72" };
    elementOrder.push(id);
  }
  return { id: "board", workspaceId: "workspace", name: "Release board", schemaVersion: 1, createdAt: 0, updatedAt: 0, elements, elementOrder };
}

describe("boardSvg", () => {
  it("exports an empty small board", () => {
    expect(boardSvg(boardWith(0), {})).toContain('width="1600"');
  });

  it("exports a medium board without losing elements", () => {
    const svg = boardSvg(boardWith(100), {});
    expect(svg.match(/Note \d+/g)).toHaveLength(100);
  });

  it("exports a large board with 2,000 elements", () => {
    const svg = boardSvg(boardWith(2000), {});
    expect(svg).toContain("Note 1999");
    expect(svg.length).toBeGreaterThan(100_000);
  });
});
