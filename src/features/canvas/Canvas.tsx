import { PointerEvent, WheelEvent, useEffect, useRef, useState, type ReactNode } from "react";
import { BringToFront, Copy, Frame, GitBranch, MousePointer2, Pencil, Redo2, Square, StickyNote, Trash2, Type, Undo2 } from "lucide-react";
import { nanoid } from "nanoid";
import type { Board, CanvasElement, CanvasElementKind } from "../../lib/types";
import styles from "./Canvas.module.css";

type Viewport = { x: number; y: number; zoom: number };
type Tool = "select" | "connector" | "draw";
type Interaction =
  | { kind: "pan"; pointerId: number; start: Point; viewport: Viewport }
  | { kind: "marquee"; pointerId: number; start: Point }
  | { kind: "drag"; pointerId: number; start: Point; before: Board; elements: Record<string, CanvasElement>; changed: boolean }
  | { kind: "resize"; pointerId: number; start: Point; before: Board; element: CanvasElement; changed: boolean }
  | { kind: "rotate"; pointerId: number; center: Point; startAngle: number; before: Board; element: CanvasElement; changed: boolean }
  | { kind: "connector"; pointerId: number; start: Point; before: Board; element: CanvasElement; changed: boolean }
  | { kind: "draw"; pointerId: number; before: Board; element: CanvasElement; points: Point[]; changed: boolean };
type Point = { x: number; y: number };

const noteColors = ["#f7dd72", "#f8b7a5", "#b8dfc4", "#add8e6", "#d8c5ef"];
const shapeColors = ["#dff2e9", "#e9e2f5", "#dfeef6"];

type CanvasProps = {
  board: Board;
  onChange: (board: Board) => void;
  onCommit: (before: Board, board: Board) => void;
  onUndo: () => void;
  onRedo: () => void;
  canUndo: boolean;
  canRedo: boolean;
  assets: Record<string, string>;
};

