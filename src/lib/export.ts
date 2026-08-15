import type { Board, CanvasElement } from "./types";

const escape = (value: string) => value.replace(/[&<>"']/g, (character) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;" })[character]!);

export function boardSvg(board: Board, assets: Record<string, string>) {
  const elements = board.elementOrder.map((id) => board.elements[id]).filter(Boolean);
  const bounds = canvasBounds(elements);
  const content = elements.map((element) => svgElement(element, assets)).join("");
  return `<svg xmlns="http://www.w3.org/2000/svg" width="${bounds.width}" height="${bounds.height}" viewBox="${bounds.x} ${bounds.y} ${bounds.width} ${bounds.height}">${content}</svg>`;
}

export function downloadSvg(board: Board, assets: Record<string, string>) {
  download(new Blob([boardSvg(board, assets)], { type: "image/svg+xml" }), `${safeName(board.name)}.svg`);
}

export async function downloadPng(board: Board, assets: Record<string, string>) {
  const svg = boardSvg(board, assets);
  const image = new Image();
  const url = URL.createObjectURL(new Blob([svg], { type: "image/svg+xml" }));
  await new Promise<void>((resolve, reject) => { image.onload = () => resolve(); image.onerror = () => reject(new Error("Não foi possível renderizar o board.")); image.src = url; });
  const canvas = document.createElement("canvas");
  canvas.width = image.width || 1600;
  canvas.height = image.height || 1000;
  canvas.getContext("2d")!.drawImage(image, 0, 0);
  URL.revokeObjectURL(url);
  const blob = await new Promise<Blob | null>((resolve) => canvas.toBlob(resolve, "image/png"));
  if (!blob) throw new Error("Não foi possível gerar o PNG.");
  download(blob, `${safeName(board.name)}.png`);
}

export function download(bytes: BlobPart, fileName: string, type = "application/octet-stream") {
  const url = URL.createObjectURL(bytes instanceof Blob ? bytes : new Blob([bytes], { type }));
  const anchor = document.createElement("a");
  anchor.href = url;
  anchor.download = fileName;
  anchor.click();
  URL.revokeObjectURL(url);
}

function svgElement(element: CanvasElement, assets: Record<string, string>) {
  const transform = `translate(${element.x} ${element.y}) rotate(${element.rotation} ${element.width / 2} ${element.height / 2})`;
  if (element.kind === "connector") return `<line x1="${element.x}" y1="${element.y}" x2="${element.x + element.width}" y2="${element.y + element.height}" stroke="#52645b" stroke-width="2"/>`;
  if (element.kind === "freehand") return `<polyline points="${escape(points(element.content))}" fill="none" stroke="#315f50" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"/>`;
  if (element.kind === "image" && assets[element.content]) return `<image href="${assets[element.content]}" x="${element.x}" y="${element.y}" width="${element.width}" height="${element.height}"/>`;
  if (element.kind === "shape") return `<rect transform="${transform}" width="${element.width}" height="${element.height}" rx="10" fill="${element.color}" stroke="#477361"/>`;
  if (element.kind === "frame") return `<rect transform="${transform}" width="${element.width}" height="${element.height}" rx="12" fill="none" stroke="#8c9c93" stroke-width="2" stroke-dasharray="7 5"/>`;
  const fill = element.kind === "sticky-note" ? `<rect width="${element.width}" height="${element.height}" rx="4" fill="${element.color}"/>` : "";
  const text = escape(element.content).split("\n").map((line, index) => `<tspan x="${element.kind === "sticky-note" ? 17 : 4}" dy="${index ? "1.3em" : "0"}">${line || " "}</tspan>`).join("");
  return `<g transform="${transform}">${fill}<text x="${element.kind === "sticky-note" ? 17 : 4}" y="${element.kind === "sticky-note" ? 38 : 28}" font-family="Arial, sans-serif" font-size="${element.kind === "text" ? 22 : 16}" fill="#34352f">${text}</text></g>`;
}

function canvasBounds(elements: CanvasElement[]) {
  if (!elements.length) return { x: 0, y: 0, width: 1600, height: 1000 };
  const left = Math.min(...elements.map((element) => element.x)) - 40;
  const top = Math.min(...elements.map((element) => element.y)) - 40;
  const right = Math.max(...elements.map((element) => element.x + Math.abs(element.width))) + 40;
  const bottom = Math.max(...elements.map((element) => element.y + Math.abs(element.height))) + 40;
  return { x: left, y: top, width: Math.max(1, right - left), height: Math.max(1, bottom - top) };
}

function points(content: string) { try { return (JSON.parse(content) as { x: number; y: number }[]).map((point) => `${point.x},${point.y}`).join(" "); } catch { return ""; } }
function safeName(name: string) { return name.replace(/[\\/:*?"<>|]/g, "-").trim() || "logline-board"; }