export function Canvas({ board, onChange, onCommit, onUndo, onRedo, canUndo, canRedo, assets }: CanvasProps) {
  const [viewport, setViewport] = useState<Viewport>({ x: 0, y: 0, zoom: 1 });
  const [selectedIds, setSelectedIds] = useState<string[]>([]);
  const [tool, setTool] = useState<Tool>("select");
  const [marquee, setMarquee] = useState<{ start: Point; end: Point } | null>(null);
  const svgRef = useRef<SVGSVGElement>(null);
  const interaction = useRef<Interaction | null>(null);
  const latestInteractionBoard = useRef<Board | null>(null);
  const boardRef = useRef(board);
  const spacePressed = useRef(false);
  const clipboard = useRef<CanvasElement[]>([]);
  boardRef.current = board;

  useEffect(() => {
    function handleKeyDown(event: KeyboardEvent) {
      if (event.code === "Space" && !isTextInput(event.target)) spacePressed.current = true;
      if (isTextInput(event.target)) return;
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "z") {
        event.preventDefault();
        if (event.shiftKey) onRedo(); else onUndo();
      }
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "d") {
        event.preventDefault();
        duplicateSelected();
      }
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "c") {
        event.preventDefault();
        copySelected();
      }
      if ((event.ctrlKey || event.metaKey) && event.key.toLowerCase() === "v") {
        event.preventDefault();
        pasteClipboard();
      }
      if (event.key === "Delete" || event.key === "Backspace") deleteSelected();
      if (event.key === "v") setTool("select");
      if (event.key === "n") addElement("sticky-note");
      if (event.key === "t") addElement("text");
      if (event.key === "r") addElement("shape");
    }
    function handleKeyUp(event: KeyboardEvent) { if (event.code === "Space") spacePressed.current = false; }
    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("keyup", handleKeyUp);
    return () => { window.removeEventListener("keydown", handleKeyDown); window.removeEventListener("keyup", handleKeyUp); };
  });

  function screenPoint(event: PointerEvent<Element>): Point {
    const rect = svgRef.current!.getBoundingClientRect();
    return { x: event.clientX - rect.left, y: event.clientY - rect.top };
  }

  function documentPoint(event: PointerEvent<Element>): Point {
    const current = screenPoint(event);
    return { x: (current.x - viewport.x) / viewport.zoom, y: (current.y - viewport.y) / viewport.zoom };
  }

  function replaceElements(update: (elements: Record<string, CanvasElement>) => Record<string, CanvasElement>) {
    return { ...boardRef.current, elements: update(boardRef.current.elements) };
  }

  function addElement(kind: Extract<CanvasElementKind, "sticky-note" | "text" | "shape" | "frame">) {
    const current = boardRef.current;
    const id = nanoid();
    const dimensions = kind === "sticky-note" ? { width: 210, height: 180 } : kind === "text" ? { width: 260, height: 80 } : kind === "frame" ? { width: 540, height: 340 } : { width: 220, height: 140 };
    const element: CanvasElement = {
      id,
      kind,
      x: (400 - viewport.x) / viewport.zoom,
      y: (280 - viewport.y) / viewport.zoom,
      ...dimensions,
      rotation: 0,
      content: kind === "text" ? "Digite aqui" : "",
      color: kind === "sticky-note" ? noteColors[current.elementOrder.length % noteColors.length] : kind === "frame" ? "#8c9c93" : shapeColors[current.elementOrder.length % shapeColors.length],
    };
    const next = { ...current, elements: { ...current.elements, [id]: element }, elementOrder: [...current.elementOrder, id] };
    onCommit(current, next);
    setSelectedIds([id]);
  }

  function updateElement(id: string, update: Partial<CanvasElement>, commit = false) {
    const before = boardRef.current;
    const next = replaceElements((elements) => ({ ...elements, [id]: { ...elements[id], ...update } }));
    if (commit) onCommit(before, next); else onChange(next);
  }

  function deleteSelected() {
    const current = boardRef.current;
    if (!selectedIds.length) return;
    const selected = new Set(selectedIds);
    const elements = Object.fromEntries(Object.entries(current.elements).filter(([id]) => !selected.has(id)));
    const next = { ...current, elements, elementOrder: current.elementOrder.filter((id) => !selected.has(id)) };
    onCommit(current, next);
    setSelectedIds([]);
  }

  function duplicateSelected() {
    const current = boardRef.current;
    if (!selectedIds.length) return;
    const copies = selectedIds.map((id) => ({ ...current.elements[id], id: nanoid(), x: current.elements[id].x + 24, y: current.elements[id].y + 24 }));
    const next = { ...current, elements: { ...current.elements, ...Object.fromEntries(copies.map((element) => [element.id, element])) }, elementOrder: [...current.elementOrder, ...copies.map((element) => element.id)] };
    onCommit(current, next);
    setSelectedIds(copies.map((element) => element.id));
  }

  function copySelected() {
    clipboard.current = selectedIds.map((id) => boardRef.current.elements[id]).filter(Boolean).map((element) => ({ ...element }));
  }

  function pasteClipboard() {
    const current = boardRef.current;
    if (!clipboard.current.length) return;
    const copies = clipboard.current.map((element) => ({ ...element, id: nanoid(), x: element.x + 32, y: element.y + 32, groupId: undefined }));
    const next = { ...current, elements: { ...current.elements, ...Object.fromEntries(copies.map((element) => [element.id, element])) }, elementOrder: [...current.elementOrder, ...copies.map((element) => element.id)] };
    onCommit(current, next);
    setSelectedIds(copies.map((element) => element.id));
  }

  function bringToFront() {
    const current = boardRef.current;
    if (!selectedIds.length) return;
    const selected = new Set(selectedIds);
    const next = { ...current, elementOrder: [...current.elementOrder.filter((id) => !selected.has(id)), ...selectedIds] };
    onCommit(current, next);
  }

  function groupSelected() {
    const current = boardRef.current;
    if (selectedIds.length < 2) return;
    const groupId = nanoid();
    const next = replaceElements((elements) => Object.fromEntries(Object.entries(elements).map(([id, element]) => [id, selectedIds.includes(id) ? { ...element, groupId } : element])));
    onCommit(current, next);
  }

  function ungroupSelected() {
    const current = boardRef.current;
    const groups = new Set(selectedIds.map((id) => current.elements[id]?.groupId).filter(Boolean));
    if (!groups.size) return;
    const next = replaceElements((elements) => Object.fromEntries(Object.entries(elements).map(([id, element]) => [id, element.groupId && groups.has(element.groupId) ? { ...element, groupId: undefined } : element])));
    onCommit(current, next);
  }

  function handlePointerDown(event: PointerEvent<SVGSVGElement>) {
    if (event.button !== 0 && event.button !== 1) return;
    const start = documentPoint(event);
    if (tool === "connector" && event.button === 0) {
      const before = boardRef.current;
      const element: CanvasElement = { id: nanoid(), kind: "connector", x: start.x, y: start.y, width: 1, height: 1, rotation: 0, content: "", color: "#52645b" };
      onChange({ ...before, elements: { ...before.elements, [element.id]: element }, elementOrder: [...before.elementOrder, element.id] });
      interaction.current = { kind: "connector", pointerId: event.pointerId, start, before, element, changed: false };
      latestInteractionBoard.current = null;
      event.currentTarget.setPointerCapture(event.pointerId);
      return;
    }
    if (tool === "draw" && event.button === 0) {
      const before = boardRef.current;
      const element: CanvasElement = { id: nanoid(), kind: "freehand", x: start.x, y: start.y, width: 0, height: 0, rotation: 0, content: JSON.stringify([start]), color: "#315f50" };
      const next = { ...before, elements: { ...before.elements, [element.id]: element }, elementOrder: [...before.elementOrder, element.id] };
      onChange(next);
      interaction.current = { kind: "draw", pointerId: event.pointerId, before, element, points: [start], changed: false };
      latestInteractionBoard.current = null;
      event.currentTarget.setPointerCapture(event.pointerId);
      return;
    }
    if (event.button === 1 || spacePressed.current) {
      interaction.current = { kind: "pan", pointerId: event.pointerId, start: screenPoint(event), viewport };
      event.currentTarget.setPointerCapture(event.pointerId);
      return;
    }
    interaction.current = { kind: "marquee", pointerId: event.pointerId, start };
    setMarquee({ start, end: start });
    event.currentTarget.setPointerCapture(event.pointerId);
  }

  function handleElementPointerDown(event: PointerEvent<SVGGElement>, element: CanvasElement) {
    if (event.button !== 0 || tool !== "select") return;
    event.stopPropagation();
    if (event.shiftKey) {
      setSelectedIds((current) => current.includes(element.id) ? current.filter((id) => id !== element.id) : [...current, element.id]);
      return;
    }
    const initialIds = selectedIds.includes(element.id) ? selectedIds : [element.id];
    const groupIds = new Set(initialIds.map((id) => boardRef.current.elements[id]?.groupId).filter(Boolean));
    const ids = groupIds.size ? Object.values(boardRef.current.elements).filter((item) => item.groupId && groupIds.has(item.groupId)).map((item) => item.id) : initialIds;
    if (!selectedIds.includes(element.id)) setSelectedIds(ids);
    const elements = Object.fromEntries(ids.map((id) => [id, boardRef.current.elements[id]]));
    interaction.current = { kind: "drag", pointerId: event.pointerId, start: documentPoint(event), before: boardRef.current, elements, changed: false };
    latestInteractionBoard.current = null;
    svgRef.current?.setPointerCapture(event.pointerId);
  }

  function startResize(event: PointerEvent<SVGRectElement>, element: CanvasElement) {
    event.stopPropagation();
    interaction.current = { kind: "resize", pointerId: event.pointerId, start: documentPoint(event), before: boardRef.current, element, changed: false };
    latestInteractionBoard.current = null;
    svgRef.current?.setPointerCapture(event.pointerId);
  }

  function startRotate(event: PointerEvent<SVGCircleElement>, element: CanvasElement) {
    event.stopPropagation();
    const center = { x: element.x + element.width / 2, y: element.y + element.height / 2 };
    const current = documentPoint(event);
    interaction.current = { kind: "rotate", pointerId: event.pointerId, center, startAngle: angle(center, current), before: boardRef.current, element, changed: false };
    latestInteractionBoard.current = null;
    svgRef.current?.setPointerCapture(event.pointerId);
  }

  function handlePointerMove(event: PointerEvent<SVGSVGElement>) {
    const active = interaction.current;
    if (!active || active.pointerId !== event.pointerId) return;
    if (active.kind === "pan") {
      const current = screenPoint(event);
      setViewport({ ...active.viewport, x: active.viewport.x + current.x - active.start.x, y: active.viewport.y + current.y - active.start.y });
      return;
    }
    const current = documentPoint(event);
    if (active.kind === "marquee") {
      setMarquee({ start: active.start, end: current });
      setSelectedIds(Object.values(boardRef.current.elements).filter((element) => intersects(active.start, current, element)).map((element) => element.id));
      return;
    }
    if (active.kind === "drag") {
      const dx = current.x - active.start.x;
      const dy = current.y - active.start.y;
      const next = { ...active.before, elements: { ...active.before.elements, ...Object.fromEntries(Object.values(active.elements).map((element) => [element.id, { ...element, x: element.x + dx, y: element.y + dy }])) } };
      active.changed = true;
      latestInteractionBoard.current = next;
      onChange(next);
      return;
    }
    if (active.kind === "resize") {
      active.changed = true;
      const next = { ...active.before, elements: { ...active.before.elements, [active.element.id]: { ...active.element, width: Math.max(80, active.element.width + current.x - active.start.x), height: Math.max(50, active.element.height + current.y - active.start.y) } } };
      latestInteractionBoard.current = next;
      onChange(next);
      return;
    }
    if (active.kind === "rotate") {
      active.changed = true;
      const rotation = active.element.rotation + angle(active.center, current) - active.startAngle;
      const next = { ...active.before, elements: { ...active.before.elements, [active.element.id]: { ...active.element, rotation } } };
      latestInteractionBoard.current = next;
      onChange(next);
      return;
    }
    if (active.kind === "connector") {
      active.changed = true;
      const next = { ...active.before, elements: { ...active.before.elements, [active.element.id]: { ...active.element, width: current.x - active.start.x, height: current.y - active.start.y } } };
      latestInteractionBoard.current = next;
      onChange(next);
    }
    if (active.kind === "draw") {
      const points = [...active.points, current];
      active.points = points;
      active.changed = true;
      const next = { ...active.before, elements: { ...active.before.elements, [active.element.id]: { ...active.element, content: JSON.stringify(points) } } };
      latestInteractionBoard.current = next;
      onChange(next);
    }
  }

  function finishInteraction(event: PointerEvent<SVGSVGElement>) {
    const active = interaction.current;
    if (!active || active.pointerId !== event.pointerId) return;
    interaction.current = null;
    event.currentTarget.releasePointerCapture(event.pointerId);
    if (active.kind === "marquee") { setMarquee(null); return; }
    if (active.kind === "pan") return;
    if (active.changed) onCommit(active.before, latestInteractionBoard.current ?? boardRef.current);
    latestInteractionBoard.current = null;
    if (active.kind === "connector" || active.kind === "draw") { setSelectedIds([active.element.id]); setTool("select"); }
  }

  function handleWheel(event: WheelEvent<SVGSVGElement>) {
    event.preventDefault();
    const rect = event.currentTarget.getBoundingClientRect();
    const cursor = { x: event.clientX - rect.left, y: event.clientY - rect.top };
    const nextZoom = Math.min(3, Math.max(0.25, viewport.zoom * (event.deltaY < 0 ? 1.1 : 0.9)));
    const ratio = nextZoom / viewport.zoom;
    setViewport({ x: cursor.x - (cursor.x - viewport.x) * ratio, y: cursor.y - (cursor.y - viewport.y) * ratio, zoom: nextZoom });
  }

  const single = selectedIds.length === 1 ? board.elements[selectedIds[0]] : null;
  return (
    <section className={styles.canvasShell}>
      <div className={styles.toolbar}>
        <ToolButton active={tool === "select"} label="Selecionar (V)" onClick={() => setTool("select")}><MousePointer2 size={17} /></ToolButton>
        <ToolButton label="Sticky note (N)" onClick={() => addElement("sticky-note")}><StickyNote size={17} /></ToolButton>
        <ToolButton label="Texto (T)" onClick={() => addElement("text")}><Type size={17} /></ToolButton>
        <ToolButton label="Forma (R)" onClick={() => addElement("shape")}><Square size={17} /></ToolButton>
        <ToolButton label="Frame" onClick={() => addElement("frame")}><Frame size={17} /></ToolButton>
        <ToolButton active={tool === "connector"} label="Conector" onClick={() => setTool("connector")}><GitBranch size={17} /></ToolButton>
        <ToolButton active={tool === "draw"} label="Desenho livre" onClick={() => setTool("draw")}><Pencil size={17} /></ToolButton>
        <span />
        <ToolButton label="Desfazer" disabled={!canUndo} onClick={onUndo}><Undo2 size={17} /></ToolButton>
        <ToolButton label="Refazer" disabled={!canRedo} onClick={onRedo}><Redo2 size={17} /></ToolButton>
        <ToolButton label="Duplicar" disabled={!selectedIds.length} onClick={duplicateSelected}><Copy size={17} /></ToolButton>
        <ToolButton label="Trazer para frente" disabled={!selectedIds.length} onClick={bringToFront}><BringToFront size={17} /></ToolButton>
        <ToolButton label="Agrupar" disabled={selectedIds.length < 2} onClick={groupSelected}>G</ToolButton>
        <ToolButton label="Desagrupar" disabled={!selectedIds.some((id) => board.elements[id]?.groupId)} onClick={ungroupSelected}>U</ToolButton>
        <ToolButton label="Excluir" disabled={!selectedIds.length} onClick={deleteSelected}><Trash2 size={17} /></ToolButton>
      </div>
      <svg ref={svgRef} className={styles.canvas} onPointerDown={handlePointerDown} onPointerMove={handlePointerMove} onPointerUp={finishInteraction} onPointerCancel={finishInteraction} onWheel={handleWheel}>
        <defs>
          <pattern id="dot-grid" width="24" height="24" patternUnits="userSpaceOnUse"><circle cx="1" cy="1" r="1" fill="#d8ddd7" /></pattern>
          <marker id="arrow" markerWidth="9" markerHeight="9" refX="7" refY="4.5" orient="auto"><path d="M0,0 L8,4.5 L0,9 Z" fill="#52645b" /></marker>
        </defs>
        <rect width="100%" height="100%" fill="url(#dot-grid)" />
        <g transform={`translate(${viewport.x} ${viewport.y}) scale(${viewport.zoom})`}>
          {board.elementOrder.map((id) => <CanvasElementView key={id} element={board.elements[id]} selected={selectedIds.includes(id)} onPointerDown={handleElementPointerDown} onTextChange={updateElement} assets={assets} />)}
          {single && single.kind !== "connector" && single.kind !== "freehand" && <SelectionHandles element={single} onResize={startResize} onRotate={startRotate} />}
          {marquee && <rect className={styles.marquee} x={Math.min(marquee.start.x, marquee.end.x)} y={Math.min(marquee.start.y, marquee.end.y)} width={Math.abs(marquee.end.x - marquee.start.x)} height={Math.abs(marquee.end.y - marquee.start.y)} />}
        </g>
      </svg>
      <div className={styles.zoom}>{Math.round(viewport.zoom * 100)}%</div>
    </section>
  );
}

function ToolButton({ active, label, onClick, disabled, children }: { active?: boolean; label: string; onClick: () => void; disabled?: boolean; children: ReactNode }) {
  return <button className={active ? styles.toolActive : ""} type="button" title={label} aria-label={label} onClick={onClick} disabled={disabled}>{children}</button>;
}

function CanvasElementView({ element, selected, onPointerDown, onTextChange, assets }: { element: CanvasElement; selected: boolean; onPointerDown: (event: PointerEvent<SVGGElement>, element: CanvasElement) => void; onTextChange: (id: string, update: Partial<CanvasElement>) => void; assets: Record<string, string> }) {
  if (!element) return null;
  if (element.kind === "connector") return <g onPointerDown={(event) => onPointerDown(event, element)}><line className={selected ? styles.connectorSelected : styles.connector} x1={element.x} y1={element.y} x2={element.x + element.width} y2={element.y + element.height} markerEnd="url(#arrow)" /></g>;
  if (element.kind === "freehand") return <g onPointerDown={(event) => onPointerDown(event, element)}><polyline className={selected ? styles.freehandSelected : styles.freehand} points={freehandPoints(element.content)} /></g>;
  return (
    <g onPointerDown={(event) => onPointerDown(event, element)} transform={`translate(${element.x} ${element.y}) rotate(${element.rotation} ${element.width / 2} ${element.height / 2})`}>
      {selected && <rect className={styles.selection} x="-5" y="-5" width={element.width + 10} height={element.height + 10} rx="8" />}
      {element.kind === "shape" && <rect className={styles.shape} width={element.width} height={element.height} rx="10" fill={element.color} />}
      {element.kind === "frame" && <><rect className={styles.frame} width={element.width} height={element.height} rx="12" /><text className={styles.frameLabel} x="14" y="27">Frame</text></>}
      {element.kind === "image" && <image className={styles.image} href={assets[element.content]} width={element.width} height={element.height} preserveAspectRatio="xMidYMid meet" />}
      {element.kind === "sticky-note" && <><rect className={styles.noteShadow} x="3" y="5" width={element.width} height={element.height} rx="4" /><foreignObject x="0" y="0" width={element.width} height={element.height}><textarea className={styles.note} value={element.content} aria-label="Conteúdo do sticky note" style={{ backgroundColor: element.color }} onChange={(event) => onTextChange(element.id, { content: event.target.value })} /></foreignObject></>}
      {element.kind === "text" && <foreignObject x="0" y="0" width={element.width} height={element.height}><textarea className={styles.text} value={element.content} aria-label="Texto" onChange={(event) => onTextChange(element.id, { content: event.target.value })} /></foreignObject>}
    </g>
  );
}

function SelectionHandles({ element, onResize, onRotate }: { element: CanvasElement; onResize: (event: PointerEvent<SVGRectElement>, element: CanvasElement) => void; onRotate: (event: PointerEvent<SVGCircleElement>, element: CanvasElement) => void }) {
  return <g transform={`translate(${element.x} ${element.y}) rotate(${element.rotation} ${element.width / 2} ${element.height / 2})`}><line className={styles.rotationLine} x1={element.width / 2} y1="-4" x2={element.width / 2} y2="-25" /><circle className={styles.rotationHandle} cx={element.width / 2} cy="-28" r="5" onPointerDown={(event) => onRotate(event, element)} /><rect className={styles.resizeHandle} x={element.width - 5} y={element.height - 5} width="10" height="10" onPointerDown={(event) => onResize(event, element)} /></g>;
}

function angle(center: Point, point: Point) { return Math.atan2(point.y - center.y, point.x - center.x) * 180 / Math.PI; }
function intersects(start: Point, end: Point, element: CanvasElement) { const left = Math.min(start.x, end.x); const right = Math.max(start.x, end.x); const top = Math.min(start.y, end.y); const bottom = Math.max(start.y, end.y); return element.x < right && element.x + Math.abs(element.width) > left && element.y < bottom && element.y + Math.abs(element.height) > top; }
function isTextInput(target: EventTarget | null) { return target instanceof HTMLTextAreaElement || target instanceof HTMLInputElement || (target instanceof HTMLElement && target.isContentEditable); }
function freehandPoints(content: string) { try { return (JSON.parse(content) as Point[]).map((point) => `${point.x},${point.y}`).join(" "); } catch { return ""; } }
